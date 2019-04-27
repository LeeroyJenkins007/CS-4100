setframe 0
push Lmain
call
halt
Lmain:
push undef
push 20
store 2
var 2
push 1
var 2
binary +
push 0
alloc
push 1
var 2
binary +
push false
alloc
push Lfib
setframe 3
swap
call
ret
store 2
Lfib:
push undef
var 0
push 0
binary ==
push _L7
branch
var 0
push 1
binary ==
push _L5
branch
var 2
push 2
var 0
binary -
get
push _L1
branch
push 2
var 0
binary -
var 1
var 2
push Lfib
setframe 5
swap
call
ret
push true
push _L2
branch
_L1:
var 1
push 2
var 0
binary -
get
push true
push _L2
branch
_L2:
var 2
push 1
var 0
binary -
get
push _L3
branch
push 1
var 0
binary -
var 1
var 2
push Lfib
setframe 5
swap
call
ret
push true
push _L4
branch
_L3:
var 1
push 1
var 0
binary -
get
push true
push _L4
branch
_L4:
binary +
push true
push _L6
branch
_L5:
push 1
push true
push _L6
branch
_L6:
push true
push _L8
branch
_L7:
push 1
push true
push _L8
branch
_L8:
store 4
var 1
var 0
var 4
set
push tt
pop
var 2
var 0
push true
set
push tt
pop
var 4
store 4
ret
