# Fix My Static Analysis Java

[![Build Status](https://travis-ci.com/QuentinLee5/fix-my-static-analysis-Java.svg?branch=master)](https://travis-ci.com/QuentinLee5/fix-my-static-analysis-Java)

[![codecov](https://codecov.io/gh/QuentinLee5/fix-my-static-analysis-Java/branch/master/graph/badge.svg)](https://codecov.io/gh/QuentinLee5/fix-my-static-analysis-Java)

This tool provides support for fixing checkstyle errors on your maven java project. 

## Functionality
- Removing unused imports.
- Adding spaces before or after characters where these are desirable.
- Add standard javadoc to getters and setters.
- Put modifiers in the right order.

## Requirements
- Your project should be using maven.
- The maven checkstyle plug-in should be installed.
- You should have provided your own checkstyle.xml
- Rust should be installed on your computer.

## Using the tool
To use this tool, open the terminal. Next run the Rust program using the file path of the maven project as argument.
When finished, the tool should have fixed the above mentioned checkstyle errors.
