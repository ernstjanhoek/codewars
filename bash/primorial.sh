#!/bin/bash

# primorial.sh vermenigvuldigt de eerste n ($1) priemgetallen met elkaar en geeft dit als output.
# Er is een rare bug als er te veel priemgetallen gevraagd worden is de output fout en soms zelfs negatief!

arrprim=()
n=2	
function primfind {
n=$1
	for ((i=2;i<=n/2;i++));do
		if [[ $((n%i)) == 0 ]]
		then
		gp="true"
		break
		fi
	done
if [[ $gp == "true" ]]
then
	gp=""
else
	arrprim+=( $n ) 
fi
}

function output {
out=${arrprim[@]}
out=${out# }
out=${out% }
out=${out// /*}
out=$(( out ))
echo "$out"
}

while [ ${#arrprim[@]} -lt "$1" ] 
do
primfind $n
n=$((n + 1))
done

echo ${arrprim[-1]}
# output