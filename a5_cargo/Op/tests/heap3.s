setframe 0
push Lmain
call
halt
Lmain:
push 0
push Lg
setframe 2
swap
call
ret
Lf:
push 1
push 3
binary /
var 0
binary ==
push _L1
branch
push 100
push false
alloc
pop
push 1
var 0
binary -
push Lf
setframe 4
swap
call
ret
push true
push _L2
branch
_L1:
push 3
push true
push _L2
branch
_L2:
Lg:
push undef
push 1
push 100
push 10
alloc
alloc
store 5
push 15
push Lf
setframe 7
swap
call
ret
pop
var 5
push 0
get
push 50
get
store 5
ret
