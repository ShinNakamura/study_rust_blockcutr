#! /bin/bash
unalias -a

bin=cutbystr
expect="aaa"
act="$(echo "aaa | oooo" | cargo run --bin $bin -- "|" 0)"
if test "_$expect" = "_$act"
then
    echo "$bin pass"
else
    echo "$bin expected '$expect' but got '$act'"
fi

bin=cutbyregex
expect="aaa"
act="$(echo " xxx <br />    aaa   <BR>oooo" | cargo run --bin $bin -- '(?i)<br[^>]*?>' 1)"
if test "_$expect" = "_$act"
then
    echo "$bin pass"
else
    echo "$bin expected '$expect' but got '$act'"
fi

