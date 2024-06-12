if [ $# != 1 ]; then
	echo 問題番号を指定してください
	exit 1
else
	TODAY=`date +"%Y%m%d_%H%M"`
	cp src/bin/a.rs done/$1_$TODAY.rs
	cp base.rs src/bin/a.rs
fi