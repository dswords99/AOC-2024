use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt")
        .expect("path is correct");

    let mut reports: Vec<Vec<i32>> = Vec::new();

    for report_str in contents.lines() {
        let num_strs = report_str.split(' ');

        let mut report = Vec::new();
        for num in num_strs {
            report.push(num.parse::<i32>().unwrap());
        }

        reports.push(report);
    }

    let count = reports.into_iter().filter(|v| is_safe(v)).count();

    println!("calculated {count}");
}


fn is_safe(v: &Vec<i32>) -> bool {

    let incr = if v[0] < v[1] {
        true
    } else {
        false
    };

    let mut iter = v.into_iter().peekable();
    let mut prev = None;
    let mut unsafe_count = 0;

    while let Some(curr) = iter.next() {
        let prev_it: &i32 = match prev {
            Some(x) => x,
            None => {
                check_first_last(incr, *curr, **iter.peek().expect("first iter"), &mut unsafe_count);
                prev = Some(curr);
                continue;
            },
        };

        let next = match iter.peek() {
            Some(x) => x,
            None => {
                check_first_last(incr, *prev_it, *curr, &mut unsafe_count);
                continue;
            },
        };

        match incr {
            true => {
                if *curr >= **next || (*curr - **next).abs() > 3 {
                    // can it be removed
                    if *prev_it >= **next || (prev_it - *next).abs() > 3 {
                        // println!("added unsafe with prev {prev_it} curr {curr} next {next}");
                        return false;
                    } else {
                        // println!("added unsafe with prev {prev_it} curr {curr} next {next}");
                        unsafe_count += 1;
                    }

                }
            },
            false => if *curr <= **next || (*curr - **next).abs() > 3 {
                // can it be removed
                if *prev_it <= **next || (*prev_it - *next).abs() > 3 {
                    // println!("added unsafe with prev {prev_it} curr {curr} next {next}");
                    return false;
                } else {
                    // println!("added unsafe with prev {prev_it} curr {curr} next {next}");
                    unsafe_count += 1;
                }

            },
        }

        if unsafe_count > 1 {
            return false;
        }

        prev = Some(curr);
    }


    true
}

fn check_first_last(incr: bool, curr: i32, next: i32, unsafe_count: &mut i32) {
    match incr {
        true => {
            if curr >= next || (curr - next).abs() > 3 {
                *unsafe_count += 1;
            }
        },
        false => if curr <= next || (curr - next).abs() > 3 {
            *unsafe_count += 1;
        },
    }
}