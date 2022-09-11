#!/usr/bin/bash

rm -rf string_sum
mkdir string_sum
cd string_sum
python3 -m venv .env
source .env/bin/activate
pip3 install maturin

maturin init

maturin develop
