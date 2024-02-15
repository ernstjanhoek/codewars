#!/bin/bash

c="$1"
function main {
len=$[${#c}-1]
for (( i=0 ; i < len  ; i++ )); do
	n=$(( i + 1))
	if [[ ${c:$i:2} == "()" ]]
	then
		c="${c:0:$i}${c:$i+2}"
		main $c
	fi
done
}

main $1

if [[ $c == "" ]]
	then
	echo "True"
	else
	echo "False"
fi