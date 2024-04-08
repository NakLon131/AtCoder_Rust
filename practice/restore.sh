if [ $# != 2 ]; then
	echo リストアするファイル名、反映する問題名を指定してください。
	exit 1
else
	cp bk/$1.rs src/bin/$2.rs
fi