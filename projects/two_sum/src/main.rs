//use std::vec;

fn main() {
    let nums = vec![3, 2, 4];
    let target = 6;
    let mut ind:Vec<i32> = Vec::new();
    let mut hit: bool = false;

    for x in 0..(nums.len()-1) {
        for y in (x+1)..nums.len(){
            match (nums[x] + nums[y] == target) {
                true => {
                    ind.push(x as i32);
                    ind.push(y as i32);
                    hit = true;
                    break;
                },
                false => {
                    continue
                }
            }
        }
        if hit {
            break;
        }
    }

    println!("{:?}", ind);
}
