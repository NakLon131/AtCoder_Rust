items=(
	"a.rs"
	"b.rs"
	"c.rs"
	"d.rs"
	"e.rs"
	"f.rs"
	"g.rs"
	"h.rs"
	"i.rs"
	"j.rs"
)


cd abc_template/src/bin/
for item in "${items[@]}" ; do
	cp z.rs $item
done
cd ../../../
cp abc_template/src/bin/z.rs work/src/main.rs