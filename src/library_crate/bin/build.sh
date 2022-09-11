#!/usr/bin/bash

rustc client.rs --extern greeting=../lib/libgreeting.rlib --edition=2018