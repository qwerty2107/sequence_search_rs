use crate::loader::read_lines;
use std::io::BufRead;


pub struct Config
{
    pub filepath: String,
    pub minimal_similarity: usize,
    pub items_to_process: usize,
}

impl Config
{
    pub fn get_config(config_path: &str) -> Self
    {
        let mut counter: u8 = 0;
        let mut output: String;
        let mut filepath: String = "".to_string();
        let mut minimal_similarity: usize = 0;
        let mut items_to_process: usize = 0;
        for line in read_lines(config_path).lines()
        {
            match line
            {
                Ok(text) =>
                {
                    output = text.split(':').nth(1).unwrap().to_string();
                    match counter
                    {
                        0 => {filepath = output;},
                        1 => {minimal_similarity = output.parse().unwrap()},
                        2 => {items_to_process = output.parse().unwrap()},
                        _ => panic!("Too many lines in config")
                    }
                    counter += 1;
                },
                Err(e) => println!("Options file cannot be read: {e:?}"),
            }
        }
        Self
        {
            filepath: filepath,
            minimal_similarity: minimal_similarity,
            items_to_process: items_to_process,
        }
    }
}