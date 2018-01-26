#!/bin/bash
set -e -u -x

NAMES="a b c d e f g h i j k l m n o p q r s t u v w x y z ryan rpb foolproof foolproof.io"
DEPTHS=$(seq 1 16)

CMD="./target/release/treefunc --seed={1} --depth={2} --output=images/{1}-{2}.png"
parallel --jobs=16 "echo $CMD && $CMD" ::: $NAMES ::: $DEPTHS
