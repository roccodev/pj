// Copyright (C) 2019 RoccoDev
// 
// This file is part of pj.
// 
// pj is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
// 
// pj is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
// 
// You should have received a copy of the GNU General Public License
// along with pj.  If not, see <http://www.gnu.org/licenses/>.

extern crate termion;
extern crate serde_json;

pub mod json;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{color, style};
use std::io::{self, BufRead, Write, stdout, stdin};
fn main() {

    let stdin = stdin();
    // let mut stdout = stdout().into_raw_mode().unwrap();
    let json = stdin.lock().lines().last().unwrap().unwrap();

    json::parse(json);
}