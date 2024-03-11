#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::tests::readfile_content_to_string_list;


    fn split_item(text: &str, separator: &str, nth: usize) -> String {
        text.split(separator).collect::<Vec<&str>>()[nth].to_string()
    }

    fn str_to_int_list(text: &str) -> Vec<i32> {
        text.trim().split_whitespace().map(|x| x.parse::<i32>().unwrap()).collect::<Vec<i32>>()
    }

    fn parse_lines(line: &str) -> (Vec<i32>, Vec<i32>) {
        let line_numbers = split_item(&*line, ":", 1);
        let winning_numbers = split_item(&*line_numbers, "|", 0);
        let owned_numbers = split_item(&*line_numbers, "|", 1);
        let int_winning_numbers = str_to_int_list(&*winning_numbers);
        let int_owned_numbers = str_to_int_list(&*owned_numbers);
        (int_winning_numbers, int_owned_numbers)
    }

    #[test]
    fn day_4_step_1() {
        let input = readfile_content_to_string_list("./src/data/day4");
        let mut score = 0;
        for line in input {
            let (int_winning_numbers, int_owned_numbers) = parse_lines(&*line);
            let mut line_score = 0;
            for number in int_owned_numbers {
                if int_winning_numbers.contains(&number) {
                    if line_score == 0 {
                        line_score = 1;
                    } else {
                        line_score *= 2;
                    }
                }
            }
            score += line_score;
        }
        println!("day 4 step 1 : {}", score);
    }

    #[test]
    fn day_4_step_2() {
        let input = readfile_content_to_string_list("./src/data/day4");
        let mut card_copies: HashMap<i32, i32> = std::collections::HashMap::new();
        let mut total_cards = 0;
        card_copies.insert(1, 1);
        for line in input {
            let game_id: i32 = line.split(":").nth(0).unwrap().split_whitespace().nth(1).unwrap().parse().unwrap();
            let (int_winning_numbers, int_owned_numbers) = parse_lines(&*line);
            let mut game_matching_count = 0;

            for number in int_owned_numbers {
                if int_winning_numbers.contains(&number) {
                    game_matching_count += 1;
                }
            }
            let current_game_copies_count = *card_copies.get(&game_id).unwrap_or_else(|| &1);
            for count in game_id..game_id + game_matching_count {
                let previous_value = card_copies.get(&(count + 1)).unwrap_or(&1).clone();
                let next_value = previous_value + current_game_copies_count;
                card_copies.insert(count + 1, next_value);
            }
            total_cards += card_copies.get(&game_id).unwrap_or(&1);
        }
        println!("day 4 step 2 : {}", total_cards);
    }
}

