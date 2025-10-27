#!/bin/bash
RUST_LOG=info cargo run < ./Fitness.md > interval.dat
gnuplot -p interval.gnuplot