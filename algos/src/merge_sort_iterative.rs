fn merge_sort_iterative(nums: &mut [i32]) {
    let n = nums.len();
    let mut size_power = 0;
    let mut size = 2_i32.pow(size_power) as usize;

    // for window size in powers of two
    while size <= n {
        let mut start = 0;
        // for every window
        while start < n {
            let right = (start + 2 * size).min(n);
            let pivot = (start + size).min(n);
            merge_helper(nums, start, right, pivot);
            start += 2 * size;
        }

        size_power += 1;
        size = 2_i32.pow(size_power) as usize;
    }
}

fn merge_helper(nums: &mut [i32], left: usize, right: usize, pivot: usize) {
    let aux = nums[left..pivot].to_vec(); // Optimization #2: half-size aux array
    let mut r_a = 0; // reader aux
    let mut r_n = pivot; // reader nums
    let mut w = left; // writer nums

    while r_a < aux.len() || r_n < right {
        if r_n >= right || (r_a < aux.len() && aux[r_a] <= nums[r_n]) {
            nums[w] = aux[r_a];
            r_a += 1;
        } else {
            nums[w] = nums[r_n];
            r_n += 1;
        }
        w += 1;
    }
}