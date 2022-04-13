#!/usr/bin/env bash

# ledger-cli-api project home dir
PROJECT_HOME=$(pwd)

# my default location for ledger files
# in the future, users will be able to set their
# default ledger directory in a config file
LEDGER_HOME="/home/natew/Dropbox/Finance/"

# store the location of the log file
LOG_FILE="$PROJECT_HOME/data/logs/processed-ledger-files.txt"

# grab any .ledger or .dat files
LEDGER_FILES=$(ls "$LEDGER_HOME" | grep ".ledger\|.dat")

# convert all .ledger and .dat files to csv
NEW_FILE_COUNTER=0
for LEDGER_FILE in $LEDGER_FILES; do

    # remove file extension, grab base file name
    LEDGER_FILE_NAME=""
    if [[ "${LEDGER_FILE#*.}" == "dat" ]]; then
        LEDGER_FILE_NAME=$(basename $LEDGER_FILE .dat)
    elif [[ "${LEDGER_FILE#*.}" == "ledger" ]]; then
        LEDGER_FILE_NAME=$(basename $LEDGER_FILE .ledger)
    else
        echo "extension not supported: ${LEDGER_FILE#*.}"
    fi

    if [[ -f "$LOG_FILE" ]]; then
        if grep -Fxq "$LEDGER_FILE" "$LOG_FILE"; then
            echo "Already processed $LEDGER_FILE"
        else
            printf "Adding data from $LEDGER_FILE ..."
            ledger -f "$LEDGER_HOME/$LEDGER_FILE" csv > "$PROJECT_HOME/data/ledger-csv/$LEDGER_FILE_NAME.csv"
            echo "$LEDGER_FILE" >> $LOG_FILE
            ((NEW_FILE_COUNTER++))
            printf "\rAdding data from $LEDGER_FILE ... Done\n"
        fi
    else
        echo "$LEDGER_FILES" > "$LOG_FILE"
    fi
done

if [[ $NEW_FILE_COUNTER != 0 ]]; then
    echo "$NEW_FILE_COUNTER new files processed!"
fi
