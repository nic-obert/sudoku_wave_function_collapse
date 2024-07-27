#!/bin/bash

find src -name "*.rs" | xargs wc -l | sort -nr
