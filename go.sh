#!/bin/bash
cat Fitness.md | RUST_LOG=info cargo run  | gnuplot interval.gnuplot > fitness.png
