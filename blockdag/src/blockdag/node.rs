
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
use blockdag::{dag_add_block,sorted_keys_by_height,calc_blue};

/// Structure providing fast access to node data.
///
pub struct Node{
    pub name: String,
    pub height: u64,
    pub size_of_dag: u64,
    pub dag: HashMap<String, Arc<RwLock<Block>>>,
    pub tips: HashMap<String, Arc<RwLock<Block>>>,
    pub classmates: HashMap<u64, Vec<String>>,
    pub hourglass: Vec<(u64,u64)>,
}

impl Node {
    pub fn init(node_name: &str) -> Arc<RwLock<Node>>{

        let node = Arc::new(RwLock::new(Node{
            name: String::from(node_name),
            height: 0,
            size_of_dag: 0,
            dag: HashMap::new(),
            tips: HashMap::new(),
            classmates: HashMap::new(),
            hourglass: Vec::new(),
        }));

        return node;
    }
}

impl fmt::Display for Node {

    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {

        let mut formatted_info = format!("node={},height={},size_of_dag={},dag={{", self.name, self.height, self.size_of_dag);

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


pub fn node_add_block(name_of_new_block: &str, references: &Vec<&str>, node: &mut Node, k: i32, do_update_tips: bool) {

    // add block
    {
        let dag = &mut node.dag;
        let classmates= &mut node.classmates;

        dag_add_block(name_of_new_block, references, dag);

        let block = dag.get(name_of_new_block);
        if block.is_some() {
            let block = Arc::clone(block.unwrap());
            let block = block.read().unwrap();
            if block.height > node.height {
                node.height = block.height;
            }

            // classmates update
            let classmate = classmates.entry(block.height).or_insert(vec![name_of_new_block.clone().into()]);
            if classmate.len() > 1 || classmate[0] != name_of_new_block  {
                classmate.push(name_of_new_block.clone().into());
            }
            //debug!("node_add_block(): new block={}. classmates update for height {}: {:?}", name_of_new_block, block.height, classmate);
            //todo: limit the classmates size, only keep latest heights.

            node.size_of_dag += 1;
        }else{
            warn!("node_add_block(): block not found in dag. dag_add_block failed?");
            return;
        }
    }

    if do_update_tips {

        // update tips
        update_tips(name_of_new_block, node);

        // keep this tips in the block as the snapshot tips
        {
            let block = Arc::clone(&node.dag.get(name_of_new_block).unwrap());
            let block_w = &mut block.write().unwrap();
            block_w.tips_snapshot = node.tips.clone();
        }

        // calculate blue
        calc_blue(name_of_new_block, node, k);
    }

}

pub fn update_tips(name_of_new_block: &str, node: &mut Node){

    //println!("update_tips(): new block={}", name_of_new_block);

    let dag = &node.dag;

    let block = dag.get(name_of_new_block);
    if block.is_none() {
        return;
    }

    let new_block = Arc::clone(block.unwrap());
    let new_block = new_block.read().unwrap();

    let mut to_be_removed: Vec<String> = Vec::new();

    for (prev, _value) in &new_block.prev {
        for (tip, _) in &node.tips {
            if prev==tip {
                to_be_removed.push(tip.to_string());
            }
        }
    }

    let tips = &mut node.tips;

    if to_be_removed.len()>0 {
        for item in &to_be_removed {
            tips.remove(item);
        }
    }

    tips.insert(new_block.name.clone(), Arc::clone(block.unwrap()));

    //println!("update_tips(): new block={}, removed={:?}, new tips={}", name_of_new_block, to_be_removed, tips.len());
}
