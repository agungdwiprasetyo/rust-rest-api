use std::cmp;

pub const MAX: i32 = i32::max_value();

pub fn calculate_dp(elements: Vec<usize>, el: usize) -> i32 {
    let mut tabel_dp = vec![0; el+1];
    let n = elements.len();
    for i in 1..el+1 {
        let mut max: i32 = MAX;
        for j in 0..n {
            let visit = elements[j];
            if visit <= i {
                let idx: usize = (visit) as usize; 
                let val: i32 = tabel_dp[i-idx] as i32;
                match val.checked_add(1) {
                    Some(v) => {
                        max = cmp::min(max, v);
                    }
                    None => {
                        println!("i32 overflow");
                    }
                }
            }
        }
        tabel_dp[i] = max;
    }

    tabel_dp[el]
}
