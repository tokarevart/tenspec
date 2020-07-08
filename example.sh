#!/bin/bash

cargo run -- tess -n 20 --dims 25 8 1 6 2 6 &&
cargo run -- reg --fmax 20 --sel 3 --mloop 5 &&
cargo run -- mesh --cl 3 -o tenspec.msh