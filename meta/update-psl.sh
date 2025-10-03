#!/usr/bin/env bash
CWD=$(dirname "$0")

wget -q -O "$CWD/public_suffix_list.dat" https://publicsuffix.org/list/public_suffix_list.dat 
