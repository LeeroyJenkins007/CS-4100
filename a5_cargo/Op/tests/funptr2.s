setframe 0
push Lmain
call
halt
Lmain:
push 3
push undef
push Lf
setframe 3
swap
call
ret
store 3
var 3
store 3
Lf:
var 0
ret
