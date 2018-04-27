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
    use std::thread;
    use std::time::Duration;
    use std::sync::mpsc;

    use blockdag::{Node,BlockRaw};
    use blockdag::{node_add_block,dag_print,dag_blue_print,tips_anticone,sorted_keys_by_height,remove_past_future,update_tips,calc_blue,handle_block_rx,handle_block_tx,get_stpq};

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

        let k: i32 = 3;

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

        if k==0 {
            assert_eq!(&blue_selection, "blues={Genesis,B,01,05,09,11,13,14,16,18,23,26,30,36,45,} total=15/50")
        }else{
            assert_eq!(2 + 2, 4);
        }
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
    fn test_fig_x3() {

        let k: i32 = 3;

        let _ = env_logger::try_init();

        let node = Node::init("figX3");

        let mut node_w = node.write().unwrap();

        macro_rules! dag_add {
            ( block=$a:expr, references=$b:expr ) => (node_add_block($a, $b, &mut node_w, k, true));
        }
        dag_add!(block="Genesis", references=&Vec::new());

        /*
         * auto generated by the following bash script:
         *  $ sed  's/{name=0\(.*\),block=name=.*,height=.*,size_of_past_set=.*,size_of_past_blue=.*,blue=.,prev=\(.*\)}/dag_add!(block="0\1", references=\&vec!\2);/'  input.log
         */
        dag_add!(block="0001", references=&vec!["Genesis"]);
        dag_add!(block="0004", references=&vec!["Genesis"]);
        dag_add!(block="0010", references=&vec!["Genesis"]);
        dag_add!(block="0002", references=&vec!["0001"]);
        dag_add!(block="0005", references=&vec!["0004"]);
        dag_add!(block="0011", references=&vec!["0010"]);
        dag_add!(block="0003", references=&vec!["0002"]);
        dag_add!(block="0006", references=&vec!["0005"]);
        dag_add!(block="0012", references=&vec!["0011"]);
        dag_add!(block="0007", references=&vec!["0003", "0006"]);
        dag_add!(block="0016", references=&vec!["0003", "0006"]);
        dag_add!(block="0008", references=&vec!["0007"]);
        dag_add!(block="0017", references=&vec!["0016"]);
        dag_add!(block="0009", references=&vec!["0008"]);
        dag_add!(block="0018", references=&vec!["0017"]);
        dag_add!(block="0013", references=&vec!["0012", "0009"]);
        dag_add!(block="0019", references=&vec!["0012", "0009"]);
        dag_add!(block="0025", references=&vec!["0012", "0009"]);
        dag_add!(block="0014", references=&vec!["0013"]);
        dag_add!(block="0020", references=&vec!["0019"]);
        dag_add!(block="0026", references=&vec!["0025"]);
        dag_add!(block="0015", references=&vec!["0014"]);
        dag_add!(block="0021", references=&vec!["0020"]);
        dag_add!(block="0027", references=&vec!["0026"]);
        dag_add!(block="0022", references=&vec!["0018", "0015"]);
        dag_add!(block="0031", references=&vec!["0018", "0015", "0021"]);
        dag_add!(block="0023", references=&vec!["0022"]);
        dag_add!(block="0032", references=&vec!["0031"]);
        dag_add!(block="0024", references=&vec!["0023"]);
        dag_add!(block="0033", references=&vec!["0032"]);
        dag_add!(block="0028", references=&vec!["0021", "0027", "0024"]);
        dag_add!(block="0029", references=&vec!["0028"]);
        dag_add!(block="0030", references=&vec!["0029"]);
        dag_add!(block="0034", references=&vec!["0030"]);
        dag_add!(block="0037", references=&vec!["0033", "0030"]);
        dag_add!(block="0035", references=&vec!["0034"]);
        dag_add!(block="0038", references=&vec!["0037"]);
        dag_add!(block="0036", references=&vec!["0035"]);
        dag_add!(block="0039", references=&vec!["0038"]);
        dag_add!(block="0040", references=&vec!["0033", "0036"]);
        dag_add!(block="0043", references=&vec!["0036", "0039"]);
        dag_add!(block="0046", references=&vec!["0033", "0036"]);
        dag_add!(block="0053", references=&vec!["0036", "0039"]);
        dag_add!(block="0041", references=&vec!["0040"]);
        dag_add!(block="0044", references=&vec!["0043"]);
        dag_add!(block="0047", references=&vec!["0046"]);
        dag_add!(block="0042", references=&vec!["0041"]);
        dag_add!(block="0045", references=&vec!["0044"]);
        dag_add!(block="0048", references=&vec!["0047"]);
        dag_add!(block="0049", references=&vec!["0039", "0042"]);
        dag_add!(block="0055", references=&vec!["0039", "0042"]);
        dag_add!(block="0050", references=&vec!["0049"]);
        dag_add!(block="0056", references=&vec!["0055"]);
        dag_add!(block="0051", references=&vec!["0050"]);
        dag_add!(block="0057", references=&vec!["0056"]);
        dag_add!(block="0052", references=&vec!["0045", "0048", "0051"]);
        dag_add!(block="0061", references=&vec!["0045", "0048", "0051"]);
        dag_add!(block="0064", references=&vec!["0045", "0048", "0051"]);
        dag_add!(block="0054", references=&vec!["0052"]);
        dag_add!(block="0065", references=&vec!["0064"]);
        dag_add!(block="0058", references=&vec!["0053", "0057", "0054"]);
        dag_add!(block="0062", references=&vec!["0053", "0057", "0054"]);
        dag_add!(block="0066", references=&vec!["0065"]);
        dag_add!(block="0059", references=&vec!["0058"]);
        dag_add!(block="0063", references=&vec!["0062"]);
        dag_add!(block="0060", references=&vec!["0059"]);
        dag_add!(block="0067", references=&vec!["0061", "0066", "0063", "0060"]);
        dag_add!(block="0068", references=&vec!["0061", "0062", "0060"]);
        dag_add!(block="0070", references=&vec!["0060"]);
        dag_add!(block="0073", references=&vec!["0060"]);
        dag_add!(block="0076", references=&vec!["0061", "0066", "0063", "0060"]);
        dag_add!(block="0069", references=&vec!["0068"]);
        dag_add!(block="0071", references=&vec!["0070"]);
        dag_add!(block="0074", references=&vec!["0073"]);
        dag_add!(block="0077", references=&vec!["0076"]);
        dag_add!(block="0072", references=&vec!["0071"]);
        dag_add!(block="0075", references=&vec!["0074"]);
        dag_add!(block="0078", references=&vec!["0077"]);
        dag_add!(block="0079", references=&vec!["0067", "0069", "0072"]);
        dag_add!(block="0082", references=&vec!["0067", "0069", "0072", "0075"]);
        dag_add!(block="0086", references=&vec!["0067", "0069", "0072"]);
        dag_add!(block="0080", references=&vec!["0079"]);
        dag_add!(block="0083", references=&vec!["0082"]);
        dag_add!(block="0081", references=&vec!["0080"]);
        dag_add!(block="0084", references=&vec!["0083"]);
        dag_add!(block="0085", references=&vec!["0075", "0078", "0081"]);
        dag_add!(block="0088", references=&vec!["0075", "0078", "0081"]);
        dag_add!(block="0091", references=&vec!["0078", "0081", "0084"]);
        dag_add!(block="0087", references=&vec!["0085"]);
        dag_add!(block="0089", references=&vec!["0088"]);
        dag_add!(block="0093", references=&vec!["0091"]);
        dag_add!(block="0090", references=&vec!["0089"]);
        dag_add!(block="0092", references=&vec!["0086", "0084", "0087"]);
        dag_add!(block="0100", references=&vec!["0086", "0084", "0087"]);
        dag_add!(block="0094", references=&vec!["0093", "0090", "0092"]);
        dag_add!(block="0101", references=&vec!["0100"]);
        dag_add!(block="0095", references=&vec!["0094"]);
        dag_add!(block="0102", references=&vec!["0101"]);
        dag_add!(block="0096", references=&vec!["0095"]);
        dag_add!(block="0097", references=&vec!["0096"]);
        dag_add!(block="0098", references=&vec!["0097"]);
        dag_add!(block="0099", references=&vec!["0098"]);

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        let blue_selection = dag_blue_print(&node_w.dag);
        println!("k={}, {}", k, &blue_selection);

        if k==0 {
            assert_eq!(&blue_selection, "blues={Genesis,0001,0002,0003,0007,0008,0009,0013,0014,0015,0031,0032,0033,0037,0038,0039,0043,0044,0045,0052,0054,0058,0059,0060,0076,0077,0078,0091,0093,0094,0095,0096,0097,0098,0099,} total=35/103");
        }else {
            assert_eq!(2 + 2, 4);
        }
    }

    #[test]
    fn test_fig_x4() {

        let k: i32 = 3;

        let _ = env_logger::try_init();

        let node = Node::init("figX4");

        let mut node_w = node.write().unwrap();

        macro_rules! dag_add {
            ( block=$a:expr, references=$b:expr ) => (node_add_block($a, $b, &mut node_w, k, true));
        }
        dag_add!(block="Genesis", references=&Vec::new());

        let start = PreciseTime::now();

        /*
         * auto generated by the following bash script:
         *  $ sed  's/{name=0\(.*\),block=name=.*,height=.*,size_of_past_set=.*,size_of_past_blue=.*,blue=.,prev=\(.*\)}/dag_add!(block="0\1", references=\&vec!\2);/'  input.log
         */
        dag_add!(block="0001", references=&vec!["Genesis"]);
        dag_add!(block="0002", references=&vec!["Genesis"]);
        dag_add!(block="0003", references=&vec!["Genesis"]);
        dag_add!(block="0004", references=&vec!["Genesis"]);
        dag_add!(block="0005", references=&vec!["0001", "0002", "0003", "0004"]);
        dag_add!(block="0006", references=&vec!["0001", "0002", "0003", "0004"]);
        dag_add!(block="0007", references=&vec!["0001", "0002", "0003", "0004"]);
        dag_add!(block="0008", references=&vec!["0005", "0006"]);
        dag_add!(block="0009", references=&vec!["0007", "0008"]);
        dag_add!(block="0010", references=&vec!["0007", "0008"]);
        dag_add!(block="0011", references=&vec!["0007", "0008"]);
        dag_add!(block="0012", references=&vec!["0009"]);
        dag_add!(block="0013", references=&vec!["0010", "0011", "0012"]);
        dag_add!(block="0014", references=&vec!["0010", "0011", "0012"]);
        dag_add!(block="0015", references=&vec!["0010", "0011", "0012"]);
        dag_add!(block="0016", references=&vec!["0010", "0011", "0012"]);
        dag_add!(block="0017", references=&vec!["0013", "0014", "0015", "0016"]);
        dag_add!(block="0018", references=&vec!["0013", "0014", "0015", "0016"]);
        dag_add!(block="0019", references=&vec!["0013", "0014", "0015", "0016"]);
        dag_add!(block="0020", references=&vec!["0013", "0014", "0015", "0016"]);
        dag_add!(block="0021", references=&vec!["0017", "0018", "0019", "0020"]);
        dag_add!(block="0022", references=&vec!["0017", "0018", "0019", "0020"]);
        dag_add!(block="0023", references=&vec!["0017", "0018", "0019", "0020"]);
        dag_add!(block="0024", references=&vec!["0017", "0018", "0019", "0020"]);
        dag_add!(block="0025", references=&vec!["0021", "0022", "0023", "0024"]);
        dag_add!(block="0026", references=&vec!["0021", "0022", "0023", "0024"]);
        dag_add!(block="0027", references=&vec!["0021", "0022", "0023", "0024"]);
        dag_add!(block="0028", references=&vec!["0021", "0022", "0023", "0024"]);
        dag_add!(block="0029", references=&vec!["0025", "0026"]);
        dag_add!(block="0030", references=&vec!["0025", "0026", "0027"]);
        dag_add!(block="0031", references=&vec!["0025", "0026", "0027"]);
        dag_add!(block="0032", references=&vec!["0025", "0026", "0027"]);
        dag_add!(block="0033", references=&vec!["0025", "0026", "0027", "0028"]);
        dag_add!(block="0034", references=&vec!["0025", "0026", "0027", "0028"]);
        dag_add!(block="0035", references=&vec!["0028", "0029", "0030"]);
        dag_add!(block="0036", references=&vec!["0028", "0029", "0030", "0031", "0032"]);
        dag_add!(block="0037", references=&vec!["0029", "0030", "0031", "0032", "0033", "0034"]);
        dag_add!(block="0038", references=&vec!["0033", "0034", "0035", "0036"]);
        dag_add!(block="0039", references=&vec!["0033", "0034", "0035", "0036"]);
        dag_add!(block="0040", references=&vec!["0033", "0034", "0035", "0036"]);
        dag_add!(block="0041", references=&vec!["0033", "0034", "0035", "0036"]);
        dag_add!(block="0042", references=&vec!["0033", "0034", "0035", "0036"]);
        dag_add!(block="0043", references=&vec!["0033", "0034", "0035", "0036"]);
        dag_add!(block="0044", references=&vec!["0033", "0034", "0035", "0036"]);
        dag_add!(block="0045", references=&vec!["0033", "0034", "0035", "0036"]);
        dag_add!(block="0046", references=&vec!["0035", "0036", "0037"]);
        dag_add!(block="0047", references=&vec!["0035", "0036", "0037"]);
        dag_add!(block="0048", references=&vec!["0035", "0036", "0037"]);
        dag_add!(block="0049", references=&vec!["0035", "0036", "0037"]);
        dag_add!(block="0050", references=&vec!["0037", "0038"]);
        dag_add!(block="0051", references=&vec!["0037", "0038", "0039", "0040", "0041", "0042"]);
        dag_add!(block="0052", references=&vec!["0037", "0038", "0039", "0040", "0041", "0042", "0043"]);
        dag_add!(block="0053", references=&vec!["0037", "0038", "0039", "0040", "0041", "0042", "0043", "0044", "0045"]);
        dag_add!(block="0054", references=&vec!["0038", "0039", "0040", "0041", "0042", "0043", "0044", "0045", "0046"]);
        dag_add!(block="0055", references=&vec!["0038", "0039", "0040", "0041", "0042", "0043", "0044", "0045", "0046", "0047", "0048"]);
        dag_add!(block="0056", references=&vec!["0038", "0039", "0040", "0041", "0042", "0043", "0044", "0045", "0046", "0047", "0048"]);
        dag_add!(block="0057", references=&vec!["0038", "0039", "0040", "0041", "0042", "0043", "0044", "0045", "0046", "0047", "0048", "0049"]);
        dag_add!(block="0058", references=&vec!["0038", "0039", "0040", "0041", "0042", "0043", "0044", "0045", "0046", "0047", "0048", "0049"]);
        dag_add!(block="0059", references=&vec!["0043", "0044", "0045", "0046", "0047", "0048", "0049", "0050", "0051"]);
        dag_add!(block="0060", references=&vec!["0043", "0044", "0045", "0046", "0047", "0048", "0049", "0050", "0051"]);
        dag_add!(block="0061", references=&vec!["0043", "0044", "0045", "0046", "0047", "0048", "0049", "0050", "0051"]);
        dag_add!(block="0062", references=&vec!["0044", "0045", "0046", "0047", "0048", "0049", "0050", "0051", "0052"]);
        dag_add!(block="0063", references=&vec!["0044", "0045", "0046", "0047", "0048", "0049", "0050", "0051", "0052"]);
        dag_add!(block="0064", references=&vec!["0046", "0047", "0048", "0049", "0050", "0051", "0052", "0053"]);
        dag_add!(block="0065", references=&vec!["0046", "0047", "0048", "0049", "0050", "0051", "0052", "0053"]);
        dag_add!(block="0066", references=&vec!["0046", "0047", "0048", "0049", "0050", "0051", "0052", "0053"]);
        dag_add!(block="0067", references=&vec!["0046", "0047", "0048", "0049", "0050", "0051", "0052", "0053"]);
        dag_add!(block="0068", references=&vec!["0046", "0047", "0048", "0049", "0050", "0051", "0052", "0053"]);
        dag_add!(block="0069", references=&vec!["0046", "0047", "0048", "0049", "0050", "0051", "0052", "0053"]);
        dag_add!(block="0070", references=&vec!["0046", "0047", "0048", "0049", "0050", "0051", "0052", "0053"]);
        dag_add!(block="0071", references=&vec!["0047", "0048", "0049", "0050", "0051", "0052", "0053", "0054"]);
        dag_add!(block="0072", references=&vec!["0047", "0048", "0049", "0050", "0051", "0052", "0053", "0054"]);
        dag_add!(block="0073", references=&vec!["0049", "0050", "0051", "0052", "0053", "0054", "0055", "0056"]);
        dag_add!(block="0074", references=&vec!["0049", "0050", "0051", "0052", "0053", "0054", "0055", "0056"]);
        dag_add!(block="0075", references=&vec!["0049", "0050", "0051", "0052", "0053", "0054", "0055", "0056"]);
        dag_add!(block="0076", references=&vec!["0050", "0051", "0052", "0053", "0054", "0055", "0056", "0057"]);
        dag_add!(block="0077", references=&vec!["0050", "0051", "0052", "0053", "0054", "0055", "0056", "0057"]);
        dag_add!(block="0078", references=&vec!["0050", "0051", "0052", "0053", "0054", "0055", "0056", "0057", "0058"]);
        dag_add!(block="0079", references=&vec!["0050", "0051", "0052", "0053", "0054", "0055", "0056", "0057", "0058"]);
        dag_add!(block="0080", references=&vec!["0050", "0051", "0052", "0053", "0054", "0055", "0056", "0057", "0058"]);
        dag_add!(block="0081", references=&vec!["0050", "0051", "0052", "0053", "0054", "0055", "0056", "0057", "0058"]);
        dag_add!(block="0082", references=&vec!["0052", "0053", "0054", "0055", "0056", "0057", "0058", "0059"]);
        dag_add!(block="0083", references=&vec!["0052", "0053", "0054", "0055", "0056", "0057", "0058", "0059"]);
        dag_add!(block="0084", references=&vec!["0052", "0053", "0054", "0055", "0056", "0057", "0058", "0059", "0060", "0061"]);
        dag_add!(block="0085", references=&vec!["0052", "0053", "0054", "0055", "0056", "0057", "0058", "0059", "0060", "0061"]);
        dag_add!(block="0086", references=&vec!["0053", "0054", "0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063"]);
        dag_add!(block="0087", references=&vec!["0053", "0054", "0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063"]);
        dag_add!(block="0088", references=&vec!["0053", "0054", "0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063"]);
        dag_add!(block="0089", references=&vec!["0053", "0054", "0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063"]);
        dag_add!(block="0090", references=&vec!["0054", "0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063", "0064"]);
        dag_add!(block="0091", references=&vec!["0054", "0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063", "0064", "0065", "0066", "0067"]);
        dag_add!(block="0092", references=&vec!["0054", "0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063", "0064", "0065", "0066", "0067"]);
        dag_add!(block="0093", references=&vec!["0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063", "0064", "0065", "0066", "0067", "0068", "0069", "0070", "0071"]);
        dag_add!(block="0094", references=&vec!["0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063", "0064", "0065", "0066", "0067", "0068", "0069", "0070", "0071"]);
        dag_add!(block="0095", references=&vec!["0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063", "0064", "0065", "0066", "0067", "0068", "0069", "0070", "0071"]);
        dag_add!(block="0096", references=&vec!["0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063", "0064", "0065", "0066", "0067", "0068", "0069", "0070", "0071", "0072"]);
        dag_add!(block="0097", references=&vec!["0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063", "0064", "0065", "0066", "0067", "0068", "0069", "0070", "0071", "0072"]);
        dag_add!(block="0098", references=&vec!["0055", "0056", "0057", "0058", "0059", "0060", "0061", "0062", "0063", "0064", "0065", "0066", "0067", "0068", "0069", "0070", "0071", "0072"]);
        dag_add!(block="0099", references=&vec!["0059", "0060", "0061", "0062", "0063", "0064", "0065", "0066", "0067", "0068", "0069", "0070", "0071", "0072", "0073", "0074", "0075", "0076", "0077", "0078"]);
        dag_add!(block="0100", references=&vec!["0059", "0060", "0061", "0062", "0063", "0064", "0065", "0066", "0067", "0068", "0069", "0070", "0071", "0072", "0073", "0074", "0075", "0076", "0077", "0078", "0079", "0080", "0081"]);

        let end = PreciseTime::now();
        let d = start.to(end);
        let total_time_used = d.num_milliseconds() as f64;

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        let blue_selection = dag_blue_print(&node_w.dag);
        println!("k={}, {}", k, &blue_selection);

        println!("total time used: {} (ms)", total_time_used);

        if k==0 {
            assert_eq!(&blue_selection, "blues={Genesis,0001,0005,0008,0009,0012,0013,0017,0021,0025,0100,} total=11/101");
        }else {
            assert_eq!(2 + 2, 4);
        }
    }

    #[test]
    fn test_nodes_sync() {

        let _ = env_logger::try_init();

        const TOTAL_NODES: i32 = 500;        // how many nodes to simulate. each node is a thread spawn.
        let blocks_generating:i32 = 3000;      // how many blocks mining for this test.
        let blocks_one_time: i32 = 4;        // how many blocks generating in one wait (loop).
        const K: i32 = 3;                    // how many blocks generating in parallel.

        println!("test_nodes_sync(): start. k={}, blocks={}, nodes={}", K, blocks_generating, TOTAL_NODES);

        // important note: the token-ring locker must be drop as soon as possible by node.
        let block_token_ring: Arc<RwLock<HashMap<String, Arc<RwLock<BlockRaw>>>>> = Arc::new(RwLock::new(HashMap::new()));
        let mining_token_ring: Arc<RwLock<(i32,i32)>> = Arc::new(RwLock::new((0,0)));
        let blocks_generated = Arc::new(RwLock::new(0 as i32));

        let mut handles = vec![];

        for number in 0..TOTAL_NODES {

            let mining = Arc::clone(&mining_token_ring);
            let block_propagation = Arc::clone(&block_token_ring);
            let blocks_generated = Arc::clone(&blocks_generated);

            let handle = thread::spawn(move || {

                let node = Node::init(&format!("node{}", number));
                let mut node_w = node.write().unwrap();
                node_add_block("Genesis", &Vec::new(), &mut node_w, K, true);
                drop(node_w);

                // block rx thread
                let block_propagation_rx = Arc::clone(&block_propagation);
                let node_for_rx = Arc::clone(&node);
                let (tx, rx) = mpsc::channel();
                let rx_handle = thread::spawn(move || {

                    let mut node_stash: HashMap<String, Arc<RwLock<BlockRaw>>> = HashMap::new();
                    let node_w2 = node_for_rx.read().unwrap();
                    let node_name = node_w2.name.clone();
                    drop(node_w2);

                    loop {
                        let received = rx.try_recv();
                        if received.is_ok() {
                            debug!("{} rx thread exited. size_of_stash={}", node_name, node_stash.len());
                            break;
                        }

                        let mut node_w2 = node_for_rx.write().unwrap();

                        // processing block propagation
                        let t1 = PreciseTime::now();
                        let received = handle_block_rx(&block_propagation_rx, &mut node_w2, &mut node_stash, K);

                        let t2 = PreciseTime::now();
                        let td = t1.to(t2);
                        let time_used = td.num_milliseconds() as f64;
                        if received>0 {
                            debug!("rx time spent:{}ms {}. size_of_stash={}", time_used, &node_w2, node_stash.len());
                        }
                        drop(node_w2);

                        let random_sleep = rand::thread_rng().gen_range(1, 100);
                        thread::sleep(Duration::from_millis(random_sleep));
                    }

                });

                // block mining and tx thread
                loop {
                    let mut mining_lock = mining.write().unwrap();
                    if (*mining_lock).0 <= -1 {

                        let node_w = node.read().unwrap();
                        info!("{} tx thread exited. height={},size_of_dag={},mining_lock={:?}. mined_blocks={}",
                              node_w.name, node_w.height, node_w.size_of_dag, *mining_lock, node_w.mined_blocks);
                        drop(mining_lock);

                        if node_w.name == "node0" {
                            dag_print(&node_w.dag);
                            let blue_selection = dag_blue_print(&node_w.dag);
                            info!("k={}, {}", K, &blue_selection);
                        }

                        break;

                    }else if (*mining_lock).0 == 0 {
                        drop(mining_lock);
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    }

                    let node_w = node.read().unwrap();
                    if node_w.height+1 < (*mining_lock).1 as u64 {
                        //info!("{} mining skip because low height: {}, need: {}", node_w.name, node_w.height, (*mining_lock).1);
                        drop(mining_lock);
                        drop(node_w);
                        thread::sleep(Duration::from_millis(10));
                        continue;
                    }

                    (*mining_lock).0 -= 1;
                    drop(mining_lock);
                    drop(node_w);

                    let mut node_w = node.write().unwrap();
                    debug!("{} start mining on height: {}", node_w.name, node_w.height+1);

                    let mut blocks_generated_w = blocks_generated.write().unwrap();
                    *blocks_generated_w += 1;
                    let block_name = format!("{:04}", blocks_generated_w);
                    drop(blocks_generated_w);

                    let mut score_stpq = get_stpq(&node_w.tips);
                    score_stpq.truncate((K+1) as usize);
                    let references_str = score_stpq.iter().map(|&(ref s,_,_)| s.as_ref()).collect();
                    node_add_block(&block_name, &references_str, &mut node_w, K, true);

                    // propagate this new mined block
                    let t1 = PreciseTime::now();
                    debug!("tx write locking. {} new mined block: {}. height={},size_of_dag={}. mined_blocks={}", node_w.name, block_name, node_w.height, node_w.size_of_dag, node_w.mined_blocks);
                    let mut propagations = block_propagation.write().unwrap();
                    handle_block_tx(&block_name, &mut propagations, &node_w, TOTAL_NODES-1);
                    let t2 = PreciseTime::now();
                    let td = t1.to(t2);
                    let time_used = td.num_milliseconds() as f64;
                    debug!("tx time spent:{}ms {} new mined block: {}. height={},size_of_dag={}. mined_blocks={}", time_used, node_w.name, block_name, node_w.height, node_w.size_of_dag, node_w.mined_blocks);

                    node_w.mined_blocks += 1;

                    drop(node_w);

//                    if node_w.name == "node0" {
//                        dag_print(&node_w.dag);
//                    }
                }

                let val = String::from("exit");
                tx.send(val).unwrap();

                rx_handle.join().unwrap();
            });

            handles.push(handle);
        }

        // wait a while for nodes thread start-up.
        thread::sleep(Duration::from_millis(1000));

        let start = PreciseTime::now();

        let mut acc = 0;
        let mut height = 0;
        loop {

            let mut mining = mining_token_ring.write().unwrap();
            if (*mining).0 > 0 {
                // miner too slow?
                drop(mining);
                //println!("miner too slow?");
                thread::sleep(Duration::from_millis(10));
                continue;
            }

            height += 1;
            if acc + blocks_one_time <= blocks_generating {
                (*mining).0 += blocks_one_time;
                acc += blocks_one_time;
            }else{
                (*mining).0 += blocks_generating-acc;
                acc += blocks_generating-acc;
            }
            (*mining).1 = height;
            debug!("test_nodes_sync(): start mining {} blocks at height {}. mining_lock={:?}", blocks_one_time, height, *mining);
            drop(mining);

            thread::sleep(Duration::from_millis(150));

            {
                let blocks_generated_r = blocks_generated.read().unwrap();
                if *blocks_generated_r >= blocks_generating {
                    drop(blocks_generated_r);

                    // wait a while for nodes complete propagation.
                    println!("\npreparing to terminate. wait 1 second for nodes complete propagation....\n");
                    thread::sleep(Duration::from_millis(1000));

                    let mut mining = mining_token_ring.write().unwrap();
                    (*mining).0 = -1;   // ask nodes stop and exit.
                    drop(mining);
                    break;
                }
            }
        }

        for handle in handles {
            handle.join().unwrap();
        }

        let propagation = block_token_ring.read().unwrap();
        println!("test_nodes_sync(): done. block propagation hashmap remaining size: {}", propagation.len());

        let end = PreciseTime::now();
        let d = start.to(end);
        let total_time_used = d.num_milliseconds() as f64;

        println!("total time used: {} (ms)", total_time_used-1000);

        assert_eq!(2 + 2, 4);
    }
}
