use std::{iter, num};

pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut m = m as usize; let mut n = n as usize;
    while (n > 0) {
        if m > 0 && (nums1[m-1] > nums2[n-1]) {
            nums1[m+n-1] = nums1[m-1];
            m -= 1;
        }
        else {
            nums1[m+n-1] = nums2[n-1];
            n -= 1;
        }
    }
}

// pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
//     let mut i: usize = (m-1) as usize;
//     let mut j: usize = (n-1) as usize;
//     if (m > 0) && (n > 0) {
//             while (i >= 0) || (j >= 0) {
//                 if nums1[i] >= nums2[j] {
//                     nums1[i+j+1] = nums1[i];
//                     if (i == 0) && (j == 0) {
//                         nums1[i] = nums2[j];
//                         break;
//                     }
//                     if i != 0 {i -= 1;}
//                     }
//                 else if nums2[j] > nums1[i] {
//                     nums1[i+j+1] = nums2[j];
//                     if j == 0 {
//                         break;
//                     }
//                     if j != 0 {j -= 1;}
//                 }
//             }
//     }
//     else if m == 0 {
//         for i in 0..n {
//             nums1[i as usize] = nums2[i as usize];
//         }
//     }
// }

fn main() {
    let mut nums1: Vec<i32>  = vec![1,3,4,0,0,0];
    let mut nums2: Vec<i32>= vec![0,2,6];
    merge(nums1.as_mut(), 3, nums2.as_mut(), 3);
    println!("{:?}", nums1);
    println!("Hello, world!");

    let x = 0;
    let y = 1;
    if (x && y) {
        println!("True");
    }
    else {
        println!("False");
    }
}
