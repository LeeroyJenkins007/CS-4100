Author: Greg Jenkins

Overview:
	Added the functionality that was prescribed in the pa3.md

Part 0:
	fixed previous issues from pa2, the only thing that needed repair was to check for a heap size exceeding 1024 objects. This was straigthforward
and was fixed by checking everytime ALLOC was called to see whether or not what the program was asking would exceed the amount, if it did panic, otherwise
continue. (This functionality was changed in Part 1)

Part 1:
	Add the garbage collection. This was done anytime the heap would exceed HEAP_SIZE and would call collect_garbage(). This is a copy collecting 
garbage collector, specifically Cheney's algorithm. 

Part 2:
	Adds the ability for multiple threads (not parrallel), this was done by creating a Vector of states, where each state was its own thread, and 
went through each thread, and called instr() the number of times dictated by quantum because I implemented a round-robin schedualer. 
