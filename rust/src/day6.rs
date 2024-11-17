#[cfg(test)]
mod tests {
    use crate::tests::readfile_content_to_string_list;

    fn line_to_int(line: &String) -> Vec<i32> {
        let str_line = line.split_whitespace().collect::<Vec<&str>>();
        str_line[1..].iter().map(|s| s.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }

    fn line_to_int_value(line: &String) -> i64 {
        let str_line = line.split_whitespace().collect::<Vec<&str>>();
        let joined = str_line[1..].join("");
        joined.parse::<i64>().unwrap()
    }

    #[test]
    fn day_6_step_1() {
        let input = readfile_content_to_string_list("./src/data/day6");
        let times = line_to_int(&input[0]);
        let distances = line_to_int(&input[1]);

        let entries_len = times.len();
        let mut result = 1;
        for i in 0..entries_len {
            let time = times[i];
            let distance = distances[i];
            // println!("time: {}, distance: {}", time, distance);
            let mut solution_count = 0;
            for push_duration in 0..time {
                let remaining_duration  = time - push_duration;
                let total_distance = remaining_duration * push_duration;

                if total_distance > distance {
                    solution_count+=1;
                }
            }
            result *= solution_count;
            // println!("solution_count for race {} : {}", i, solution_count);
        }
        println!("day 6 step 1 : {}", result);

    }

    #[test]
    fn day_6_step_2() {
        let input = readfile_content_to_string_list("./src/data/day6");

        let time  = line_to_int_value(&input[0]);
        let distance = line_to_int_value(&input[1]);
        // println!("time: {}, distance: {}", time, distance);
        let mut solution_count = 0;
        let mut result = 1;
        for push_duration in 0..time {
            let remaining_duration  = time - push_duration;
            let total_distance = remaining_duration * push_duration;

            if total_distance > distance {
                solution_count+=1;
            }
        }
        result *= solution_count;
        println!("day 6 step 2 : {}", result);
    }
}

