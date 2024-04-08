if [ $# != 1 ]; then
	echo バックアップするファイル名を指定してください。
	exit 1
else
	cp src/bin/a.rs bk/$1.rs
fi