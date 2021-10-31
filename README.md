# Civic Senior Blockchain Engiennier Challenge 2
# Description
* This script takes two arguments from the CLI
* The first argument is a string DAG, from here the program creates a DAG
* The DAG is formed by a vector of structs, where each element is a Node (Struct)
* The second parameter is the vertex that we are deleting, a node represented by a character
* IMPORTANT: I do not check for the syntax of the parameters
* IMPORTANT: From the test case pdf, the objective is to delete a node, and reconnect its childs with its parent

# Workflow
This is my first program in Rust, so first I coded a JS program to visualize the logical flow
* 1) Create a DAG from the string
* 2) Strip the DAG
* 3) Print the DAG alphabetically sorted

# Commands
* To run the help menu: run cargo
* To run example: run cargo example
* To run a custom DAG, Write: cargo run dag removeNode
*   a) Where, dag= a-b,b-c,c-d,c-e,e-f
*   b) Where, removeNode is one character, for example c

# Tests
* There are 10 tests in total
* Seven Unit Tests with two different DAGs
* Three System Tests with three different DAGs