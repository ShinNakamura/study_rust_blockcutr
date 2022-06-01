#! /bin/bash
unalias -a

expect="aaa"
act="$(echo "aaa | oooo" | cargo run --bin cutbystr -- "|" 0)"
if test "_$expect" = "_$act"
then
    echo "pass"
else
    echo "expected '$expect' but got '$act'"
fi
