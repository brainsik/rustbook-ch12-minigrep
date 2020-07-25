use std::env;
use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    for line in results {
        println!("{}", line)
    }

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    for line in contents.lines() {
        if line.contains(query) {
            result.push(line);
        }
    }
    result
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();

    let query = query.to_lowercase();
    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            result.push(line);
        }
    }
    result
}

pub struct Config {
    query: String,
    filename: String,
    case_sensitive: bool,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
        if args.len() < 3 {
            return Err(format!("Usage: {} <query> <filename>", args[0]));
        }
        Ok(Self {
            query: args[1].clone(),
            filename: args[2].clone(),
            case_sensitive: env::var("CASE_INSENSITIVE").is_err(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

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
    fn new_config_case_insensitive_env_var() {
        let args = [
            "binary".to_string(),
            "query".to_string(),
            "filename".to_string(),
        ];

        env::remove_var("CASE_INSENSITIVE");
        let config = Config::new(&args).unwrap();
        assert_eq!(config.case_sensitive, true);

        env::set_var("CASE_INSENSITIVE", "kthx");
        let config = Config::new(&args).unwrap();
        assert_eq!(config.case_sensitive, false);
    }

    #[test]
    #[should_panic(expected = "Usage")]
    fn new_config_not_enough_args() {
        let args = ["binary".to_string()];
        Config::new(&args).unwrap();
    }

    #[test]
    fn run_with_missing_file() {
        let args = [
            "binary".to_string(),
            "query".to_string(),
            "this-file-does-not-exist".to_string(),
        ];
        let config = Config::new(&args).unwrap();

        // how the hell do I convert the Box<Error> to a thing where I can test the Os Error fields!?
        // for now, just testing that we get any error :-(
        run(config).unwrap_err();
    }

    #[test]
    fn run_with_empty_file() {
        let tmpfile = NamedTempFile::new().unwrap();
        let filename = tmpfile.path().to_str().unwrap().to_string();

        let args = ["binary".to_string(), "query".to_string(), filename];
        let config = Config::new(&args).unwrap();
        run(config).unwrap();
    }

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn case_insensitive() {
        let query = "rUsT";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        );
    }
}
