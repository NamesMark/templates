fn insertion_sort(nums: &mut [i32]) {
    if nums.len() <= 1 { return; }

    for i in 1..nums.len() {
        let num = nums[i];
        let mut j = i;
        while j > 0 && nums[j-1] > num {
            nums[j] = nums[j-1];
            j -= 1;
        }
        nums[j] = num;
    }
}