
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

use blockdag::{Block,MaxMin,Node};
use blockdag::{sorted_keys_by_height,step_one_past,append_maps};

/// Function providing anti-cone calculations.
///
pub fn tips_anticone(tip_name: &str, tips: &HashMap<String, Arc<RwLock<Block>>>) -> HashMap<String, Arc<RwLock<Block>>>{

    let mut anticone: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();

    if tips.len()==0 {
        //println!("tips_anticone(): tip={} error! tips is empty", tip_name);
        return anticone;
    }

    let mut maxi_pred_set: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
    let mut rest_pred_set: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();

    for (key, value) in tips {

//        let tip = Arc::clone(value);
//        let tip = tip.read().unwrap();

        if key == tip_name {
            maxi_pred_set.insert(String::from(key.clone()), Arc::clone(value));
        }else {
            rest_pred_set.insert(String::from(key.clone()), Arc::clone(value));
            anticone.insert(String::from(key.clone()), Arc::clone(value));
        }
    }

    if maxi_pred_set.len()==0 {
        println!("tips_anticone(): error! tip {} is not in tips", tip_name);
        return HashMap::new();
    }

    //println!("tips_anticone(): tip={} size_of_anticone={}", tip_name, anticone.len());

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

        //println!("tips_anticone(): tip={} rest_height_min={} rest={:?} maxi_height_max={} max={:?} size_of_anticone={}", tip_name, rest_maxmin.min,
        //         sorted_keys_by_height(&new_rest_pred, true).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>(),
        //         maxi_height_max, sorted_keys_by_height(&maxi_pred_set, true).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>(),
        //         anticone.len());
        let rest_keys = new_rest_pred.iter().map(|(k,_)|{k.clone()}).collect::<Vec<String>>();
        for name in &rest_keys {
            if maxi_pred_set.get(name).is_some() {
                new_rest_pred.remove(name);
            }
        }

        append_maps(&mut anticone, &new_rest_pred);

        rest_pred_set = new_rest_pred;
        //println!("tips_anticone(): tip={} size_of_anticone={} rest_pred_set={}", tip_name, anticone.len(), rest_pred_set.len());
    }
    //println!("tips_anticone(): tip={} final result: size_of_anticone={}", tip_name, anticone.len());

    return anticone;
}

/// Function providing anti-cone blue counting, optimized for k: exit if counter > k already. No limitation, any block can be the input block.
///
/// 'any_name' block may have no relationship with tips.
/// 'classmates' collect blocks name whose height is same.
/// 'tips' here is the unique identification of the block DAG G, denotes all those reachable blocks from tips blocks.
///
pub fn anticone_blue(any_name: &str, node: &Node, tips: &HashMap<String, Arc<RwLock<Block>>>, k: i32) -> (i32,HashMap<String, Arc<RwLock<Block>>>) {

    if tips.get(any_name).is_some() {
        return tips_anticone_blue(any_name, tips, k);
    }

    // firstly, we have to create a virtual tips, a nice way is to find which have the same height as the 'any_name' block.
    let mut virtual_tips: HashMap<String, Arc<RwLock<Block>>> = HashMap::new();
    {
        let dag = &node.dag;
        let the_height = dag.get(any_name).unwrap().read().unwrap().height;

        let classmate = &node.classmates.get(&the_height);
        if classmate.is_none() {
            error!("anticone_blue(): classmates don't have such height! input block's height={}", the_height);
            return (-1, HashMap::new());
        }else{
            let classmate = classmate.unwrap();
            for name in classmate {
                virtual_tips.insert(name.clone(), Arc::clone(dag.get(name).unwrap()));
            }
        }
    }
    debug!("anticone_blue(): virtual tips={:?}", sorted_keys_by_height(&virtual_tips, false).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>());

    // left half
    let (anticone_blue_count_left,mut anticone_left) = tips_anticone_blue(any_name, &virtual_tips, k);
    debug!("anticone_blue(): left half anticone_blue_count={}, anticone_blue={:?}", anticone_blue_count_left, sorted_keys_by_height(&anticone_left, true).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>());
    if anticone_blue_count_left > k {
        return (anticone_blue_count_left, anticone_left);
    }

    // right half
    let (anticone_blue_count_right,anticone_right) = tips_anticone_blue_rev(any_name, &virtual_tips, k-anticone_blue_count_left);
    debug!("anticone_blue(): right half anticone_blue_count={}", anticone_blue_count_right);
    append_maps(&mut anticone_left, &anticone_right);

    return (anticone_blue_count_left+anticone_blue_count_right, anticone_left);
}

