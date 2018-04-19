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

extern crate core;

pub mod blockdag;

#[cfg(test)]
mod tests {

    use std::collections::HashMap;
    use std::sync::{Arc,RwLock};
    use blockdag::Block;
    use blockdag::{dag_add_block,dag_print};

    #[test]
    fn test_fig3() {

        let mut dag: HashMap<String, Arc<RwLock<Block>>> = HashMap::new();

        dag_add_block("Genesis", &Vec::new(), &mut dag);

        dag_add_block("B", &vec!["Genesis"], &mut dag);
        dag_add_block("C", &vec!["Genesis"], &mut dag);
        dag_add_block("D", &vec!["Genesis"], &mut dag);
        dag_add_block("E", &vec!["Genesis"], &mut dag);

        dag_add_block("F", &vec!["B","C"], &mut dag);
        dag_add_block("H", &vec!["C","D","E"], &mut dag);
        dag_add_block("I", &vec!["E"], &mut dag);

        dag_add_block("J", &vec!["F","H"], &mut dag);
        dag_add_block("K", &vec!["B","H","I"], &mut dag);
        dag_add_block("L", &vec!["D","I"], &mut dag);
        dag_add_block("M", &vec!["F","K"], &mut dag);

        dag_print(&dag);

        assert_eq!(2 + 2, 4);
    }
}
