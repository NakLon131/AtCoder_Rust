if [ $# -lt 2 ]; then
	echo 引数が足りません
	exit 1
fi

# コンテスト開始
contest_new() {
	EXAM=$1
	cp -r abc_template $EXAM
	# cp thoughts.md $1/src/bin
}

# コードコピー
# cat *.rs | xsel --clipboard --input
code_copy() {
	EXAM=$1
	NO=$2
	cd $EXAM
	cat src/bin/$NO.rs | xsel --clipboard --input
}

if [ $1 = "n" ]; then
	if [ $# -lt 2 ]; then
		echo 問題番号を指定してください
		exit 1
	fi
	contest_new $2

elif [ $1 = "sc" ]; then
	if [ $# -lt 3 ]; then
		echo 問題番号を指定してください
		exit 1
	fi
	if [ $? -eq 0 ]; then
		code_copy $2 $3
		echo ソースコードをコピーしました。
	fi

fi