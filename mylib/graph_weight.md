# 木の重み

ABC348E

- 木の重心からコストを求める
- 「重心に対する距離(辺の数)*その頂点の重み 」の総和
- 全体の重みを求める必要があるので、直前まで調べた分 + 自身の重み + 子の重み

```rust
// 入力
// 開始位置、ひとつ前の位置(後戻り禁止)
// 探索できる辺
// 重み
// 距離
// 現在の重み
// 出力
// 現在の重みを加えた重みを返す
fn graph_weight_dfs(
	cpos: usize, ppos: usize, 
	g: &Vec<Vec<usize>>, c: &Vec<usize>, 
	dist: usize, ccost: usize
) -> usize {
	let mut ncost = ccost + c[cpos] * dist;	// それまでの重みに、自身の重み*距離を加える
	for &npos in &g[cpos] {
		if npos == ppos { continue; }
		// 下の階層の重み*距離も加える
		ncost += graph_cost_dfs(npos, cpos, g, c, dist+1, ccost);
	}
	ncost	
}

```