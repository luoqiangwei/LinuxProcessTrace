// Copyright (c) 2024, 🌟夕元 & 🌟VEA
// All Rights Reserved
// 
// This file is part of LinuxProcessTrace distributed under the BSD 3-Clause License. 
// See the LICENSE file at the root directory of this project for more details.

use std::env;
use std::process::exit;

pub use procutils::*;

fn main() {
    // Get command line parameters
    let args: Vec<String> = env::args().collect();

    // Check the number of command line parameters
    if args.len() < 3 {
        eprintln!("Usage: {} <monitor_time_in_seconds> <check_interval> [[monitor_list_item] [...]]", args[0]);
        exit(1);
    }

    // Analyze the monitoring time and inspection interval, which should be integer values
    let monitor_time: i64 = args[1]
        .parse()
        .expect("Monitor time should be an unsigned integer");
    let check_interval: i64 = args[2]
        .parse()
        .expect("Check interval should be an unsigned integer");

    // Starting from the third parameter, the remaining parameters are treated as monitor_list
    let monitor_list: Vec<&str> = args[3..]
        .iter()
        .map(String::as_str)
        .collect();

    // Call the trace_process function and pass in the value obtained from the command line
    procutils::proc_analysis::trace_process(monitor_time, check_interval, &monitor_list);
}
