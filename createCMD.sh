#!/bin/bash

echo "pub fn cmd_$1(args: &Vec<String>) -> i8 {return 0;}" > src/commands/$1.rs
