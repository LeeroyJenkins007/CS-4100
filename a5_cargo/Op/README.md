Author: Greg Jenkins
Assignment: PA4

Overview:
	The program is to take in .gpy files, which are an intermediate representation of the Grumpy lannguage. It
will then print to stdout the assembly code that could be handed to the assembler. I used a recursive descent parser,
and LL(1) compiler. There are muliple files associated with this assignment:

main.rs - This is the file that takes in the .gpy and hands it off the the lexer/parser, then once its returned hands 
	the list of instructions to the compiler.

types.rs - This holds all the grammar forms and extra functions for each struct, enum or type that is used throughout 
	 the rest of the program. By following the types outlined in this file, keeps it from falling outside the grammar
	outlined in the Grumpy documentation.

lexer.rs - This tokenizes the entire .gpy file, and provides functions to peek at the next token, and eat the token.
	Although you can tokenize the entire file at once, the parser incrementally calls lexer as its parsing throgh
	the file. This comes in handy when determining if the variable you just tokenized is the name of a function
	or if it is part of a let expression. 

parser.rs - This works in conjunction with the lexer to create an abstract systax tree that could then be handed to the 
	compiler to work through. This also uses structs and enums defined in types.rs to maintain the integrity of the
	grammar. This file iteratively calls on the lexer to work its way through the file, and helps to find syntax 
	errors or unwanted tokens.

compile.rs - This files takes the abstract syntax tree and uses LL(1) to produces a list of instruction that are in 
	the Grumpy Assembly language, which will be output to the standard out. It is here that the variable locations 
	and labels are produced. 
