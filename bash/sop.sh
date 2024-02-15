#!/bin/bash

readarray -t A < <(( seq -$1 $1 | shuf))
N=${#A[@]}
for (( i = 0; i <= N ; i++ )); do
for (( q = 0; q < i ; q++ )); do
S=$(( ${A[$i]} + ${A[$q]} ))
[[ $S == $2 ]] && echo "$q $i: ${A[$i]}+${A[$q]}=$2" && exit
done; done
