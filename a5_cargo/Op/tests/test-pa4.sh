#!/usr/bin/env bash
pushd .. &>/dev/null

if ! make -s build; then
	exit 1
fi

i=0
j=0

pass=( "tests/array2.gpy" \
	"tests/array3.gpy" "tests/array.gpy" "tests/comment.gpy" "tests/div.gpy" "tests/fact.gpy" "tests/fib-memo.gpy" \
	"tests/fib.gpy" "tests/funptr2.gpy" "tests/funptr3.gpy" "tests/funptr.gpy" "tests/heap.gpy" "tests/heap2.gpy" "tests/heap3.gpy" "tests/let1.gpy" \
	"tests/let2.gpy" "tests/minus.gpy" "tests/multi-arg.gpy" "tests/neg.gpy" \
	"tests/plus.gpy" "tests/seq.gpy" "tests/times.gpy" )

fail=( "tests/fact2.gpy" )

echo "Cases that should not have a result:"
for f in "${fail[@]}"
do
	((j++))
	make -s run F=$f > "${f%.gpy}.s" 2>/dev/null
	if tests/grumpy-assembly-interp "${f%.gpy}.s" > "${f%.gpy}.result" 2>/dev/null; then
		echo "FAILED: $f produced a result!"
	else
		echo "passed"
		((i++))
	fi
done

echo
echo "Cases that should have a result:"
for f in "${pass[@]}"; do
	((j++))
	make -s run F=$f > "${f%.gpy}.s" 2>/dev/null
	if ! tests/grumpy-assembly-interp "${f%.gpy}.s" > "${f%.gpy}.result" 2>/dev/null;  then
		rm "${f%.gpy}.result"
	fi
done
for f in "${pass[@]}"; do
	if [ ! -f "${f%.gpy}.expected" ]; then
		echo "${f%.gpy}.expected not found!"
	elif [ ! -f "${f%.gpy}.result" ]; then
		echo "FAILED: ${f%.gpy}.result not found!"
	elif cmp -s "${f%.gpy}.expected" "${f%.gpy}.result"; then
		echo "passed"
		((i++))
	elif cmp -s "${f%.gpy}.other_expected" "${f%.gpy}.result"; then
		echo "passed"
		((i++))
	else
		echo "FAILED: $f output wrong!"
	fi
done

popd &>/dev/null

echo
echo "$i tests passed out of $j total!"
