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


#[cfg(test)]
mod tests {
    extern crate rand;
    extern crate time;

    use std::sync::{Arc};
    use self::rand::Rng;
    use self::time::{PreciseTime};

    use blockdag::{Node};
    use blockdag::{node_add_block,dag_print,tips_anticone,sorted_keys_by_height,remove_past_future,update_tips,calc_blue};

    #[test]
    fn test_fig3() {

        let node = Node::init("fig3");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w, true);
        calc_blue("Genesis", &mut node_w, 3);

        node_add_block("B", &vec!["Genesis"], &mut node_w, true);
        calc_blue("B", &mut node_w, 3);
        node_add_block("C", &vec!["Genesis"], &mut node_w, true);
        node_add_block("D", &vec!["Genesis"], &mut node_w, true);
        node_add_block("E", &vec!["Genesis"], &mut node_w, true);

        node_add_block("F", &vec!["B","C"], &mut node_w, true);
        node_add_block("H", &vec!["C","D","E"], &mut node_w, true);
        node_add_block("I", &vec!["E"], &mut node_w, true);

        node_add_block("J", &vec!["F","H"], &mut node_w, true);
        node_add_block("K", &vec!["B","H","I"], &mut node_w, true);
        node_add_block("L", &vec!["D","I"], &mut node_w, true);
        node_add_block("M", &vec!["F","K"], &mut node_w, true);

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_fig4() {

        let node = Node::init("fig4");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w, true);

        node_add_block("B", &vec!["Genesis"], &mut node_w, true);
        node_add_block("C", &vec!["Genesis"], &mut node_w, true);
        node_add_block("D", &vec!["Genesis"], &mut node_w, true);
        node_add_block("E", &vec!["Genesis"], &mut node_w, true);

        node_add_block("F", &vec!["B","C"], &mut node_w, true);
        node_add_block("H", &vec!["E"], &mut node_w, true);
        node_add_block("I", &vec!["C","D"], &mut node_w, true);

        node_add_block("J", &vec!["F","D"], &mut node_w, true);
        node_add_block("K", &vec!["J","I","E"], &mut node_w, true);
        node_add_block("L", &vec!["F"], &mut node_w, true);
        node_add_block("N", &vec!["D","H"], &mut node_w, true);

        node_add_block("M", &vec!["L","K"], &mut node_w, true);
        node_add_block("O", &vec!["K"], &mut node_w, true);
        node_add_block("P", &vec!["K"], &mut node_w, true);
        node_add_block("Q", &vec!["N"], &mut node_w, true);

        node_add_block("R", &vec!["O","P","N"], &mut node_w, true);

        node_add_block("S", &vec!["Q"], &mut node_w, true);
        node_add_block("T", &vec!["S"], &mut node_w, true);
        node_add_block("U", &vec!["T"], &mut node_w, true);

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_anticone() {

        let node = Node::init("block add test");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w, true);

        node_add_block("B", &vec!["Genesis"], &mut node_w, true);
        node_add_block("C", &vec!["Genesis"], &mut node_w, true);
        node_add_block("D", &vec!["Genesis"], &mut node_w, true);
        node_add_block("E", &vec!["Genesis"], &mut node_w, true);

        node_add_block("F", &vec!["B","C"], &mut node_w, true);
        node_add_block("H", &vec!["C","D","E"], &mut node_w, true);
        node_add_block("I", &vec!["E"], &mut node_w, true);

        let anticone = tips_anticone("H", &node_w.tips);
        let result = format!("anticone of {} = {:?}", "H", sorted_keys_by_height(&anticone, false));
        println!("{}",result);
        assert_eq!(result, "anticone of H = [(\"B\", 1), (\"F\", 2), (\"I\", 2)]");

        node_add_block("J", &vec!["F","H"], &mut node_w, true);
        node_add_block("K", &vec!["B","H","I"], &mut node_w, true);
        node_add_block("L", &vec!["D","I"], &mut node_w, true);
        node_add_block("M", &vec!["F","K"], &mut node_w, true);
//
//        let max_back_steps = 8;
//        let max_classmate_blocks = 5;
//        let max_prev_blocks = 5;

        let anticone = tips_anticone("M", &node_w.tips);
        let result = format!("anticone of {} = {:?}", "M", sorted_keys_by_height(&anticone, false));
        println!("{}",result);
        assert_eq!(result, "anticone of M = [(\"J\", 3), (\"L\", 3)]");
    }


    #[test]
    fn test_add_block() {
        let start = PreciseTime::now();

        let node = Node::init("block add test");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w, true);

        node_add_block("B", &vec!["Genesis"], &mut node_w, true);
        node_add_block("C", &vec!["Genesis"], &mut node_w, true);
        node_add_block("D", &vec!["Genesis"], &mut node_w, true);
        node_add_block("E", &vec!["Genesis"], &mut node_w, true);

        let max_classmate_blocks = 5;
        let max_prev_blocks = 5;

//        let anticone = tips_anticone("B", &node_w.tips, &node_w.dag);
//        let result = format!("anticone of {} = {:?}", "B",
//                             sorted_keys_by_height(&anticone, false).iter().map(|&(ref n,_)|{n}).collect::<Vec<_>>());
//        println!("{}",result);

        let mut blocks_generated = 0;

        for _height in 2..10000 {
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

                node_add_block(&blocks_generated.to_string(), &references_str,&mut node_w, false);

                //println!("{}", &node_w);

                //dag_print(&node_w.dag);
            }

            // update tips once when a batch of blocks generated.
            let mut classmate_name = blocks_generated;
            for _classmate in 1..classmate_blocks+1 {
                update_tips(&classmate_name.to_string(), &mut node_w);
                classmate_name -= 1;
            }
        }

        let end = PreciseTime::now();
        let d = start.to(end);
        let total_time_used = d.num_milliseconds() as f64;

        println!("node=\"{}\",height={},size_of_dag={}", node_w.name, node_w.height, node_w.size_of_dag);
        println!("total time used: {} (ms)", total_time_used);

        assert_eq!(2 + 2, 4);
    }
}
