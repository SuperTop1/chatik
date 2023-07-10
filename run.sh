#!/bin/bash
set -e

cargo b
sudo setcap CAP_NET_ADMIN=eip ./target/debug/project_starter
./target/debug/project_starter &
pid=$!

#sudo ip a add 192.168.0.127/24 dev mytun
sudo ip a add 10.0.1.5/24 dev tun0
sudo ip link set up dev tun0

trap "kill $pid" INT TERM
wait $pid
