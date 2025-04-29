if [ $# -lt 1 ]; then
	echo 引数が足りません
	exit 1
fi

cargo run < testcase/$1.txt > testresult/$1.txt