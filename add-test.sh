#!/bin/env bash

cat << EOF >> "src/problems.rs"
mod ${1//-/_};
EOF

cat << EOF > "src/problems/${1//-/_}.rs"
#[test]
fn run_problem() {
	assert!(true);
}

EOF