/// Function providing anti-cone blue counting, optimized for k: exit once counter > k already. Limitation: input block must be one of tips.
///
pub fn tips_anticone_blue(tip_name: &str, tips: &HashMap<String, Arc<RwLock<Block>>>, k: i32) -> (i32,HashMap<String, Arc<RwLock<Block>>>){

    debug!("tips_anticone_blue(): tip={} func enter. tips={:?}", tip_name, sorted_keys_by_height(tips, true).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>());

    let mut anticone_blue_count: i32 = 0;
    let mut anticone: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();

    if tips.len()==0 {
        error!("tips_anticone_blue(): tip={} error! tips is empty", tip_name);
        return (-1,anticone);
    }

    let mut maxi_pred_set: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
    let mut rest_pred_set: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();

    for (key, value) in tips {

        if key == tip_name {
            maxi_pred_set.insert(String::from(key.clone()), Arc::clone(value));
        }else {
            rest_pred_set.insert(String::from(key.clone()), Arc::clone(value));

            let tip = &value.read().unwrap();
            if tip.is_blue {
                anticone.insert(String::from(key.clone()), Arc::clone(value));
                anticone_blue_count += 1;
            }
        }
    }

    if maxi_pred_set.len()==0 {
        error!("tips_anticone_blue(): error! tip {} is not in tips", tip_name);
        return (-1,HashMap::new());
    }

    debug!("tips_anticone_blue(): tip={} size_of_anticone_blue={}", tip_name, anticone.len());

    let mut used_rest: HashMap<String,bool> = HashMap::new();
    let mut used_maxi: HashMap<String,bool> = HashMap::new();

    let mut rest_maxmin = MaxMin{max:0, min:<u64>::max_value()};
    let mut maxi_maxmin = MaxMin{max:0, min:<u64>::max_value()};

    while rest_pred_set.len() > 0 && anticone_blue_count <= k {

        let mut new_rest_pred: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
        let _rest_local_maxmin = step_one_past(&rest_pred_set, &mut new_rest_pred, &mut used_rest, &mut rest_maxmin);

        let mut maxi_height_max = 0;
        loop {
            let mut new_maxi_pred: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
            let max_local_maxmin = step_one_past(&maxi_pred_set, &mut new_maxi_pred, &mut used_maxi, &mut maxi_maxmin);

            append_maps(&mut maxi_pred_set, &new_maxi_pred);
            drop(new_maxi_pred);

            if max_local_maxmin.max <= rest_maxmin.min {
                maxi_height_max = max_local_maxmin.max;
                break;
            }
        }

        debug!("tips_anticone_blue(): tip={} rest_height_min={} rest={:?} maxi_height_max={} max={:?} size_of_anticone={}", tip_name, rest_maxmin.min,
                 sorted_keys_by_height(&new_rest_pred, true).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>(),
                 maxi_height_max, sorted_keys_by_height(&maxi_pred_set, true).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>(),
                 anticone.len());
        let rest_keys = new_rest_pred.iter().map(|(k,_)|{k.clone()}).collect::<Vec<String>>();
        for name in &rest_keys {
            if maxi_pred_set.get(name).is_some() {
                new_rest_pred.remove(name);
            }
        }

        for (key, value) in &new_rest_pred {

            let rest = &value.read().unwrap();
            if rest.is_blue {
                if let Entry::Vacant(v) = anticone.entry(key.clone()) {
                    v.insert(Arc::clone(value));
                    anticone_blue_count += 1;
                }
            }
        }

        rest_pred_set = new_rest_pred;
        debug!("tips_anticone_blue(): tip={} size_of_anticone={} rest_pred_set={}", tip_name, anticone.len(), rest_pred_set.len());
    }
    debug!("tips_anticone_blue(): tip={} final result: size_of_anticone={}", tip_name, anticone.len());

    return (anticone_blue_count,anticone);
}


