use std::{env, error::Error, fs};

pub struct Config {
    pub file_path: String,
    pub query: String,
    pub case_insensitive: bool,
}

impl Config {
    pub fn new(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();
        let query = match args.next() {
            Some(query) => query,
            None => return Err("Not enough arguments"),
        };

        let file_path = match args.next() {
            Some(file_path) => file_path,
            None => return Err("Not enough arguments"),
        };

        if args.next().is_some() {
            return Err("Too many arguments");
        }

        let case_insensitive = env::var("CASE_INSENSITIVE").is_ok();

        Ok(Config {
            query,
            file_path,
            case_insensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;
    let results = match config.case_insensitive {
        true => search_insensitive(&config.query, &contents),
        false => search(&config.query, &contents),
    };

    results.iter().for_each(|line| println!("{}", line));

    Ok(())
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

fn search_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_creation() {
        let args = vec![
            "minigrep".to_string(),
            "safe".to_string(),
            "poem.txt".to_string(),
        ]
        .into_iter();

        let config = Config::new(args).unwrap();

        assert_eq!(config.file_path, "poem.txt");
        assert_eq!(config.query, "safe");
    }

    #[test]
    fn config_not_enough_args() {
        let args = vec!["minigrep".to_string(), "safe".to_string()].into_iter();

        let err = Config::new(args).err().unwrap();
        assert_eq!(err, "Not enough arguments")
    }

    #[test]
    fn config_to_many_args() {
        let args = vec![
            "minigrep".to_string(),
            "safe".to_string(),
            "poem.txt".to_string(),
            "any thing".to_string(),
        ]
        .into_iter();

        let err = Config::new(args).err().unwrap();
        assert_eq!(err, "Too many arguments")
    }

    #[test]
    fn search_case_sensitive() {
        let query = "duct";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }

    #[test]
    fn search_case_insensitive() {
        let query = "rUsT";
        let contents = "Rust:\nsafe, fast, productive.\nPick three.\nTrust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_insensitive(query, contents)
        );
    }
}
