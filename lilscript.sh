#!/bin/sh

cargo run
julia misc/join.jl
cat data/bov_joined.csv