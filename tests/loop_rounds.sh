#!/bin/bash

for ((i=0; i<20; i++))
do
    anchor run round
    sleep 2
    anchor run multibet
    sleep 2
    anchor run play
    sleep 2
    anchor run assess
    sleep 2
    anchor run payout
    sleep 2
    anchor run close

done