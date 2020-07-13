use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args);

    println!("Searching for {}", config.query);
    println!("In the file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With the text:\n{}", contents)
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Config {
        Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_config() {
        let args = [
            "binary".to_string(),
            "query".to_string(),
            "filename".to_string(),
        ];
        let config = Config::new(&args);
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }
}
