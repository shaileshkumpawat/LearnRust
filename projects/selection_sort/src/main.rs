use std::{array, mem::swap};

fn max_idx<T: Ord>(arr: &mut [T], i: u32) -> u32 {
    if i > 0 {
        let j: u32 = max_idx(arr, i-1);
        if arr[i as usize] < arr[j as usize] {
            return j
        }
    }
    i
}

// recursive implementation
fn selection_sort<T:Ord>(arr: &mut [T], mut i: Option<u32>) {
    match i {
        None => {
            i = Some((arr.len() - 1) as u32);
            selection_sort(arr, i);
        }
        Some(i) => {
            if i > 0 {
                let j = max_idx(arr, i);
                arr.swap(i as usize, j as usize);
                selection_sort(arr, Some(i-1));
            }
        }
    }
}


// iterative implementation of selection sort
pub fn selection_sort_iter<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for left in 0..len {
        let mut smallest = left;
        for right in (left + 1)..len {
            if arr[right] < arr[smallest] {
                smallest = right;
            }
        }
        arr.swap(smallest, left);
    }
}
fn main() {
    let mut arr = [8, 3, 5, 9, 2];
    println!("Before sort: {:?}", arr);
    // selection_sort(&mut arr, None);
    selection_sort_iter(&mut arr);
    println!("After sort: {:?}", arr);
    // println!("Hello, world!");
}
