use std::error::Error;
use std::fs;

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    println!("Searching for {}", config.query);
    println!("In the file {}", config.filename);

    let contents = fs::read_to_string(config.filename)?;
    println!("With the text:\n{}", contents);
    Ok(())
}

pub struct Config {
    query: String,
    filename: String,
}

impl Config {
    pub fn new(args: &[String]) -> Result<Config, String> {
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
}
