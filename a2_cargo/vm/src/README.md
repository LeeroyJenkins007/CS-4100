Author: Greg Jenkins
DATE: 01/Mar/2019
Project: PA2 Virtual Machine implementation

In this implementation of the Grumpy VM, I used Rust as my choice of language, and designed the 
VM to it's strengths with pattern matching. 

+Converting Binary to Instructions:
	I used traits and the Byteorder Crate to convert binary into a vector of instructions which would be 
	used later for execution.

+Execution loop:
	I used Switch_dispatching for my execution loop, to play on the strengths of the Rust programming 
	language.

Learned Things:
-Keeping track of who owns what and what can be borrowed was a large task, especially since I am still
new to this kind of language.
