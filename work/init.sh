if [ $# -lt 1 ]; then
    echo 引数が足りません
    exit 1
fi

# AtCoderスタイル
if [ $1 = "a" ]; then
    cp base_rs/atcoder.rs src/main.rs
# Plainスタイル
elif [ $1 = "p" ]; then
    cp base_rs/plain.rs src/main.rs
# Otherスタイル
elif [ $1 = "o" ]; then
    cp base_rs/aizu.rs src/main.rs
fi