#!/usr/bin/env bash

set -e

echo "------------------------------------------------------"
echo "Initializing Minerva instance in '/minerva-instance'.."
echo "------------------------------------------------------"

if [ ! -d "/minerva-instance" ]; then
    echo "ERROR: Directory /minerva-instance does not exist."
    exit 1
fi

export PGSSLMODE=disable

minerva -V
minerva initialize --create-schema /minerva-instance

echo "---------------------------------------"
echo "Finished initializing Minerva instance "
echo "---------------------------------------"
