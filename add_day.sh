#!/bin/env bash
day=$1
if [ -f "src/day$day.rs" ]; 
then
echo "File for day $day already exists"
else
echo "Adding day $day"
# Replace day number in template and create new day file
sed "s|\DAY_NUMBER|${day}|g" "day_template.rs" > "src/day$day.rs"

# Create Input files for easy filling
touch "inputs/day$day_test.txt"
touch "inputs/day$day.txt"

# Add module and match to main.rs
sed  -i "/\/\/ Day modules/i mod day${day};" src/main.rs 
sed  -i "/[[:blank:]]*\/\/ Day invocations/i ${day} => day${day}::run(bench)?," src/main.rs 
rustfmt src/main.rs
rustfmt "src/day$day.rs"

fi
