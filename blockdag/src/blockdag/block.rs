
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
use std::fmt;

/// Structure providing fast access to block data.
///
pub struct Block{
    pub name: String,
    pub height: u64,
    pub size_of_past_set: u64,
    pub prev: HashMap<String, Arc<RwLock<Block>>>,
    pub next: HashMap<String, Arc<RwLock<Block>>>,
}

pub struct MaxMin{
    pub max: u64,
    pub min: u64,
}

impl fmt::Display for Block {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut formated_info = format!("name={},height={},size_of_past_set={},prev={{", self.name, self.height, self.size_of_past_set);

        for (key, _value) in &self.prev {

            let tmp = format!("{},", key);
            formated_info.push_str(&tmp);
        }

        if self.prev.len() > 0 {
            formated_info.pop();
        }
        formated_info.push_str("}");

        write!(f, "{}", formated_info)
    }
}

pub fn append_maps(target: &mut HashMap<String,Arc<RwLock<Block>>>, source: &HashMap<String,Arc<RwLock<Block>>>){

    for (key, value) in source {

        if let Entry::Vacant(v) = target.entry(key.clone()){
            v.insert(Arc::clone(value));
        }
    }
}

/// Remove from the list all the block predecessors and successors which is in the list, self included.
///
pub fn remove_past_future(block: &Block, list: &mut HashMap<String, Arc<RwLock<Block>>>){

    let exist = list.remove(&String::from(block.name.clone()));
    if exist.is_none() {
        return;
    }

    remove_successors(block, list);
    remove_predecessors(block, list);
}


/// Remove from the list all the block successors which is in the list, self not included.
///
fn remove_successors(block: &Block, list: &mut HashMap<String, Arc<RwLock<Block>>>){

    for (_key, value) in &block.next {

        let next = Arc::clone(value);
        let next = next.read().unwrap();

        let exist = list.remove(&String::from(next.name.clone()));
        if exist.is_some() {
            remove_successors(&next, list);
        }
    }
}

/// Remove from the list all the block predecessors which is in the list, self not included.
///
fn remove_predecessors(block: &Block, list: &mut HashMap<String, Arc<RwLock<Block>>>){

    for (_key, value) in &block.prev {

        let prev = Arc::clone(value);
        let prev = prev.read().unwrap();

        let exist = list.remove(&String::from(prev.name.clone()));
        if exist.is_some() {
            remove_predecessors(&prev, list);
        }
    }
}

