#! /bin/bash
unalias -a

test -d ./sample || mkdir ./sample
cargo run -- tests/textdata/input.utf8.lf.txt >sample/out.txt
diff --binary --color tests/textdata/expected.txt sample/out.txt
