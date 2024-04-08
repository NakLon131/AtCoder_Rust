if [ $# -lt 1 ]; then
	echo 移動するフォルダを指定してください
	exit 1
fi
FOLDER=$1
MVPATH="../../AtCoders"
TODAY=`date +"%Y%m%d"`

mkdir $MVPATH/$FOLDER-$TODAY
cp $FOLDER/src/bin/* $MVPATH/$FOLDER-$TODAY/
rm -rf $FOLDER