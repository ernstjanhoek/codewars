#!/bin/bash

v=$1
mu=$2
g=9.81
scale=4
bc <<< "(v / 3.6) + (( (v / 3.6) * (v / 3.6) / ( 2 * mu * g ))"