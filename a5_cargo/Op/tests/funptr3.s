setframe 0
push Lmain
call
halt
Lmain:
push undef
push false
push _L1
branch
push Lg
setframe 3
swap
call
ret
push true
push _L2
branch
_L1:
push Lf
setframe 3
swap
call
ret
push true
push _L2
branch
_L2:
store 2
push 3
push Lfptr
setframe 4
swap
call
ret
store 2
Lf:
var 0
Lg:
push 1
var 0
binary +
ret
