if [ $# -lt 2 ]; then
	echo 引数が足りません
	exit 1
fi

cd ahc$1/src/bin
rustc a.rs
./a < ../../testcases/seed_$2.txt > result.txt

