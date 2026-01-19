if [ $# != 1 ]; then
	echo 問題番号を指定してください
	exit 1
else
	TODAY=`date +"%Y%m%d_%H%M"`
	cp src/main.rs done/$1_$TODAY.rs
	# cp base_rs/atcoder.rs src/main.rs
	cp base_rs/plain.rs src/main.rs
fi