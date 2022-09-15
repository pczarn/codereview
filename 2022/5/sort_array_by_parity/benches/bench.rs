
#![feature(slice_swap_unchecked, core_intrinsics)]

use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Sorts a vector, so that even numbers appear first.
fn sort_array_by_parity_orig(nums: Vec<i32>) -> Vec<i32> {
    let mut res = nums.clone();
    let mut even = 0;  // index of first element of unknown parity

    let mut cursor = nums.iter().enumerate();

    for (i, n) in nums.iter().enumerate() {
        if n % 2 == 0 {
            res.swap(even, i);
            even += 1;
        }
    }
    
    res
}

// Sorts a vector, so that even numbers appear first.
fn sort_array_by_parity(nums: Vec<i32>) -> Vec<i32> {
    let mut res = nums.clone();
    let mut even = 0;  // index of first element of unknown parity

    let mut cursor = nums.iter().enumerate();

    for (i, n) in nums.iter().enumerate() {
        if n % 2 == 0 {
            // this is safe because `even` never exceeds `res` length
            // and `i` is an index into `even`
            unsafe {
                res.swap_unchecked(even, i);
            }
            even += 1;
        }
    }
    
    res
}

use std::intrinsics;

// Sorts a vector, so that even numbers appear first.
fn sort_array_by_parity_0(nums: Vec<i32>) -> Vec<i32> {
    let mut res = nums.clone();
    let mut even = 0;  // index of first element of unknown parity

    let mut cursor = nums.iter().enumerate();

    for (i, n) in nums.iter().enumerate() {
        if n % 2 == 0 {
            // this is safe because `even` never exceeds `res` length
            // and `i` is an index into `even`
            unsafe {
                intrinsics::assume(even <= i);
            }
            res.swap(even, i);
            even += 1;
        }
    }
    
    res
}

use itertools::partition;

// Sorts a vector, so that even numbers appear first.
fn sort_array_by_parity_2(mut nums: Vec<i32>) -> Vec<i32> {
    partition(&mut nums, |n| n % 2 == 0);
    nums
}

use std::cell::Cell;

// Sorts a vector, so that even numbers appear first.
fn sort_array_by_parity_3(nums: Vec<i32>) -> Vec<i32> {
    let mut res: Vec<_> = nums.into_iter().map(|n| Cell::new(n)).collect();
    let mut even_cursor = res.iter();

    for n in &res {
        if n.get() % 2 == 0 {
            let dest = even_cursor.next().unwrap();
            let temp = n.get();
            n.set(dest.get());
            dest.set(temp);
        }
    }
    
    res.into_iter().map(|n| n.get()).collect()
}

// Sorts a vector, so that even numbers appear first.
fn sort_array_by_parity_4(mut nums: Vec<i32>) -> Vec<i32> {
    let mut even = 0;  // index of first element of unknown parity

    for i in 0 .. nums.len() {
        unsafe {
            if nums.get_unchecked(i) % 2 == 0 {
                // this is safe because `even` never exceeds `res` length
                // and `i` is an index into `even`
                let tmp = *nums.get_unchecked(even);
                *nums.get_unchecked_mut(even) = *nums.get_unchecked(i);
                *nums.get_unchecked_mut(i) = tmp;
                even += 1;
            }
        }
    }

    nums
}

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parity-orig", |b| b.iter(|| sort_array_by_parity_orig(black_box(vec![3, 2, 1, 0]))));
    c.bench_function("parity-unsafe", |b| b.iter(|| sort_array_by_parity(black_box(vec![3, 2, 1, 0]))));
    c.bench_function("parity-assume", |b| b.iter(|| sort_array_by_parity_0(black_box(vec![3, 2, 1, 0]))));
    c.bench_function("parity-partition", |b| b.iter(|| sort_array_by_parity_2(black_box(vec![3, 2, 1, 0]))));
    c.bench_function("parity-cell", |b| b.iter(|| sort_array_by_parity_3(black_box(vec![3, 2, 1, 0]))));
    c.bench_function("parity-unsafe-2", |b| b.iter(|| sort_array_by_parity_4(black_box(vec![3, 2, 1, 0]))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);