#!/bin/bash

set -e

TEST_CASES=(
  'set foo bar'
  'get foo'
  'set foo barr'
  'set foo barrr'
  'get foo'
  'get foo'
)

set -x

echo "Running test"

# run the server
./target/debug/server &
SERVER_PID=$!
echo ":; started server."

# kill the server process when the script exits.
trap 'echo ":; killing server"; kill ${SERVER_PID};' EXIT

sleep 1

for CASE in "${TEST_CASES[@]}"; do
  target/debug/cli ${CASE[*]} &
  pids+=($!)
done

wait "${pids[@]}"

echo ":; killing server"; kill ${SERVER_PID};
