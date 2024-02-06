use std::collections::HashMap;

fn main() {
    let nums = vec![3, 2, 4];
    let target = 7;
    let mut map: HashMap<i32, u32> = HashMap::new();

    for x in 0..nums.len() {
        map.insert(nums[x], x as u32);
    }
    let mut complement: i32;
    for x in 0..nums.len() {
        complement = target - nums[x];
        if (map.contains_key(&complement) && map[&complement] != x as u32) {
            println!("{:?}",vec![x as u32, map[&complement]]);
            break;
        }
    }
}
  