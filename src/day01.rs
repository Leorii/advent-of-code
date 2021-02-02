// pub fn a(_input: Vec<String>) -> String {
//     "INCOMPLETE".into()
// }

// pub fn b(_input: Vec<String>) -> String {
//     "INCOMPLETE".into()
// }

pub fn a(input: Vec<String>) -> String {
    for i in 0..input.len() {
        let x = to_i32(&input[i]);

        for j in (i + 1)..input.len() {
            let y = to_i32(&input[j]);

            if x + y == 2020 {
                return format!("{}", x * y);
            }
        }
    }
    "[ERROR]: Could not find answer!".into()
}

pub fn b(input: Vec<String>) -> String {
    for i in 0..input.len() {
        let x = to_i32(&input[i]);

        for j in i + 1..input.len() {
            let y = to_i32(&input[j]);

            if x + y < 2020 {
                for k in j + 1..input.len() {
                    let z = to_i32(&input[k]);

                    if x + y + z == 2020 {
                        return format!("{}", x * y * z);
                    }
                }
            }
        }
    }
    "[ERROR]: Could not find answer!".into()
}

fn to_i32(num: &str) -> i32 {
    str::parse::<i32>(num).unwrap()
}
