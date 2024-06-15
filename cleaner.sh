sudo sh -c "echo 3 >'/proc/sys/vm/drop_caches'"  # swap=0の場合
sudo sh -c "echo 3 >'/proc/sys/vm/drop_caches' && swapoff -a && swapon -a" # swap設定値ありの場合
rm -rf target/debug/.fingerprint/abc*
rm -rf target/debug/deps/abc*
rm -rf target/debug/deps/libabc*
rm -rf target/debug/incremental/abc*
rm -rf target/debug/abc*
rm -rf target/release/.fingerprint/abc*
rm -rf target/release/deps/abc*
rm -rf target/release/deps/libabc*
rm -rf target/release/incremental/abc*
rm -rf target/release/abc*