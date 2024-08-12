# 木の重心

ABC348E

- 木の重心を求める
- 今の重みと下の階層の重みの合計((1)再帰で調べる) 、それ以外がどちらも半分以下(全体の重み-(1))
- 自身の重み + 子の重み , その他がそれぞれ半分以下
  - 子の部分木の重みの合計も調べる必要がある。

```rust
// 入力
// 開始位置、ひとつ前の位置(後戻り禁止)
// 探索できる辺
// 重み
// 取得する頂点(*)
// 出力
// 部分木の重み
fn graph_gravity_dfs(
	cpos: usize, ppos: usize, 
	g: &Vec<Vec<usize>>, c: &Vec<usize>, 
	totcost: usize, 
	getpos: &mut usize
) -> usize{
	let mut maxcost = 0; // 木の重みのmax

	let mut nowcost = 0; // 自身木の重み
	nowcost += c[cpos];	// 自身を含む木の重み
	
	// 部分木を調べる
	for &npos in &g[cpos] {
		if npos == ppos { continue; }
		
		// 部分木取得
		let getcost = gravity_dfs(npos, cpos, g, c, totcost, getpos);
		maxcost = max(maxcost, getcost);

		// 部分木の重みを加える
		nowcost += getcost;
	}

	// 自身の重み含む部分木、その他のmaxをとる
	maxcost = max(maxcost, totcost - nowcost);
	if maxcost * 2 <= totcost {
		// 木の重心を取得
		*getpos = cpos;
	}
	nowcost
}

```