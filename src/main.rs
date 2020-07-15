use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::new(&args).unwrap_or_else(|e| {
        println!("{}", e);
        process::exit(1);
    });

    println!("Searching for {}", config.query);
    println!("In the file {}", config.filename);

    let contents =
        fs::read_to_string(config.filename).expect("Something went wrong reading the file");
    println!("With the text:\n{}", contents);
}

struct Config {
    query: String,
    filename: String,
}

impl Config {
    fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Usage: {} <query> <filename>", args[0]));
        }
        Ok(Self {
            query: args[1].clone(),
            filename: args[2].clone(),
        })
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
        let config = Config::new(&args).unwrap();
        assert_eq!(config.query, "query");
        assert_eq!(config.filename, "filename");
    }

    #[test]
    #[should_panic(expected = "Usage")]
    fn new_config_not_enough_args() {
        let args = ["binary".to_string()];
        Config::new(&args).unwrap();
    }
}
