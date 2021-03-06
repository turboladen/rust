// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![allow(unused_must_use)]
#![allow(unused_mut)]
// ignore-emscripten no threads support

use std::thread;

pub fn main() {
    let mut result = thread::spawn(child);
    println!("1");
    thread::yield_now();
    println!("2");
    thread::yield_now();
    println!("3");
    result.join();
}

fn child() {
    println!("4"); thread::yield_now(); println!("5"); thread::yield_now(); println!("6");
}
