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

//use std::collections::HashMap;
//use std::collections::hash_map::Entry;
//use std::sync::{Arc,RwLock};
//
//use blockdag::{Block,Node,MaxMin,append_maps};
//
//// Function providing blue hourglass blocks calculation.
////
////     input 'block': add a new blue block
////
//pub fn blue_hourglass_update(block: &Block, node: &mut Node){
//
////    let dag = &node.dag;
////    let hourglass = &node.hourglass;
//}
//
//// Function looking for nearest hourglass pair.
////
//// For example: Vec![ (2,4), (4,5), (7,8), (9,12), (15,19), (20,21)   ]
////
//pub fn get_nearest_hourglass(height: u64, hourglass: &Vec<(u64,u64)>) -> (u64,u64){
//
//    let mut nearest:(u64,u64) = (0,0);
//
//    //todo: not efficient to use sequential loop for searching, to be improved.
//    for &(low,high) in hourglass {
//        if height>=high {
//           break;
//        }
//
//        nearest = (low,high);
//    }
//
//    return nearest;
//}