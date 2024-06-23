fn merge_sort(nums: &mut [i32]) {
    if nums.len() <= 1 { return; }
    let n = nums.len();

    merge_sort(&mut nums[..n/2]);
    merge_sort(&mut nums[n/2..]);

    // Optimization: half-size aux array
    let aux = nums[..n/2].to_vec();
    let mut r_a = 0; // reader aux
    let mut r_n = n/2; // reader nums
    let mut w = 0; // writer nums

    while r_a < aux.len() || r_n < n {
        if r_n >= n || (r_a < aux.len() && aux[r_a] <= nums[r_n]) {
            nums[w] = aux[r_a];
            r_a += 1;
        } else {
            nums[w] = nums[r_n];
            r_n += 1;
        }
        w += 1;
    }
}