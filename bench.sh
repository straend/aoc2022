#!/bin/env bash
day=$1
cargo build --release

input=()
p1=()
p2=()
executions=100
for i in `seq ${executions}`; do
t=$(target/release/main "$day" bench)
IFS="\t" set $t
input+=($1)
p1+=($2)
p2+=($3)
done

sum_input=$((${input[@]/%/+}0))
sum_p1=$((${p1[@]/%/+}0))
sum_p2=$((${p2[@]/%/+}0))

one_i=`expr $sum_input / $executions`
one_p1=`expr $sum_p1 / $executions`
one_p2=`expr $sum_p2 / $executions`

echo "$one_i    $one_p1    $one_p2"
