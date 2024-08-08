use std::{collections::{HashMap, HashSet}, num, ops::Deref, vec};

pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
    let mut k = 0;
    for i in 0..nums.len() {
        if nums[i] != val {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}

pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    let mut char_set: HashSet<i32> = HashSet::new();
    let mut k = 0;
    for i in 0..nums.len() {
        if (!char_set.contains(&nums[i])) {
            char_set.insert(nums[i]);
            nums[k] = nums[i];
            k += 1;
        }
    }
    char_set.len() as i32
}

pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
    let mut char_set: HashMap<i32, i32> = HashMap::new();
    let mut k = 0;
    for i in 0..nums.len() {
        if (!char_set.contains_key(&nums[i])) {
            char_set.insert(nums[i], 1);
            nums[k] = nums[i];
            k += 1;
        }
        else if char_set[&nums[i]] == 1 {
            *char_set.get_mut(&nums[i]).unwrap() += 1;
            nums[k] = nums[i];
            k += 1;
        }
        else if char_set[&nums[i]] > 1 {
            *char_set.get_mut(&nums[i]).unwrap() += 1;
        }
    }
    k as i32
}

pub fn remove_duplicates2_sorted(nums: &mut Vec<i32>) -> i32 {
    let mut k = 1;
    for i in 1..nums.len() {
        if (i == 1) || (nums[i] != nums[k-2]) {
            nums[k] = nums[i];
            k += 1;
        }
    }
    k as i32
}

fn main() {
    // let mut nums = vec![0,2,-1,10,1,8,1,5,6,32,2,4,6,3,9,7,6,2];
    let mut nums = vec![0,1,1,1,2,3,4,4,5,6,6,6,6,6,7,7,8];
    println!("{:?}", nums);
    // let val = 2;
    // remove_element(&mut nums, val);
    let k = remove_duplicates2_sorted(&mut nums);
    println!("{:?}", nums);
    println!("#unique numbers = {}", k);
}