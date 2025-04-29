if [ $# != 1 ]; then
	echo リストアするファイル名を指定してください。
	exit 1
else
	cp bk_rs/$1.rs src/main.rs
fi