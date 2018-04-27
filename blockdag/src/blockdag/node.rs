
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

use blockdag::{Block,BlockRaw};
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
    pub mined_blocks: u64,
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
            mined_blocks: 0 as u64,
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

pub fn handle_block_tx(new_mined_block_name:&str, propagations: &mut HashMap<String, Arc<RwLock<BlockRaw>>>, node: &Node, total_nodes: i32) {

    let new_mined_block = &node.dag.get(new_mined_block_name).unwrap().read().unwrap();
    let prev_names = new_mined_block.prev.iter().map(|(k,_)|{k.clone()}).collect::<Vec<String>>();

    let new_block_raw = BlockRaw{
        name:new_mined_block_name.clone().to_string(),
        height: new_mined_block.height,
        size_of_past_set: new_mined_block.size_of_past_set,
        prev: prev_names,
        propagation: total_nodes,
    };

    propagations.insert(new_mined_block_name.clone().to_string(), Arc::new(RwLock::new(new_block_raw)));
    drop(propagations);
}

pub fn handle_block_rx(block_propagation_rx: &Arc<RwLock<HashMap<String, Arc<RwLock<BlockRaw>>>>>, node: &mut Node, node_stash: &mut HashMap<String, Arc<RwLock<BlockRaw>>>, k: i32) -> i32{

    let mut arrivals = block_propagation_rx.write().unwrap();

    let mut block_received_total: i32 = 0;

    let mut to_be_removed: Vec<String> = Vec::new();
    let stash = node_stash;

    {
        let dag = &node.dag;

        for (name_of_new_block, value) in &*arrivals {
            if dag.get(name_of_new_block).is_some(){
                continue; // block already received.
            }

            let is_real_new = stash.get(name_of_new_block).is_none();
            stash.entry(name_of_new_block.clone()).or_insert(Arc::clone(value));
            if is_real_new {
                block_received_total += 1;
                let arrival = &mut value.write().unwrap();

                arrival.propagation -= 1;
                if arrival.propagation <= 0 {
                    to_be_removed.push(name_of_new_block.clone());
                }
            }
        }
    }

    for name in &to_be_removed {
        arrivals.remove(name);
    }

    // important note: the 'arrivals' must be drop lock as soon as possible by node.
    drop(arrivals);

    // after drop lock, start local processing with stash

    let mut block_added: Vec<String> = Vec::new();
    loop {
        'outer: for (name_of_stash_block, value) in &*stash {
            let stash_block = Arc::clone(value);
            let stash_block = stash_block.read().unwrap();

            // before adding to dag, make sure all its predecessors are already in dag, otherwise skip it for this time.
            for prev in &stash_block.prev {
                let dag = &node.dag;
                if dag.get(prev).is_none() {
                    continue 'outer;
                }
            }

            let prev_names = stash_block.prev.iter().map(|k| { &k[..] }).collect::<Vec<&str>>();
            if true == node_add_block(name_of_stash_block, &prev_names, node, k, true) {
                block_added.push(name_of_stash_block.clone());
            }
        }

        for added_name in &block_added {
            stash.remove(added_name);
        }

        if block_added.len()==0 {
            break;
        }
        // in cast one released stash block could release another stash block, loop check.
        block_added.truncate(0);
    }

    return block_received_total;
}

pub fn node_add_block(name_of_new_block: &str, references: &Vec<&str>, node: &mut Node, k: i32, do_update_tips: bool) -> bool {

    // add block
    {
        let dag = &mut node.dag;
        if dag.get(name_of_new_block).is_some(){
            return false; // block already received.
        }

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
            return false;
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

    return true;
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
