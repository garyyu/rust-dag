
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
use std::rc::Rc;
use std::cell::RefCell;

/// Structure providing fast access to block data.
///
#[derive(Debug)]
pub struct Block{
    pub name: String,
    pub height: u64,
    pub size_of_past_set: u64,
    pub prev: HashMap<String, Rc<RefCell<Block>>>,
    pub next: HashMap<String, Rc<RefCell<Block>>>,
}