#[cfg(test)]
mod tests {
    use crate::tests::readfile_content_to_string_list;

    #[test]
    fn day_1_step_1() {
        let mut total: i32 = 0;
        let lines = readfile_content_to_string_list("./src/data/day_1");
        for line in lines {
            let mut numbers: Vec<i32> = Vec::new();
            for c in line.chars() {
                if c.is_digit(10) {
                    let number: i32 = c.to_string().parse().unwrap();
                    numbers.push(number);
                }
            }
            let value1 = numbers[0].to_string().to_owned();
            let value2 = numbers[numbers.len() - 1].to_string().to_owned();
            let result: i32 = (value1 + &value2).parse().unwrap();
            total += result;
        }
        println!("day 1 step 1 : {}", total);
    }

    #[test]
    fn day_1_step_2() {
        let mut total: i32 = 0;
        let lines: Vec<String> = readfile_content_to_string_list("./src/data/day_1");
        let string_numbers = vec![(0, "zero"), (1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five"), (6, "six"), (7, "seven"), (8, "eight"), (9, "nine")];
        let mut line_string_buffer = String::new();
        for line in lines {
            let mut numbers: Vec<i32> = Vec::new();
            for c in line.chars() {
                line_string_buffer.push(c);
                if c.is_digit(10) {
                    let number: i32 = c.to_string().parse().unwrap();
                    numbers.push(number);
                }
                // iterate over numbers like python for item in numbers

                for (value, str_number) in string_numbers.iter() {
                    if line_string_buffer.ends_with(str_number) {
                        numbers.push(*value);
                    }
                }
            }
            let value1 = numbers[0].to_string().to_owned();
            let value2 = numbers[numbers.len() - 1].to_string().to_owned();
            let result: i32 = (value1 + &value2).parse().unwrap();
            total += result;
        }
        println!("day 1 step 2 : {}", total);
    }
}

