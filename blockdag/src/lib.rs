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

    use std::collections::HashMap;
    use std::sync::{Arc,RwLock};
    use blockdag::{Block,Node};
    use blockdag::{node_add_block,dag_add_block,dag_print};

    #[test]
    fn test_fig3() {

        let node = Node::init("fig3");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w);

        node_add_block("B", &vec!["Genesis"], &mut node_w);
        node_add_block("C", &vec!["Genesis"], &mut node_w);
        node_add_block("D", &vec!["Genesis"], &mut node_w);
        node_add_block("E", &vec!["Genesis"], &mut node_w);

        node_add_block("F", &vec!["B","C"], &mut node_w);
        node_add_block("H", &vec!["C","D","E"], &mut node_w);
        node_add_block("I", &vec!["E"], &mut node_w);

        node_add_block("J", &vec!["F","H"], &mut node_w);
        node_add_block("K", &vec!["B","H","I"], &mut node_w);
        node_add_block("L", &vec!["D","I"], &mut node_w);
        node_add_block("M", &vec!["F","K"], &mut node_w);

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn test_fig4() {

        let node = Node::init("fig4");

        let mut node_w = node.write().unwrap();

        node_add_block("Genesis", &Vec::new(), &mut node_w);

        node_add_block("B", &vec!["Genesis"], &mut node_w);
        node_add_block("C", &vec!["Genesis"], &mut node_w);
        node_add_block("D", &vec!["Genesis"], &mut node_w);
        node_add_block("E", &vec!["Genesis"], &mut node_w);

        node_add_block("F", &vec!["B","C"], &mut node_w);
        node_add_block("H", &vec!["E"], &mut node_w);
        node_add_block("I", &vec!["C","D"], &mut node_w);

        node_add_block("J", &vec!["F","D"], &mut node_w);
        node_add_block("K", &vec!["J","I","E"], &mut node_w);
        node_add_block("L", &vec!["F"], &mut node_w);
        node_add_block("N", &vec!["D","H"], &mut node_w);

        node_add_block("M", &vec!["L","K"], &mut node_w);
        node_add_block("O", &vec!["K"], &mut node_w);
        node_add_block("P", &vec!["K"], &mut node_w);
        node_add_block("Q", &vec!["N"], &mut node_w);

        node_add_block("R", &vec!["O","P","N"], &mut node_w);

        node_add_block("S", &vec!["Q"], &mut node_w);
        node_add_block("T", &vec!["S"], &mut node_w);
        node_add_block("U", &vec!["T"], &mut node_w);

        println!("{}", &node_w);

        dag_print(&node_w.dag);

        assert_eq!(2 + 2, 4);
    }
}
