if [ $# != 1 ]; then
	echo バックアップ時のファイル名を指定してください。
	exit 1
else
	cp src/main.rs bk_rs/$1.rs
fi