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
use std::rc::Rc;
use std::cell::RefCell;

pub fn dag_add_block(name: &str, references: &Vec<&str>, dag: &mut HashMap<String, Rc<RefCell<Block>>>){

    //create this block
    let this_block = Rc::new(RefCell::new(Block{
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
                let refcell_reference_block = Rc::clone(block);
                let reference_block = refcell_reference_block.borrow();

                this_block.borrow_mut()
                    .prev.insert(reference_block.name.clone(), Rc::clone(block));
            }
        }
    }


    dag.insert(String::from(name.clone()), this_block);
}