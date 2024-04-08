if [ $# -lt 1 ]; then
	echo 移動するフォルダを指定してください
	exit 1
fi
FOLDER=$1
MVPATH="../../AtCoders"
mkdir $MVPATH/$FOLDER
cp $FOLDER/src/bin/* $MVPATH/$FOLDER/
rm -rf $FOLDER