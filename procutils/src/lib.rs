// Copyright (c) <2024>, <🌟夕元><🌟VEA>
// All Rights Reserved
// 
// This file is part of <LinuxProcessTrace> distributed under the BSD 3-Clause License. 
// See the LICENSE file at the root directory of this project for more details.

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub mod file_utils;
pub mod proc_analysis;
