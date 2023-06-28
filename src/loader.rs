use std::{fs::File, io::{BufReader, BufRead}};
use crate::{atgc::ATGC, sequence::Sequence};


pub fn read_lines(filename: &str) -> BufReader<File>
{
    let file = File::open(filename).expect("No file for me");
    BufReader::new(file)
}

pub fn read_file(filepath: &str) -> Vec<Sequence>
{
    let mut sequences: Vec<Sequence> = Vec::with_capacity(30000);
    let mut counter: u8 = 0;
    let mut name: String = "".to_string();
    let mut content: String;
    for line in read_lines(filepath).lines()
    {
        match line
        {
            Ok(text) =>
            {
                if counter == 0
                {
                    name = text;
                }
                else
                {
                    content = text;
                    sequences.push(Sequence::new(sequences.len(), name.clone(), ATGC::from_string(content)));
                }
            },
            Err(e) => println!("File cannot be read: {e:?}"),
        }
        counter = 1 - counter;
    }

    return sequences;
}

