use std::error::Error;
use std::fs;

pub struct Config {
    pub query: String,
    pub file_path: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, String> {
        let args_len = args.len();
        let required_args_len = 3;
        if args_len != required_args_len {
            let err_msg = format!(
                "Unexpected number of arguments. Expected {}, but got {}",
                required_args_len, args_len
            );
            return Err(err_msg);
        }

        let query = args[1].clone();
        let file_path = args[2].clone();

        Ok(Config { query, file_path })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.file_path)?;

    println!("With text:\n{contents}");

    Ok(())
}
