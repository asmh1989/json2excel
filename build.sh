#! /bin/bash

cargo build --release

sudo cp target/release/json2excel /home/docker/baidu

sudo cp target/release/json2excel /usr/bin 