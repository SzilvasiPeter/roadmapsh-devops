#!/bin/bash
echo 2
n=3
while :; do
  is_prime=1
  for ((i=3; i*i<=n; i+=2)); do
    if ((n % i == 0)); then
      is_prime=0
      break
    fi
  done
  ((is_prime)) && echo $n
  ((n+=2))
done
