use std::cmp;

const MAX: i32 = i32::max_value();

pub fn calculate_dp(elements: Vec<usize>, el: usize) -> usize {
    let mut tabel_dp = vec![0; el+1];
    let n = elements.len();
    for i in 1..el+1 {
        let mut max: i32 = MAX;
        for j in 0..n {
            let visit = elements[j];
            if visit <= i {
                let idx: usize = (visit) as usize;
                max = cmp::min(max, tabel_dp[idx]+1);
            }
        }
        tabel_dp[i] = max;
    }

    tabel_dp[el] as usize
}
