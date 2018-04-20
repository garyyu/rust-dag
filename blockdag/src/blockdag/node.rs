
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
use std::fmt;

use blockdag::Block;
use blockdag::{dag_add_block,dag_print,sorted_keys_by_height};

/// Structure providing fast access to block data.
///
pub struct Node{
    pub name: String,
    pub height: u64,
    pub size_of_dag: u64,
    pub dag: HashMap<String, Arc<RwLock<Block>>>,
    pub tips: HashMap<String, Arc<RwLock<Block>>>,
}

impl fmt::Display for Node {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut formatted_info = format!("node={},height={},size_of_past_set={},dag={{", self.name, self.height, self.size_of_dag);

        let sorted_keys = sorted_keys_by_height(&self.dag, false);

        for (name,_) in sorted_keys {
            let tmp = format!("{},", &name);
            formatted_info.push_str(&tmp);
        }

        if self.dag.len() > 0 {
            formatted_info.pop();
        }
        formatted_info.push_str("},tips={");

        for (key, _value) in &self.tips {
            let tmp = format!("{},", key);
            formatted_info.push_str(&tmp);
        }

        if self.tips.len() > 0 {
            formatted_info.pop();
        }

        formatted_info.push_str("}");

        write!(f, "{}", formatted_info)
    }
}


pub fn node_add_block(name: &str, references: &Vec<&str>, node: &mut Node) {

    let dag = &mut node.dag;

    dag_add_block(name, references, dag);
}
