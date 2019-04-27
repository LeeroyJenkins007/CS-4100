setframe 0
push Lmain
call
halt
Lmain:
push 12
push Lfact
setframe 2
swap
call
ret
Lfact:
push undef
push 100
push 0
alloc
store 3
var 3
push 0
var 0
set
push tt
pop
push undef
var 3
push 0
get
store 6
var 6
push 0
binary ==
push _L1
branch
push 1
var 6
binary -
push Lfact
setframe 7
swap
call
ret
var 6
binary *
push true
push _L2
branch
_L1:
push 1
push true
push _L2
branch
_L2:
store 6
store 3
ret
