#!/bin/bash

c="$1"
function main {
len=$[${#c}-1]
for (( i=0 ; i < len  ; i++ )); do
n=$[$i+1]
if [ ${c:$i:1} = "N" ]
then
  if [ ${c:$n:1} = "S" ]
  then
    c="${c:0:$i}${c:$i+2}"
    main $c
  fi
elif [ ${c:$i:1} = "S" ]
then
  if [ ${c:$n:1} = "N" ]
  then
    c="${c:0:$i}${c:$i+2}"
    main $c
  fi
elif [ ${c:$i:1} = "E" ]
then
  if [ ${c:$n:1} = "W" ]
  then
    c="${c:0:$i}${c:$i+2}"
    main $c
  fi
elif [ ${c:$i:1} = "W" ]
then
  if [ ${c:$n:1} = "E" ]
  then
    c="${c:0:$i}${c:$i+2}"
    main $c
  fi
fi
done
}

main $1
echo $c