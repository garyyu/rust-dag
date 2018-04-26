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

pub mod blockdag;


#[macro_use]
extern crate log;

#[cfg(test)]
mod tests {

    extern crate env_logger;

    extern crate rand;
    extern crate time;

    use std::collections::HashMap;
    use std::sync::{Arc,RwLock};
    use self::rand::Rng;
    use self::time::{PreciseTime};

    use blockdag::{Node,BlockRaw};
    use blockdag::{node_add_block,dag_print,dag_blue_print,tips_anticone,sorted_keys_by_height,remove_past_future,update_tips,calc_blue,handle_block_rx,handle_block_tx};
    use std::thread;
    use std::time::Duration;

    #[test]
    fn test_fig3() {

        let k: i32 = 3;

        let _ = env_logger::try_init();

        let node = Node::init("fig3");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w, k, true);

        node_add_block("B", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("C", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("D", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("E", &vec!["Genesis"], &mut node_w, k, true);

        node_add_block("F", &vec!["B","C"], &mut node_w, k, true);
        node_add_block("H", &vec!["C","D","E"], &mut node_w, k, true);
        node_add_block("I", &vec!["E"], &mut node_w, k, true);

        node_add_block("J", &vec!["F","H"], &mut node_w, k, true);
        node_add_block("K", &vec!["B","H","I"], &mut node_w, k, true);
        node_add_block("L", &vec!["D","I"], &mut node_w, k, true);
        node_add_block("N", &vec!["L","K"], &mut node_w, k, true);
        node_add_block("M", &vec!["F","K"], &mut node_w, k, true);

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        let blue_selection = dag_blue_print(&node_w.dag);
        println!("k={}, {}", k, &blue_selection);

        assert_eq!(&blue_selection, "blues={Genesis,B,C,D,F,H,J,K,M,N,} total=10/13");
    }

    #[test]
    fn test_fig4() {

        let k: i32 = 3;

        let _ = env_logger::try_init();

        let node = Node::init("fig4");

        let mut node_w = node.write().unwrap();

        macro_rules! dag_add {
            ( block=$a:expr, references=$b:expr ) => (node_add_block($a, $b, &mut node_w, k, true));
        }
        dag_add!(block="Genesis", references=&Vec::new());

        dag_add!(block="B", references=&vec!["Genesis"]);
        dag_add!(block="C", references=&vec!["Genesis"]);
        dag_add!(block="D", references=&vec!["Genesis"]);
        dag_add!(block="E", references=&vec!["Genesis"]);

        dag_add!(block="F", references=&vec!["B","C"]);
        dag_add!(block="H", references=&vec!["E"]);
        dag_add!(block="I", references=&vec!["C","D"]);

        dag_add!(block="J", references=&vec!["F","D"]);
        dag_add!(block="K", references=&vec!["J","I","E"]);
        dag_add!(block="L", references=&vec!["F"]);
        dag_add!(block="N", references=&vec!["D","H"]);

        dag_add!(block="M", references=&vec!["L","K"]);
        dag_add!(block="O", references=&vec!["K"]);
        dag_add!(block="P", references=&vec!["K"]);
        dag_add!(block="Q", references=&vec!["N"]);

        dag_add!(block="R", references=&vec!["O","P","N"]);

        dag_add!(block="S", references=&vec!["Q"]);
        dag_add!(block="T", references=&vec!["S"]);
        dag_add!(block="U", references=&vec!["T"]);

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        let blue_selection = dag_blue_print(&node_w.dag);
        println!("k={}, {}", k, &blue_selection);

        assert_eq!(&blue_selection, "blues={Genesis,B,C,D,F,I,J,K,M,O,P,R,} total=12/20");
    }

    #[test]
    fn test_anticone() {

        let k: i32 = 3;

        let node = Node::init("block add test");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w, k, true);

        node_add_block("B", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("C", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("D", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("E", &vec!["Genesis"], &mut node_w, k, true);

        node_add_block("F", &vec!["B","C"], &mut node_w, k, true);
        node_add_block("H", &vec!["C","D","E"], &mut node_w, k, true);
        node_add_block("I", &vec!["E"], &mut node_w, k, true);

        let anticone = tips_anticone("H", &node_w.tips);
        let result = format!("anticone of {} = {:?}", "H", sorted_keys_by_height(&anticone, false));
        println!("{}",result);
        assert_eq!(result, "anticone of H = [(\"B\", 1), (\"F\", 2), (\"I\", 2)]");

        node_add_block("J", &vec!["F","H"], &mut node_w, k, true);
        node_add_block("K", &vec!["B","H","I"], &mut node_w, k, true);
        node_add_block("L", &vec!["D","I"], &mut node_w, k, true);
        node_add_block("M", &vec!["F","K"], &mut node_w, k, true);

        let anticone = tips_anticone("M", &node_w.tips);
        let result = format!("anticone of {} = {:?}", "M", sorted_keys_by_height(&anticone, false));
        println!("{}",result);
        assert_eq!(result, "anticone of M = [(\"J\", 3), (\"L\", 3)]");
    }


    #[test]
    fn test_add_block() {

        let blocks_generating:i32 = 1000;

        let max_classmate_blocks = 3;
        let max_prev_blocks = 5;

        let k: i32 = max_classmate_blocks;

        let start = PreciseTime::now();

        let node = Node::init("block add test");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w, k, true);

        node_add_block("B", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("C", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("D", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("E", &vec!["Genesis"], &mut node_w, k, true);

        let mut blocks_generated = 0;

        let mut _height:i32 = 1;
        while blocks_generated < blocks_generating {
            _height += 1;
            let classmate_blocks = rand::thread_rng().gen_range(1, max_classmate_blocks+1);
//            let back_steps = rand::thread_rng().gen_range(1, max_back_steps+1);
            //println!("height={} classmate_blocks={}", height, classmate_blocks);

            for _classmate in 1..classmate_blocks+1 {

                let prev_blocks = rand::thread_rng().gen_range(1, max_prev_blocks+1);
                //println!("height={} classmate={} prev_blocks={}", height, classmate, prev_blocks);

                let mut references = Vec::new();

                // get one block from tips as 1st prev
                let mut tip_name_selected = String::new();
                for (key, _) in node_w.tips.iter() {
                    references.push(key.clone());
                    tip_name_selected.push_str(key);
                    break;  // just take one tip only.
                }

                // randomly select one from the anticone of that tip
                let mut anticone = tips_anticone(&tip_name_selected, &node_w.tips);

                while references.len() < prev_blocks && anticone.len()>0 {

                    let mut anticone_clone = anticone.clone();

                    for (key, value) in anticone.iter() {
                        if references.len() >= prev_blocks {
                            break;
                        }

                        let block = Arc::clone(value);
                        let block = block.read().unwrap();

                        references.push(key.clone());

                        // update anticone to remove all the past of this new referenced block.
                        remove_past_future(&block, &mut anticone_clone);
                        break;
                    }

                    anticone = anticone_clone;
                    //println!("height={} classmate={} classmate_blocks={} prev_blocks={} references={:?} anticone size={}", height, classmate, classmate_blocks, prev_blocks, references, anticone.len());
                }

                //println!("height={} classmate={} classmate_blocks={} prev_blocks={} references={:?}", height, classmate, classmate_blocks, prev_blocks, references);

                blocks_generated += 1;

                let mut references_str:Vec<&str> = Vec::new();
                for reference in &references {
                    references_str.push(reference);
                }

                let block_name = format!("{:04}", blocks_generated);
                node_add_block(&block_name, &references_str,&mut node_w, k, false);

                //println!("{}", &node_w);

                //dag_print(&node_w.dag);
            }

            // update tips once when a batch of blocks generated.
            let mut classmate_name = blocks_generated;
            for _classmate in 1..classmate_blocks+1 {
                let block_name = format!("{:04}", classmate_name);
                update_tips(&block_name, &mut node_w);
                calc_blue(&block_name, &mut node_w, k);
                classmate_name -= 1;
            }
        }

        let end = PreciseTime::now();
        let d = start.to(end);
        let total_time_used = d.num_milliseconds() as f64;

        dag_print(&node_w.dag);

        println!("node=\"{}\",height={},size_of_dag={}", node_w.name, node_w.height, node_w.size_of_dag);
        println!("total time used: {} (ms)", total_time_used);

        let blue_selection = dag_blue_print(&node_w.dag);
        println!("k={}, {}", k, &blue_selection);

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_fig_x1() {

        let k: i32 = 0;

        let _ = env_logger::try_init();

        let node = Node::init("figX1");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w, k, true);

        node_add_block("B", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("C", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("D", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("E", &vec!["Genesis"], &mut node_w, k, true);

        node_add_block("01", &vec!["B","C","D","E"], &mut node_w, k, true);
        node_add_block("02", &vec!["B","E"], &mut node_w, k, true);
        node_add_block("03", &vec!["B","C","D","E"], &mut node_w, k, true);
        node_add_block("04", &vec!["E"], &mut node_w, k, true);

        node_add_block("05", &vec!["01","04"], &mut node_w, k, true);
        node_add_block("06", &vec!["01","03","04"], &mut node_w, k, true);
        node_add_block("07", &vec!["01","02"], &mut node_w, k, true);

        node_add_block("08", &vec!["02","03","05"], &mut node_w, k, true);
        node_add_block("09", &vec!["05","06","07"], &mut node_w, k, true);

        node_add_block("10", &vec!["08","09"], &mut node_w, k, true);
        node_add_block("11", &vec!["08","09"], &mut node_w, k, true);

        node_add_block("12", &vec!["11"], &mut node_w, k, true);
        node_add_block("13", &vec!["10","11"], &mut node_w, k, true);

        node_add_block("14", &vec!["13"], &mut node_w, k, true);
        node_add_block("15", &vec!["12","13"], &mut node_w, k, true);

        node_add_block("16", &vec!["12","14"], &mut node_w, k, true);
        node_add_block("17", &vec!["15","16"], &mut node_w, k, true);
        node_add_block("18", &vec!["16"], &mut node_w, k, true);

        node_add_block("19", &vec!["17","18"], &mut node_w, k, true);
        node_add_block("20", &vec!["17","18"], &mut node_w, k, true);
        node_add_block("21", &vec!["17"], &mut node_w, k, true);
        node_add_block("22", &vec!["17","18"], &mut node_w, k, true);
        node_add_block("23", &vec!["17","18"], &mut node_w, k, true);

        node_add_block("24", &vec!["19","23"], &mut node_w, k, true);
        node_add_block("25", &vec!["23"], &mut node_w, k, true);
        node_add_block("26", &vec!["23"], &mut node_w, k, true);

        node_add_block("27", &vec!["20","22","24","26"], &mut node_w, k, true);
        node_add_block("28", &vec!["21","22","24"], &mut node_w, k, true);
        node_add_block("29", &vec!["22","24","25","26"], &mut node_w, k, true);
        node_add_block("30", &vec!["21","24","25","26"], &mut node_w, k, true);
        node_add_block("31", &vec!["24"], &mut node_w, k, true);

        node_add_block("32", &vec!["22","25","31"], &mut node_w, k, true);
        node_add_block("33", &vec!["26","31"], &mut node_w, k, true);
        node_add_block("34", &vec!["22","31"], &mut node_w, k, true);

        node_add_block("35", &vec!["20","26","28","34"], &mut node_w, k, true);
        node_add_block("36", &vec!["20","28","30","33","34"], &mut node_w, k, true);
        node_add_block("37", &vec!["32"], &mut node_w, k, true);
        node_add_block("38", &vec!["20","32","33"], &mut node_w, k, true);
        node_add_block("39", &vec!["32"], &mut node_w, k, true);

        node_add_block("40", &vec!["21","33","37","39"], &mut node_w, k, true);
        node_add_block("41", &vec!["21","26","34","37"], &mut node_w, k, true);

        node_add_block("42", &vec!["27","29","36","39","41"], &mut node_w, k, true);
        node_add_block("43", &vec!["28","29","33","41"], &mut node_w, k, true);
        node_add_block("44", &vec!["29","32"], &mut node_w, k, true);
        node_add_block("45", &vec!["27","29","36","38","40"], &mut node_w, k, true);

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        let blue_selection = dag_blue_print(&node_w.dag);
        println!("k={}, {}", k, &blue_selection);

        assert_eq!(2 + 2, 4);
    }


    #[test]
    fn test_fig_x2() {

        let k: i32 = 0;

        let _ = env_logger::try_init();

        let node = Node::init("figX2");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w, k, true);

        node_add_block("01", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("02", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("03", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("04", &vec!["Genesis"], &mut node_w, k, true);
        node_add_block("05", &vec!["Genesis"], &mut node_w, k, true);

        node_add_block("06", &vec!["01","02","03","04","05"], &mut node_w, k, true);
        node_add_block("07", &vec!["01","02","03","04","05"], &mut node_w, k, true);
        node_add_block("08", &vec!["01","02","03","04","05"], &mut node_w, k, true);
        node_add_block("09", &vec!["01","02","03","04","05"], &mut node_w, k, true);
        node_add_block("10", &vec!["01","02","03","04","05"], &mut node_w, k, true);

        node_add_block("11", &vec!["06","07","08","09","10"], &mut node_w, k, true);
        node_add_block("12", &vec!["06","07","08","09","10"], &mut node_w, k, true);
        node_add_block("13", &vec!["06","07","08","09","10"], &mut node_w, k, true);
        node_add_block("14", &vec!["06","07","08","09","10"], &mut node_w, k, true);
        node_add_block("15", &vec!["06","07","08","09","10"], &mut node_w, k, true);

        node_add_block("16", &vec!["11","12","13","14","15"], &mut node_w, k, true);
        node_add_block("17", &vec!["11","12","13","14","15"], &mut node_w, k, true);
        node_add_block("18", &vec!["11","12","13","14","15"], &mut node_w, k, true);
        node_add_block("19", &vec!["11","12","13","14","15"], &mut node_w, k, true);
        node_add_block("20", &vec!["11","12","13","14","15"], &mut node_w, k, true);

        node_add_block("21", &vec!["16","17","18","19","20"], &mut node_w, k, true);
        node_add_block("22", &vec!["16","17","18","19","20"], &mut node_w, k, true);
        node_add_block("23", &vec!["16","17","18","19","20"], &mut node_w, k, true);
        node_add_block("24", &vec!["16","17","18","19","20"], &mut node_w, k, true);
        node_add_block("25", &vec!["16","17","18","19","20"], &mut node_w, k, true);

        node_add_block("26", &vec!["21","22","23","24","25"], &mut node_w, k, true);
        node_add_block("27", &vec!["21","22","23","24","25"], &mut node_w, k, true);
        node_add_block("28", &vec!["21","22","23","24","25"], &mut node_w, k, true);
        node_add_block("29", &vec!["21","22","23","24","25"], &mut node_w, k, true);
        node_add_block("30", &vec!["21","22","23","24","25"], &mut node_w, k, true);

        node_add_block("31", &vec!["26","27","28","29","30"], &mut node_w, k, true);
        node_add_block("32", &vec!["26","27","28","29","30"], &mut node_w, k, true);
        node_add_block("33", &vec!["26","27","28","29","30"], &mut node_w, k, true);
        node_add_block("34", &vec!["26","27","28","29","30"], &mut node_w, k, true);
        node_add_block("35", &vec!["26","27","28","29","30"], &mut node_w, k, true);

        node_add_block("36", &vec!["31","32","33","34","35"], &mut node_w, k, true);
        node_add_block("37", &vec!["31","32","33","34","35"], &mut node_w, k, true);
        node_add_block("38", &vec!["31","32","33","34","35"], &mut node_w, k, true);
        node_add_block("39", &vec!["31","32","33","34","35"], &mut node_w, k, true);
        node_add_block("40", &vec!["31","32","33","34","35"], &mut node_w, k, true);

        node_add_block("41", &vec!["36","37","38","39","40"], &mut node_w, k, true);
        node_add_block("42", &vec!["36","37","38","39","40"], &mut node_w, k, true);
        node_add_block("43", &vec!["36","37","38","39","40"], &mut node_w, k, true);
        node_add_block("44", &vec!["36","37","38","39","40"], &mut node_w, k, true);
        node_add_block("45", &vec!["36","37","38","39","40"], &mut node_w, k, true);

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        let blue_selection = dag_blue_print(&node_w.dag);
        println!("k={}, {}", k, &blue_selection);

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_nodes_sync() {

        const TOTAL_NODES: i32 = 2;        // how many nodes to simulate. each node is a thread spawn.
        let blocks_generating:i32 = 10;   // how many blocks mining for this test.
        const K: i32 = 3;                    // how many blocks generating in parallel.

        let start = PreciseTime::now();

        // important note: the token-ring locker must be drop as soon as possible by node.
        let block_token_ring: Arc<RwLock<HashMap<String, Arc<RwLock<BlockRaw>>>>> = Arc::new(RwLock::new(HashMap::new()));
        let mining_token_ring = Arc::new(RwLock::new(0 as i32));
        let blocks_generated = Arc::new(RwLock::new(0 as i32));

        let mut handles = vec![];

        for number in 0..TOTAL_NODES {

            let mining = Arc::clone(&mining_token_ring);
            let block_propagation = Arc::clone(&block_token_ring);
            let blocks_generated = Arc::clone(&blocks_generated);

            let handle = thread::spawn(move || {

                let node = Node::init(&format!("node{}", number));
                let mut node_stash: HashMap<String, Arc<RwLock<BlockRaw>>> = HashMap::new();
                let mut node_w = node.write().unwrap();

                node_add_block("Genesis", &Vec::new(), &mut node_w, K, true);

                loop {
                    let mining_lock = mining.read().unwrap();
                    if *mining_lock == -1 {
                        break;  // ask thread to exit

                    }else if *mining_lock == 0 {
                        drop(mining_lock);

                        // processing block propagation
                        let mut arrivals = block_propagation.write().unwrap();
                        handle_block_rx(&mut arrivals, &mut node_w, &mut node_stash, K);
                        thread::sleep(Duration::from_millis(1));
                        continue;
                    }

                    drop(mining_lock);
                    let mut mining_lock = mining.write().unwrap();
                    *mining_lock -= 1;
                    drop(mining_lock);

                    let mut blocks_generated_w = blocks_generated.write().unwrap();
                    *blocks_generated_w += 1;
                    let block_name = format!("{:04}", blocks_generated_w);
                    drop(blocks_generated_w);

                    let references_str = node_w.tips.iter().map(|(k,_)|{k.clone()}).collect::<Vec<String>>();
                    node_add_block(&block_name, &references_str.iter().map(|s| s.as_ref()).collect(), &mut node_w, K, true);

                    //println!("{}", &node_w);

                    calc_blue(&block_name, &mut node_w, K);

                    // propagate this new mined block
                    let mut propagations = block_propagation.write().unwrap();
                    handle_block_tx(&block_name, &mut propagations, &node_w, TOTAL_NODES);
                }
            });

            handles.push(handle);
        }

        loop {
            let mut mining = mining_token_ring.write().unwrap();
            *mining = K;
            drop(mining);
            thread::sleep(Duration::from_millis(500));

            {
                let blocks_generated_r = blocks_generated.read().unwrap();
                if *blocks_generated_r >= blocks_generating {
                    drop(blocks_generated_r);

                    let mut mining = mining_token_ring.write().unwrap();
                    *mining = -1;   // ask nodes stop and exit.
                    drop(mining);
                    break;
                }
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let propagation = block_token_ring.read().unwrap();
        println!("test_nodes_sync(): block propagation hashmap remaining size: {}", propagation.len());


        let end = PreciseTime::now();
        let d = start.to(end);
        let total_time_used = d.num_milliseconds() as f64;

        //dag_print(&node_w.dag);

        //println!("node=\"{}\",height={},size_of_dag={}", node_w.name, node_w.height, node_w.size_of_dag);
        println!("total time used: {} (ms)", total_time_used);

        //let blue_selection = dag_blue_print(&node_w.dag);
        //println!("k={}, {}", k, &blue_selection);

        assert_eq!(2 + 2, 4);
    }
}
