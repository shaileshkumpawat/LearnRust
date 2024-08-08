// Given a string s, find the length of the longest substring without repeating characters.

use std::{char, collections::{HashMap, HashSet}, hash::Hash};

pub fn length_of_longest_substring(s: String) -> i32 {
    let mut current_len = 0;    // longest length of the substring at the current position
    let mut max_len = 0;
    let mut char_set:HashMap<char, usize> = HashMap::new();
    for (i, s) in s.chars().enumerate() {
        if !(char_set.contains_key(&s)) {
            char_set.insert(s, i);
            current_len += 1;
            println!("the current len at {} is {}", &s, current_len);
        }
        else {
            // here if "i-val" represent distance between the present occurence of 's' and the previous occurence of 's'
            // if the distance is greater than the current length of the substring, that means we have a situation like 'abba' where at the last 'a' we should have the
            // previous length to be 1, but the distance between 'a' and 'a' is 3. In this case we just increment current_len by +1, because we only need to incude
            // the current instance of 'a'
            match char_set.get(&s) {Some(val) => if (i-val) > current_len {current_len += 1;} else {current_len = i - val;}, None => current_len += 0};
            println!("the current len at {} is {}", &s, current_len);
            char_set.remove(&s);
            char_set.insert(s, i);
        }
        if (current_len > max_len) {max_len = current_len;}
    }
    max_len as i32
}

fn main() {
    // let str = String::from("abcabdeab");
    let str = String::from("abbabc");
    let len = length_of_longest_substring(str);
    println!("{}", len);
}
