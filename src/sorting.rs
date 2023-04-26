#![allow(dead_code)]
pub fn merge_sort<T: Ord + Copy> (data: &[T]) -> Vec<T> {
    let n = data.len();
    if n < 2 { return Vec::from(data); }
    let m = n / 2;
    let first_half = merge_sort(&data[..m]);
    let second_half = merge_sort(&data[m..]);
    return merge(&first_half, &second_half);

    fn merge<T: Ord + Copy> (first: &[T], second: &[T]) -> Vec<T> {
        let f = first.len();
        let s = second.len();
        let mut i: usize = 0;
        let mut j: usize = 0;
        let mut res: Vec<T> = Vec::new();

        while i < f && j < s {
            if first[i] < second[j] {
                res.push(first[i]);
                i += 1;
            } else {
                res.push(second[j]);
                j += 1;
            }
        }

        while i < f {
            res.push(first[i]);
            i += 1;
        }

        while j < s {
            res.push(second[j]);
            j += 1;
        }
        res
    }
}

pub fn bubble_sort<T: Ord + Copy> (data: &[T]) -> Vec<T> {
    let mut out = Vec::from(data);
    let n = data.len();
    loop {
        let mut sorted = true;
        for i in 0..(n-1) {
            let j = i + 1;
            if out[i] > out[j] {
                sorted = false;
                (out[i], out[j]) = (out[j], out[i]);
            }
        }
        if sorted { break; }
    }
    out
}

pub fn cocktail_sort<T: Ord + Copy> (data: &[T]) -> Vec<T> {
    let mut arr = Vec::from(data);
    let n = arr.len();
    for i in 0..(n/2) {
        for j in i..(n-i-1) {
            if arr[j] > arr[j+1] { 
                (arr[j], arr[j+1]) = (arr[j+1], arr[j]); 
            }
        }
        for j in (i+1)..(n-i) {
            if arr[n-j] < arr[n-j-1] {
                (arr[n-j], arr[n-j-1]) = (arr[n-j-1], arr[n-j]);
            }
        }
    }
    arr
}

pub fn odd_even_sort<T: Ord + Copy> (data: &[T]) -> Vec<T> {
    let mut arr = Vec::from(data);
    let n = arr.len();
    let mut start = 0;
    'sort: loop {
        let mut sorted = true;
        for i in (start..n-1).step_by(2) {
            if arr[i] > arr[i+1] {
                sorted = false;
                (arr[i], arr[i+1]) = (arr[i+1], arr[i]);
            }
        }
        if sorted { break 'sort; }
        start = 1 - start;
    }
    arr
}

pub fn comb_sort<T: Ord + Copy> (data: &[T]) -> Vec<T> {
    let arr = Vec::from(data);
    arr
}

pub fn selection_sort<T: Ord + Copy> (data: &[T]) -> Vec<T> {
    let mut arr = Vec::from(data);
    let n = arr.len();
    for i in 0..n {
        let mut max = arr[0];
        let mut max_index = 0;
        for j in 0..(n-i) {
            if arr[j] > max {
                max = arr[j];
                max_index = j;
            }
        }
        (arr[max_index], arr[n-i-1]) = (arr[n-i-1], arr[max_index]);
    }
    arr
}

pub fn doubly_selection_sort<T: Ord + Copy> (data: &[T]) -> Vec<T> {
    let mut arr = Vec::from(data);
    let n = arr.len();
    for i in 0..n {
        let mut max = arr[0];
        let mut max_i = 0;
        let mut min = arr[1];
        let mut min_i = 1;
        for j in 0..(n-i) {
            if arr[j] > max {
                max = arr[j];
                max_i = j;
            } else if arr[j] < min {
                min = arr[j];
                min_i = j;
            }
        }
        (arr[max_i], arr[n-i-1]) = (arr[n-i-1], arr[max_i]);
        (arr[min_i], arr[0]) = (arr[0], arr[min_i]);
    }
    arr
}
