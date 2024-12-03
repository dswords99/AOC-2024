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

    let mut prev = None;

    for num in v {
        let prev_num = match prev {
            Some(x) => x,
            None => {
                prev = Some(num);
                continue;
            },
        };

        match incr {
            true => {
                if prev_num >= num || (prev_num - num).abs() > 3 {
                    return false;
                }
            },
            false => {
                if prev_num <= num || (prev_num - num).abs() > 3 {
                    return false;
                }
            },
        }

        prev = Some(num);
    }


    true
}