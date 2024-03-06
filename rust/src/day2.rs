#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use crate::tests::readfile_content_to_string_list;


    struct Game {
        id: i32,
        colors: HashMap<String, i32>,
    }

    fn parse_game() -> Vec<Game> {
        let mut games: Vec<Game> = Vec::new();
        let lines = readfile_content_to_string_list("./src/data/day_2");
        for line in lines.iter() {
            let mut game = Game { id: 1, colors: HashMap::new() };
            let mut game_info = line.split(":");
            let game_id = game_info.next().unwrap().replace("Game ", "");
            let game_colors_str = game_info.next().unwrap();
            let game_colors = game_colors_str.split(";");
            for color in game_colors {
                let color_token = color.split(",");
                for token in color_token {
                    let mut value_color = token.trim().split(" ");
                    let value: i32 = value_color.next().unwrap().parse().unwrap();
                    let key = value_color.next().unwrap();
                    if !game.colors.contains_key(&key.to_string()) {
                        game.colors.insert(key.to_string(), 0);
                    }
                    if game.colors.get(key).unwrap_or(&0) < &value {
                        game.colors.insert(key.to_string(), value);
                    }
                }
            }
            game.id = game_id.parse().unwrap();
            games.push(game);
        }
        return games;
    }

    #[test]
    fn day_2_step_1() {
        let games = parse_game();

        let mut result: i32 = 0;
        for game in games.iter() {
            let mut ok = true;
            if game.colors["green"] > 13 {
                ok = false;
            }
            if game.colors["red"] > 12 {
                ok = false;
            }
            if game.colors["blue"] > 14 {
                ok = false;
            }
            if ok {
                result += game.id;
            }
        }
        println!("day 2 step 1 : {}", result);
    }

    #[test]
    fn day_2_step_2() {
        let games = parse_game();
        let mut result: i32 = 0;
        for game in games.iter() {
            result += game.colors["green"] * game.colors["red"] * game.colors["blue"];
        }

        println!("day 2 step 2 : {}", result);
    }
}

