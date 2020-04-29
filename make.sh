#!/bin/bash

set -eu

if [ $# -ne 1 ]; then
  echo "Specify one argument."
  exit 1
fi

if [[ $(which procon-template 2>&1 > /dev/null; echo $?) -ne 0 ]]; then
  echo "Unable to find the template extraction script."
  exit 1
fi

cargo new $1 --vcs git
cd $1

mkdir src/bin

for f in a.rs b.rs c.rs d.rs e.rs f.rs
do
  procon-template > src/bin/$f
done

cargo check

echo "Finished!"
