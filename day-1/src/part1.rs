use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("Should be able to open the file");

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    for line in contents.lines() {
        let (first, second) = line.split_once("   ").expect("has spaces");

        vec1.push(first.parse::<i32>().unwrap());
        vec2.push(second.parse::<i32>().unwrap());
    }

    vec1.sort();
    vec2.sort();

    let zip = vec1.into_iter().zip(vec2.into_iter());

    let mut count = 0;

    for tup in zip {
        count += (tup.0 - tup.1).abs();
    }
    

    println!("calculated {count}");
}
