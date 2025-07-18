# backup
if [ $# -lt 1 ]; then
	echo 問題フォルダと番号を指定してください
	exit 1
fi
if [ $# -lt 2 ]; then
	echo 番号を指定してください
	exit 1
fi
TIME=`date +"%H%M%S"`
cp $1/src/bin/$2.rs $1/src/bin/$2_$TIME.rs

# reset
cp $1/src/bin/z.rs $1/src/bin/$2.rs