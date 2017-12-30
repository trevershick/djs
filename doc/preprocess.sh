#!/bin/bash

_file="${1:-/dev/null}"   #fail safe
while IFS= read -r line; do
    case "$line" in
        __INCLUDE__* )
            f=$(echo "$line" | awk '{print $2}')
            if [ ! -f "$f" ]; then
                >&2 echo "$f does not exist."
                exit 1
            fi
            cat "$f"
            ;;
        * )
            echo "$line"
            ;;
    esac
done < "$_file"
