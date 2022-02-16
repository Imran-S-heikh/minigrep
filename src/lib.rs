//! # Minigrep
//!
//! `minigrep` is a package for searching text in file, similar like grep

use std::{env, error::Error, fs};

pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new(mut args: env::Args) -> Result<Config, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file name"),
        };

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config {
            query,
            filename,
            case_sensitive,
        })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(&config.filename)?;
    let search = if config.case_sensitive {
        search_case_sensitive
    } else {
        search_case_insensitive
    };

    for line in search(&config.query, &contents) {
        println!("{line}")
    }

    Ok(())
}

/// Case Sensitive Search and Return the whole line.
///
/// # Examples
///
/// ```
/// let query = "alone";
/// let contents = "I am not alone in dark";
/// let result = minigrep::search_case_sensitive(query,contents);
///
/// assert_eq!(vec!["I am not alone in dark"],result);
/// ```

pub fn search_case_sensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {

    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Case Insensitive Search and Return the whole line.
///
/// # Examples
///
/// ```
/// let query = "Alone";
/// let contents = "I am not alone in dark";
/// let result = minigrep::search_case_insensitive(query,contents);
///
/// assert_eq!(vec!["I am not alone in dark"],result);
/// ```

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();

    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(query.as_str()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "safe, fast, productive.";
        let contents = "\
        Rust:
safe, fast, productive.
safe, fast, Productive.
        Pick three.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search_case_sensitive(query, contents)
        );
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
