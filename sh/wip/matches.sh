#!/bin/bash

# Define the substring to match and the list of words as a space-separated string
substring_to_match="exam"
word_list="sample example test exemplar exam"

# Convert the string to a newline-separated list, filter with grep, and use fzf for selection
selected_word=$(ls | tr ' ' '\n' | grep "$substring_to_match" | fzf)

echo "$selected_word"

