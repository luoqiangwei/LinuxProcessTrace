// Copyright (c) <2024>, <🌟夕元><🌟VEA>
// All Rights Reserved
// 
// This file is part of <LinuxProcessTrace> distributed under the BSD 3-Clause License. 
// See the LICENSE file at the root directory of this project for more details.

pub use procutils::*;

fn main() {
    // Modify this... To trace process
    let monitor_list: Vec<&str> = vec!["init"];
    procutils::proc_analysis::trace_process(60, 10, &monitor_list);
}
