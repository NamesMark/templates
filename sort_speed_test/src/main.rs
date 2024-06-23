#![allow(unused_imports)]
use std::io::{self, BufRead};
use rand::prelude::*;
use std::time::Instant;

fn main() {
    let mut rng = rand::thread_rng();
    //let sizes = [10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 200, 210, 220, 230, 240, 250, 260, 300, 310, 320, 330, 340, 350, 360, 370, 400, 500, 600, 700, 800, 1000];

    let mut results = vec![];

    for _ in 0..100 {
        let mut left = 1;
        let mut right = 10000;
        for _ in 0..10 {
            let size = left + (right - left) / 2;
            let mut data: Vec<i32> = (0..size).map(|_| rng.gen_range(0..1000)).collect();
            let time_insertion = measure_sort(&mut data.clone(), insertion_sort);
            let time_merge = measure_sort(&mut data.clone(), merge_sort);

            if time_insertion <= time_merge {
                left = size;
            } else {
                right = size;
            }

            //println!("Size: {:>3} | Insertion Sort: {:>6}μs | Merge Sort: {:>6}μs {}", size, time_insertion, time_merge, if time_insertion < time_merge {"INSERTION"} else {"MERGE"});
        }
        results.push(right);
    }

    println!("Final size turning point: {}", results.iter().sum::<usize>() / results.len());
}

fn measure_sort<T>(data: &mut [T], sort_function: fn(&mut [T])) -> u128
where
    T: Ord + Copy,
{
    let start = Instant::now();
    sort_function(data);
    start.elapsed().as_micros()
}

fn merge_sort(nums: &mut [i32]) {
    if nums.len() <= 1 { return; }
    let n = nums.len();

    merge_sort(&mut nums[..n/2]);
    merge_sort(&mut nums[n/2..]);

    // Optimization #2: half-size aux array
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

