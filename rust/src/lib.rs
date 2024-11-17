mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;

#[cfg(test)]
mod tests {
    pub fn readfile_content_to_string_list(file_path: &str) -> Vec<String> {
        let content = std::fs::read_to_string(file_path).unwrap();
        let lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
        lines
    }
}