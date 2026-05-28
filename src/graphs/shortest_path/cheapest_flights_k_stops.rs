use std::cmp::min;

// 2D Bellman Ford
pub fn find_cheapest_price_unoptimized(
    n: i32,
    flights: Vec<Vec<i32>>,
    src: i32,
    dst: i32,
    k: i32,
) -> i32 {
    //  preliminary
    let n = n as usize;
    let src = src as usize;
    let dst = dst as usize;
    let k = k as usize;

    // DP to store cost to each node using k number of edges
    let mut dp: Vec<Vec<i32>> = vec![vec![i32::MAX; n]; k + 1];

    // set the starting cost of getting to the source as 0
    for k in dp.iter_mut() {
        k[src] = 0;
    }

    let mut ans = i32::MAX;

    for i in 0..k + 1 {
        for edge in flights.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let c = edge[2];

            match i {
                0 => {
                    dp[i][v] = match u == src {
                        true => min(dp[i][v], c),
                        _ => dp[i][v],
                    }
                }
                _ => {
                    if dp[i - 1][u] != i32::MAX {
                        dp[i][v] = min(dp[i][v], dp[i - 1][u] + c)
                    }
                }
            }

            if v == dst {
                ans = ans.min(dp[i][v]);
            }
        }
    }

    if ans == i32::MAX { -1 } else { ans }
}

// 1D Bellman Ford
pub fn find_cheapest_price(n: i32, flights: Vec<Vec<i32>>, src: i32, dst: i32, k: i32) -> i32 {
    //  preliminary
    let n = n as usize;
    let src = src as usize;
    let dst = dst as usize;
    let k = (k + 1) as usize;

    // Prevents overflows, but is sufficiently large
    let inf = i32::MAX / 2;

    let mut prev = vec![inf; n];
    prev[src] = 0;
    let mut ans = prev[dst];

    for _ in 0..k {
        let mut curr = prev.clone();

        for edge in flights.iter() {
            let u = edge[0] as usize;
            let v = edge[1] as usize;
            let c = edge[2];

            curr[v] = curr[v].min(prev[u] + c);
        }

        ans = ans.min(curr[dst]);
        prev = curr;
    }

    if ans >= inf { -1 } else { ans }
}
