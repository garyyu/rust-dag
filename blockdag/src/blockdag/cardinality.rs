
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
use std::sync::{Arc,RwLock};

use blockdag::{Block,MaxMin,append_maps};

/// Function providing cardinality of pastset blocks calculation.
///
pub fn sizeof_pastset(block: &Block) -> (u64,u64){

    let mut size_of_past: u64 = 0;
    let mut size_of_past_blue: u64 = 0;

    if block.prev.len()==0 {
        return (size_of_past,size_of_past_blue);
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
            return (1,1);
        }

        if prev.is_blue {
            size_of_past_blue += 1;
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
    size_of_past_blue += bmax_block.read().unwrap().size_of_past_blue;
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

        //println!("sizeof_pastset(): block={} rest_height_min={} rest={:?} maxi_height_max={} max={:?} size_of_past={}", block.name, rest_maxmin.min,
        //         sorted_keys_by_height(&new_rest_pred, false).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>(),
        //         maxi_height_max, sorted_keys_by_height(&maxi_pred_set, false).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>(),
        //         size_of_past);
        let rest_keys = new_rest_pred.iter().map(|(k,_)|{k.clone()}).collect::<Vec<String>>();
        for name in &rest_keys {
            if maxi_pred_set.get(name).is_some() {
                new_rest_pred.remove(name);
            }
        }

        size_of_past += new_rest_pred.len() as u64;
        for (_,value) in &new_rest_pred {
            let rest = &value.read().unwrap();
            if rest.is_blue {
                size_of_past_blue += 1;
            }
        }

        drop(rest_pred_set);
        rest_pred_set = new_rest_pred;
        //println!("sizeof_pastset(): block={} size_of_past={} rest_pred_set={}", block.name, size_of_past, rest_pred_set.len());
    }
    //println!("sizeof_pastset(): block={} final result: size_of_past={}", block.name, size_of_past);

    return (size_of_past,size_of_past_blue);
}

pub fn step_one_past(pred: &HashMap<String,Arc<RwLock<Block>>>, new_pred: &mut HashMap<String,Arc<RwLock<Block>>>, used: &mut HashMap<String,bool>, maxmin: &mut MaxMin) -> MaxMin{

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

