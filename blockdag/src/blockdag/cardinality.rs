
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
use blockdag::Block;
use std::sync::{Arc,RwLock};

/// Function providing cardinality of pastset blocks calculation.
///
pub fn sizeof_pastset(block: &Block, dag: &HashMap<String, Arc<RwLock<Block>>>) -> u64{

    if block.prev.len()==0 {
        return 0;
    }

    let mut size_of_past: u64 = 0;

    let mut max_pred_set: HashMap<String,Arc<RwLock<Block>>> = HashMap::new();
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
        panic!("cardi_pastset(): impossible! bmax=nil.");
    }

    let bmax_block = block.prev.get(&bmax_name).unwrap();
    max_pred_set.insert(bmax_name.clone(), Arc::clone(bmax_block));

    rest_pred_set.remove(&bmax_name);

    size_of_past = max_sizeofpast;



    return size_of_past;
}

