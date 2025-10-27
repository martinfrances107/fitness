#!/bin/bash
RUST_LOG=info cargo run < ./Fitness.md > interval.dat
gnuplot interval.gnuplot > fitness.png
