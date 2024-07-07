#!/bin/bash

for ((i=0; i<100; i++))
do
    anchor run start
    sleep 2
    anchor run finish

done