/// Function providing anti-cone blue counting, optimized for k: exit once counter > k already, but step in reverse direction.  Limitation: input block must be one of tips.
///
pub fn tips_anticone_blue_rev(tip_name: &str, tips: &HashMap<String, Arc<RwLock<Block>>>, k: i32) -> (i32,HashMap<String, Arc<RwLock<Block>>>){

    let mut anticone_blue_count: i32 = 0;
    let mut anticone: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();

    if tips.len()==0 {
        error!("tips_anticone_blue_rev(): tip={} error! tips is empty", tip_name);
        return (-1,anticone);
    }

    let mut maxi_pred_set: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
    let mut rest_pred_set: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();

    for (key, value) in tips {

        if key == tip_name {
            maxi_pred_set.insert(String::from(key.clone()), Arc::clone(value));
        }else {
            rest_pred_set.insert(String::from(key.clone()), Arc::clone(value));
        }
    }

    if maxi_pred_set.len()==0 {
        error!("tips_anticone_blue_rev(): error! tip {} is not in tips", tip_name);
        return (-1,HashMap::new());
    }

    debug!("tips_anticone_blue_rev(): tip={} size_of_anticone_blue={}", tip_name, anticone.len());

    let mut used_rest: HashMap<String,bool> = HashMap::new();
    let mut used_maxi: HashMap<String,bool> = HashMap::new();

    let mut rest_maxmin = MaxMin{max:0, min:<u64>::max_value()};
    let mut maxi_maxmin = MaxMin{max:0, min:<u64>::max_value()};

    while rest_pred_set.len() > 0 && anticone_blue_count <= k {

        let mut new_rest_pred: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
        let _rest_local_maxmin = step_one_next(&rest_pred_set, &mut new_rest_pred, &mut used_rest, &mut rest_maxmin);

        let mut maxi_height_min = 0;
        loop {
            let mut new_maxi_pred: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
            let max_local_maxmin = step_one_next(&maxi_pred_set, &mut new_maxi_pred, &mut used_maxi, &mut maxi_maxmin);

            append_maps(&mut maxi_pred_set, &new_maxi_pred);
            drop(new_maxi_pred);

            if max_local_maxmin.min >= rest_maxmin.max {
                maxi_height_min = max_local_maxmin.min;
                break;
            }
        }

        debug!("tips_anticone_blue_rev(): tip={} rest_height_max={} rest={:?} maxi_height_min={} max={:?} size_of_anticone={}", tip_name, rest_maxmin.max,
                 sorted_keys_by_height(&new_rest_pred, true).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>(),
                 maxi_height_min, sorted_keys_by_height(&maxi_pred_set, true).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>(),
                 anticone.len());
        let rest_keys = new_rest_pred.iter().map(|(k,_)|{k.clone()}).collect::<Vec<String>>();
        for name in &rest_keys {
            if maxi_pred_set.get(name).is_some() {
                new_rest_pred.remove(name);
            }
        }

        for (key, value) in &new_rest_pred {

            let rest = &value.read().unwrap();
            if rest.is_blue {
                if let Entry::Vacant(v) = anticone.entry(key.clone()) {
                    v.insert(Arc::clone(value));
                    anticone_blue_count += 1;
                }
            }
        }

        rest_pred_set = new_rest_pred;
        debug!("tips_anticone_blue_rev(): tip={} size_of_anticone_blue={} rest_pred_set={}", tip_name, anticone.len(), rest_pred_set.len());
    }
    debug!("tips_anticone_blue_rev(): tip={} final result: size_of_anticone_blue={}", tip_name, anticone.len());

    return (anticone_blue_count,anticone);
}

fn step_one_next(pred: &HashMap<String,Arc<RwLock<Block>>>, new_pred: &mut HashMap<String,Arc<RwLock<Block>>>, used: &mut HashMap<String,bool>, maxmin: &mut MaxMin) -> MaxMin{

    let mut local_maxmin = MaxMin{max:0, min:<u64>::max_value()};

    for (key, value) in pred {
        if let Entry::Vacant(v) = used.entry(key.clone()){

            let rest = Arc::clone(value);
            let rest = rest.read().unwrap();

            for (key2, value2) in &rest.next {

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



