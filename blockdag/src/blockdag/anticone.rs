
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
use std::sync::{Arc,RwLock};

use blockdag::{Block,MaxMin,append_maps};
use blockdag::{sorted_keys_by_height,step_one_past};

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
        //println!("tips_anticone(): error! tip {} is not in tips", tip_name);
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



