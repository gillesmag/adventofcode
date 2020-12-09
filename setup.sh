#!/usr/bin/env bash

set -euo pipefail

SCRIPTPATH="$( cd "$(dirname "$0")" > /dev/null 2>&1 ; pwd -P )"

DAY="$(date +'%d')"
MONTH="$(date +'%m')"
YEAR="$(date +'%Y')"

if [ "$MONTH" -ne "12" ] || [ "$YEAR" -ne "2020" ] || [ "$DAY" -gt "25" ]; then
	echo "This script only works from the 1st to 25th December 2020, aborting..."
	exit
fi

NEW_DAY_PATH="$SCRIPTPATH/day_$DAY"

if [ ! -d "$NEW_DAY_PATH" ]; then
	mkdir "$NEW_DAY_PATH" && \
	rm -f today && \
	ln -s "$NEW_DAY_PATH" today && \
	touch "$NEW_DAY_PATH"/{main.jl,{test,input}.txt}
fi
