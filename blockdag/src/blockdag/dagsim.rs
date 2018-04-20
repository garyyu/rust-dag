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

use blockdag::Block;
use blockdag::{sizeof_pastset,sorted_keys_by_height};

pub fn dag_add_block(name: &str, references: &Vec<&str>, dag: &mut HashMap<String, Arc<RwLock<Block>>>){

    //create this block
    let this_block = Arc::new(RwLock::new(Block{
        name: String::from(name.clone()),
        height: 0,
        size_of_past_set: 0,
        prev: HashMap::new(),
        next: HashMap::new(),
    }));

    //add references
    for reference in references {
        let value = dag.get(*reference);
        match value {
            None => {
                let except_message = format!("dag_add_block(): error! block reference invalid. block name = {} references = {}", name, reference);
                panic!(except_message);
            },
            Some(block) => {
                let reference_block = Arc::clone(block);

                // add previous blocks to this block
                {
                    let reference_block = reference_block.read().unwrap();

                    let mut this_block_w = this_block.write().unwrap();
                    this_block_w.prev.insert(reference_block.name.clone(), Arc::clone(block));

                    // height is the maximum previous height +1
                    if reference_block.height+1 > this_block_w.height {
                        this_block_w.height = reference_block.height+1;
                    }
                }

                // add self as previous block's next
                let mut reference_block = reference_block.write().unwrap();
                reference_block.next.insert(String::from(name.clone()), Arc::clone(&this_block));
            }
        }
    }

    // size of pastset
    let size_of_past_set = sizeof_pastset(&this_block.read().unwrap(), dag);
    {
        let mut this_block_w = this_block.write().unwrap();
        this_block_w.size_of_past_set = size_of_past_set;
    }

    dag.insert(String::from(name.clone()), this_block);
}

pub fn dag_print(dag: &HashMap<String, Arc<RwLock<Block>>>){

    let sorted_keys = sorted_keys_by_height(dag, false);

    println!("dag={{");
    for (name,_) in sorted_keys {
        let block = dag.get(&name);
        if block.is_some() {
            let block = Arc::clone(block.unwrap());
            let block = block.read().unwrap();
            println!(" {{name={},block={}}}", name, block);
        }
    }
    println!("}}");
}