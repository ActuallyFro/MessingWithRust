#!/bin/bash

#ChatGPT:
function random_number {
  local min=$1
  local max=$2

  # Check if min is greater than max
  if [ $min -gt $max ]; then
    local temp=$min
    min=$max
    max=$temp
  fi

  # Generate a random number between min and max
  echo $(( (RANDOM % (max - min + 1)) + min ))
}

#ChatGPT:
function read_key {
  local key
# -s suppresses output, -n 1 reads 1 character, -t 1 waits 1 second
  read -s -n 1 -t 1 key

  echo "$key"
}


filename="current-time.txt"

run_loop=true
echo "[NOTICE] Running time-hack! Press Q to quit."
while [[ $run_loop == "true" ]]; do
  random_num=$(random_number 1 5)
  current_time=$(date -Iseconds)

  #I. Update File
  echo "$current_time" > "$filename"

  #II. Update Terminal
  # Clear the current line on stdout
  printf "\033[2K\r"
  echo -n "Last ran at: $current_time (Sleeping for $random_num secs)"

  key=$(read_key)
  # "^[", "\033", nor "\e" is working for ESC
  if [[ $key = "q" ]] || [[ $key = "Q" ]]; then
    run_loop=false
    echo ""
    echo "[NOTICE] Q hit, stopping time-hack!"
    # break
  fi
done

