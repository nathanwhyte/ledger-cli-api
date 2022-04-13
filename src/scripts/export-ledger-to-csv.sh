#!/usr/bin/env bash

# ledger-cli-api project home dir
PROJECT_HOME=$(pwd)

# my default location for ledger files
# in the future, users will be able to set their
# default ledger directory in a config file
LEDGER_HOME="/home/natew/Dropbox/Finance"

# go to default ledger dir. to execute commands
cd $LEDGER_HOME

# convert all .ledger and .dat files to csv
LEDGER_FILES=$(ls *.ledger *.dat)
for LEDGER_FILE in $LEDGER_FILES; do

    # remove file extension, grab base file name
    LEDGER_FILE_NAME=""
    if [[ "${LEDGER_FILE: -4}" == ".dat" ]]; then
        LEDGER_FILE_NAME=$(basename $LEDGER_FILE .dat)
    else
        LEDGER_FILE_NAME=$(basename $LEDGER_FILE .ledger)
    fi

    # export csv data to project directory
    ledger -f $LEDGER_FILE csv > "$PROJECT_HOME/data/ledger-csv/$LEDGER_FILE_NAME.csv"
done

LOG_FILE_PATH="$PROJECT_HOME/data/logs/processed-ledger-files.txt"
if [[ -f "$LOG_FILE_PATH" ]]; then
    echo "hi there"
else
    echo "$LEDGER_FILES" > "$LOG_FILE_PATH"
fi
