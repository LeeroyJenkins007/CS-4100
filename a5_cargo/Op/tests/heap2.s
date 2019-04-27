setframe 0
push Lmain
call
halt
Lmain:
push 200
push Lf
setframe 2
swap
call
ret
Lf:
push 0
var 0
binary ==
push _L1
branch
push 100
push false
alloc
pop
push undef
push 1
var 0
alloc
store 5
push 1
var 5
push 0
get
binary -
push Lf
setframe 6
swap
call
ret
store 5
push true
push _L2
branch
_L1:
push 3
push true
push _L2
branch
_L2:
ret
