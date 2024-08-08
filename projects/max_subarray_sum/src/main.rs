// Problem: Given an array of n numbers, our task is to calculate the max subarray sum
// i.e. the largest possible sum of a sequence of consecutive values in the array.
// The algorithm used here in known as Kadane's Algorithm. It solves the problem in O(n) time

fn max_subarray_sum(arr: Vec<i32>) -> i32 {
    let mut max = arr[0];
    let mut sum = 0;
    for i in 0..arr.len() {
        sum += arr[i];
        if sum > max {max = sum;}
        if (sum < 0) {sum = 0;}
    }
    return max;
}

fn main() {
    let arr = vec![1,2,0,-3,5,10,-5,4,6];
    println!("{}", max_subarray_sum(arr));
}
