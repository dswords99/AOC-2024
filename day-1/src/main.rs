use std::{collections::HashMap, fs};

fn main() {
    let contents = fs::read_to_string("../input.txt")
        .expect("Should be able to open the file");

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in contents.lines() {
        let (first, second) = line.split_once("   ").expect("has spaces");

        vec1.push(first.parse::<i32>().unwrap());
        vec2.push(second.parse::<i32>().unwrap());
    }

    let mut similarity: HashMap<i32, i32> = HashMap::new();
    let mut count = 0;

    for num in vec1 {
        match similarity.get(&num) {
            Some(v) => count += num * v,
            None => {
                let freq = vec2.iter().filter(|e| **e == num).count() as i32;
                similarity.insert(num, freq);

                count += num * freq;
            },
        }
    }
    
    println!("calculated {count}");
}