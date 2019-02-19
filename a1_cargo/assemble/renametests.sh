#!/usr/bin/env bash
for i in tests/*.o; do
	mv $i ${i%.o}.expected
done
