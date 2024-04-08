if [ $# -lt 2 ]; then
	echo 引数が足りません
	exit 1
fi

# コンテスト開始
# cargo compete new xxx
contest_new() {
	EXAM=$1
	cargo compete new $EXAM
}

# テスト実行
# cargo compete test xxx
contest_test() {
	EXAM=$1
	NO=$2
	RELEASE=$3
	cd $EXAM
	if [ $# -eq 3 ]; then
		cargo compete test $NO --release
	else
		cargo compete test $NO
	fi
}

# 提出
# cargo compete submit xxx
contest_submit() {
	EXAM=$1
	NO=$2
	RELEASE=$3
	cd $EXAM
	if [ $# -eq 3 ]; then
		cargo compete submit $NO --release
	else
		cargo compete submit $NO
	fi
}


if [ $1 = "n" ]; then
	contest_new $2
elif [ $1 = "t" ]; then
	if [ $# -lt 3 ]; then
		echo 問題番号を指定してください
		exit 1
	fi
	contest_test $2 $3 $4
elif [ $1 = "s" ]; then
	if [ $# -lt 3 ]; then
		echo 問題番号を指定してください
		exit 1
	fi
	contest_submit $2 $3 $4
fi