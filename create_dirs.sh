#!/bin/bash

main() {
    for i in {1..9}
    do
        basename="$1/Day_0$i"
        mkdir $basename
        touch "$basename/input.txt"
        # for j in a b
        # do
        #     basename="$1/Day_0$i-$j"
        #     mkdir $basename
        #     touch "$basename/input.txt"
        # done
    done

    for i in {10..25}
    do
        basename="$1/Day_$i"
        mkdir $basename
        touch "$basename/input.txt"
        # for j in a b
        # do
        #     basename="$1/Day_$i-$j"
        #     mkdir $basename
        #     touch "$basename/input.txt"
        # done
    done

}

main $1