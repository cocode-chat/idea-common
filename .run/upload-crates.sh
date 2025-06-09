#!/bin/bash

echo '-----> 1. cargo pkg'
cargo package

echo '-----> 2. cargo publish'
cargo publish --registry crates-io