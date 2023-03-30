#!/bin/env bash

cargo test problems::${1//-/_}::run_problem -- --nocapture
