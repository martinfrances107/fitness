#!/bin/bash
rm fitness.png
cat Fitness.md | RUST_LOG=info cargo run --release | gnuplot fitness.gnuplot > fitness.png
