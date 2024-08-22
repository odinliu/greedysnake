#!/bin/bash

arrline=()
for i in `seq 0 15`; do
    arr=()
    for j in `seq 0 15`; do
        idx=$(($i*16+$j))
        arr[$j]="font-$idx.png"
    done
    magick convert +append ${arr[*]} line-$i.png
    arrline[$i]="line-$i.png"
done
magick convert -append ${arrline[*]} snake.png
