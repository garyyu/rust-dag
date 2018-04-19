
// Copyright 2018 The rust-dag Authors
// This file is part of the rust-dag library.
//
// The rust-dag library is free software: you can redistribute it and/or modify
// it under the terms of the GNU Lesser General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// The rust-dag library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License
// along with the rust-dag library. If not, see <http://www.gnu.org/licenses/>.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use blockdag::{Block,MaxMin};
use std::sync::{Arc,RwLock};

/// Function providing cardinality of pastset blocks calculation.
///
pub fn sizeof_pastset(block: &Block, dag: &HashMap<String, Arc<RwLock<Block>>>) -> u64{

    let mut size_of_past: u64 = 0;

    if block.prev.len()==0 {
        return size_of_past;
    }

    let mut maxi_pred_set: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
    let mut rest_pred_set: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();

    // find the max sizeofpast among block's predecessors
    let mut max_sizeofpast: u64 = 0;
    let mut bmax_name = String::new();

    for (_key, value) in &block.prev {

        let prev = Arc::clone(value);
        let prev = prev.read().unwrap();

        if max_sizeofpast < prev.size_of_past_set {
            max_sizeofpast = prev.size_of_past_set;
            bmax_name = String::from(prev.name.clone());
        }

        if prev.name == "Genesis" {
            return 1;
        }

        rest_pred_set.insert(String::from(prev.name.clone()), Arc::clone(value));
    }

    if bmax_name.len()==0 {
        panic!("sizeof_pastset(): impossible! bmax=nil.");
    }

    let bmax_block = block.prev.get(&bmax_name).unwrap();
    maxi_pred_set.insert(bmax_name.clone(), Arc::clone(bmax_block));

    rest_pred_set.remove(&bmax_name);

    size_of_past = max_sizeofpast + block.prev.len() as u64;
    //println!("sizeof_pastset(): block={} bmax={} size_of_past={}", block.name, bmax_name, size_of_past);

    let mut used_rest: HashMap<String,bool> = HashMap::new();
    let mut used_maxi: HashMap<String,bool> = HashMap::new();

    let mut rest_maxmin = MaxMin{max:0, min:<u64>::max_value()};
    let mut maxi_maxmin = MaxMin{max:0, min:<u64>::max_value()};

    while rest_pred_set.len() > 0 {

        let mut new_rest_pred: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
        let _rest_local_maxmin = step_one_past(&rest_pred_set, &mut new_rest_pred, &mut used_rest, &mut rest_maxmin);

        //let mut maxi_height_max = 0;
        loop {
            let mut new_maxi_pred: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
            let max_local_maxmin = step_one_past(&maxi_pred_set, &mut new_maxi_pred, &mut used_maxi, &mut maxi_maxmin);

            append_maps(&mut maxi_pred_set, &new_maxi_pred);
            drop(new_maxi_pred);

            if max_local_maxmin.max <= rest_maxmin.min {
                //maxi_height_max = max_local_maxmin.max;
                break;
            }
        }

        let sorted_keys = sorted_keys_by_height(&new_rest_pred);
        //println!("sizeof_pastset(): block={} maxi_height_max={} rest_height_min={} sorted_keys={:?} maxi_pred_set={:?}", block.name, maxi_height_max, rest_maxmin.min,
        //         sorted_keys, sorted_keys_by_height(&maxi_pred_set));
        for (name,_) in sorted_keys {
            let found_block = maxi_pred_set.get(&name);
            if found_block.is_some() {
                let size_of_rest = new_rest_pred.len();

                let rest = Arc::clone(found_block.unwrap());
                let rest = rest.read().unwrap();
                //println!("sizeof_pastset(): block={} common block found: {}", block.name, rest);
                remove_successors(&rest, &mut new_rest_pred);

                size_of_past -= (size_of_rest - new_rest_pred.len()) as u64;
                new_rest_pred.remove(&name);
                //println!("sizeof_pastset(): block={} size_of_past={}", block.name, size_of_past);
            }
        }

        size_of_past += new_rest_pred.len() as u64;

        rest_pred_set = new_rest_pred;
        //println!("sizeof_pastset(): block={} size_of_past={} rest_pred_set={}", block.name, size_of_past, rest_pred_set.len());
    }
    //println!("sizeof_pastset(): block={} final result: size_of_past={}", block.name, size_of_past);

    return size_of_past;
}

/// Remove from the list all the block successors which is in the list, self not included.
///
fn remove_successors(block: &Block, list: &mut HashMap<String, Arc<RwLock<Block>>>){

    for (_key, value) in &block.next {

        let next = Arc::clone(value);
        let next = next.read().unwrap();

        remove_successors(&next, list);

        list.remove(&String::from(next.name.clone()));
    }
}

fn sorted_keys_by_height(source: &HashMap<String,Arc<RwLock<Block>>>) -> Vec<(String, u64)>{

    let mut keys_vec: Vec<(String, u64)> = Vec::new();

    for (_key, value) in source {
        let block = Arc::clone(value);
        let block = block.read().unwrap();

        keys_vec.push((String::from(block.name.clone()), block.height));
    }

    keys_vec.sort_by(|a, b| a.1.cmp(&b.1).reverse());
    return keys_vec;
}

fn append_maps(target: &mut HashMap<String,Arc<RwLock<Block>>>, source: &HashMap<String,Arc<RwLock<Block>>>){

    for (key, value) in source {

        if let Entry::Vacant(v) = target.entry(key.clone()){
            v.insert(Arc::clone(value));
        }
    }
}

fn step_one_past(pred: &HashMap<String,Arc<RwLock<Block>>>, new_pred: &mut HashMap<String,Arc<RwLock<Block>>>, used: &mut HashMap<String,bool>, maxmin: &mut MaxMin) -> MaxMin{

    let mut local_maxmin = MaxMin{max:0, min:<u64>::max_value()};

    for (key, value) in pred {
        if let Entry::Vacant(v) = used.entry(key.clone()){

            let rest = Arc::clone(value);
            let rest = rest.read().unwrap();

            for (key2, value2) in &rest.prev {

                if let Entry::Vacant(v) = new_pred.entry(key2.clone()) {
                    let prev = Arc::clone(value2);
                    let prev = prev.read().unwrap();

                    if prev.height > local_maxmin.max {
                        local_maxmin.max = prev.height;
                    }

                    if prev.height < local_maxmin.min {
                        local_maxmin.min = prev.height;
                    }

                    v.insert(Arc::clone(value2));
                }
            }

            v.insert(true);
        }
    }

    if local_maxmin.max > maxmin.max {
        maxmin.max = local_maxmin.max;
    }

    if local_maxmin.min < maxmin.min {
        maxmin.min = local_maxmin.min;
    }

    return local_maxmin;
}

