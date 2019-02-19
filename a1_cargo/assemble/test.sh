#!/usr/bin/env bash
for f in tests/*.s; do
	cargo run $f 2>/dev/null
	if diff ${f%.s}.expected ${f%.s}.o; then
		echo "$f: passed"
	else
		echo "$f: FAILED!!!"
	fi
done
