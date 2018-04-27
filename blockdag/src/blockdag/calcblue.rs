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

use blockdag::{Block,Node,tips_anticone,tips_anticone_blue,anticone_blue,get_ltpq,get_stpq,sorted_keys_by_height,sizeof_pastset};

/// Function providing blue block calculation.
///
///     input 'block': a new added block to be calculated. before call this function, tips must have been updated for this new block.
///
pub fn calc_blue(block_name: &str, node: &mut Node, k: i32){

    debug!("calc_blue(): block {}. func enter.", block_name);

    let dag = &node.dag;

    {
        let block = dag.get(block_name);
        if block.is_none() {
            error!("calc_blue(): error! block {} not exist in dag.", block_name);
            return;
        }
    }   // scope to limit the lifetime of block.

    if block_name=="Genesis" {
        let mut block_w = dag.get(block_name).unwrap().write().unwrap();
        block_w.is_blue = true;
        block_w.size_of_anticone_blue = 0;
        return;
    }

    let tips = &node.tips;
    if tips.len() == 0 {
        error!("calc_blue(): error! tips must not be empty.");
        return;
    }

    // step 2
    let score_stpq = get_stpq(tips);

    debug!("calc_blue(): block {}.tip_max_name={},max_past_blue={}. tips={:?}", block_name, score_stpq[0].0, score_stpq[0].1,
           sorted_keys_by_height(tips, false).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>());

    // step 3
    if &score_stpq[0].0 == block_name {

        debug!("calc_blue(): step 3. block {}. new block is the max past blue", block_name);

        // step 4. clear all others tip's blue flag
        for (_key, value) in tips {

            let block = Arc::clone(value);
            let tip = &mut block.write().unwrap();
            if &tip.name != block_name {
                tip.is_blue = false;
                tip.size_of_anticone_blue = -1;
            }
        }

        // step 5
        for &(ref name,_,_) in &score_stpq {

            // step 6
            let (blues, blue_anticone) = tips_anticone_blue(name, tips, k);
            if blues < 0 || blues > k {
                debug!("calc_blue(): block {}. tip {} size_of_anticone_blue={} not blue.", block_name, name, blues);
            }else {
                // step 7
                {
                    let mut block_w = dag.get(name).unwrap().write().unwrap();
                    block_w.is_blue = true;
                    block_w.size_of_anticone_blue = blues;
                    drop(block_w);
                    debug!("calc_blue(): step 4.1. block {}. add {} to the blue. size_of_anticone_blue={}", block_name, name, blues);
                }   // scope to limit the lifetime of 'write()' lock.

                // step 8
                check_blue(&blue_anticone, k);
            }

        }   // scope to limit the lifetime of blue_anticone.

        // step 9
//        let block_r = dag.get(block_name).unwrap().read().unwrap();
//        let prev_keys = get_ltpq(&block_r.prev);
//        drop(block_r);  // must be released immediately, otherwise the following loop could enter deadlock.

        // another algorithm, to check all the anticone(bmax), not only the predecessor(z;G)
        let anticone_of_new = tips_anticone(block_name, tips);
        let prev_keys = get_ltpq(&anticone_of_new);
        drop(anticone_of_new);

        for &(ref name,_) in &prev_keys {

            if dag.get(name).unwrap().read().unwrap().is_blue {
                continue;
            }   // if expression has an implicit scope, so the 'read()' lock will be released immediately after if {}.

            debug!("calc_blue(): step 6. block {}. come to block {}", block_name, name);
            {
                // step 10
                let (blues, blue_anticone) = anticone_blue(name, node, tips, k);

                if blues >= 0 && blues <= k {

                    // step 11
                    debug!("calc_blue(): step 7. block {}. query block {}: size_of_anticone_blue={}. try to write_lock {}", block_name, name, blues, name);
                    {
                        let mut pred = dag.get(name).unwrap().write().unwrap();
                        pred.is_blue = true;
                        pred.size_of_anticone_blue = blues;
                        debug!("calc_blue(): step 7. block {}. add {} to the blue. size_of_anticone_blue={}", block_name, pred.name, blues);

                    }   // scope to limit the lifetime of 'write()' lock.

                    // step 12
                    check_blue(&blue_anticone, k);
                }
            }   // scope to limit the lifetime of blue_anticone.
        }

        // step 9. have to re-calculate 'size_of_past_blue' (for those successors) because 'blue' state changed.
        for &(ref name,_) in &prev_keys {

            let block = &dag.get(name).unwrap();
            let (_,size_of_past_blue) = sizeof_pastset(&block.read().unwrap());
            {
                let block_w = &mut block.write().unwrap();
                block_w.size_of_past_blue = size_of_past_blue;
            }
        }

    }else{

        debug!("calc_blue(): block {}. new block is not the max past blue", block_name);

        // step 16
        let (blues,blue_anticone) = tips_anticone_blue(block_name, tips, k);
        debug!("calc_blue(): step 11. block {}. size_of_anticone_blue={}", block_name, blues);
        if blues>=0 && blues<=k {

            let mut block_w = dag.get(block_name).unwrap().write().unwrap();

            // step 17
            block_w.is_blue = true;
            block_w.size_of_anticone_blue = blues;
            //println!("calc_blue(): block {}. add {} to the blue. size_of_anticone_blue={}", block_name, block_w.name, blues);
            drop(block_w);

            // step 18
            check_blue(&blue_anticone, k);
        }
    }


}

fn check_blue(blue_anticone: &HashMap<String, Arc<RwLock<Block>>>, _k: i32) {

    for (_key, value) in blue_anticone {

        //debug!("check_blue(): try to write_lock {}", key);
        let mut block_w = value.write().unwrap();
        block_w.size_of_anticone_blue += 1;
        //debug!("check_blue(): {} size_of_anticone_blue increase to {}", block_w.name, block_w.size_of_anticone_blue);
    }
}



// Function update all successors (recursively) of this block, if it's blue, size_of_past_blue minus 1.
//
// todo: this iteration could be terrible in performance!
//
//fn dec_successors_past_blue(block: &Block, used: &mut HashMap<String,bool>){
//
//    for (key, value) in &block.next {
//
//        if used.get(key).is_some() {
//            continue;
//        }else{
//            used.insert(key.clone(), true);
//        }
//
//        //debug!("dec_successors_anticone_blue(): try to write_lock {}", key);
//        {
//            let mut next = value.write().unwrap();
//            if next.is_blue {
//                next.size_of_past_blue -= 1;
//            }
//        }   // scope to limit the lifetime of 'write()' lock.
//
//        let next = &value.read().unwrap();
//        dec_successors_past_blue(next, used);
//    }
//}