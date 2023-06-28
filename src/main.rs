
mod atgc;
mod sequence;
mod loader;
mod pattern;
mod config;

use atgc::ATGC;
use config::Config;
use pattern::Pattern;


fn main()
{
    let config: Config = Config::get_config("config.txt");
    let sequences: Vec<sequence::Sequence> = loader::read_file(&config.filepath);
    let mut patterns: Vec<Pattern> = Vec::with_capacity(30000);
    let items_to_process;
    if config.items_to_process == 0
    {items_to_process = sequences.len();}
    else 
    {items_to_process = config.items_to_process;}
    println!("Amount of items to process: {}", items_to_process);
    println!("comparing sequences:");
    for seq_id in 0..items_to_process
    {
        for target_id in 0..items_to_process
        {
            sequences[seq_id].compare(&mut patterns, &sequences[target_id], config.minimal_similarity)
        }
        print!(".");
    }
    println!("");
    println!("counting patterns:");
    for pattern in patterns.iter_mut()
    {
        for seq_id in 0..items_to_process
        {
            if !pattern.sequences.contains(&seq_id)
            {
                if pattern.template.apply_template(&sequences[seq_id].content)
                {
                    pattern.sequences.push(seq_id);
                }
            }
        }
    }
    patterns.sort_by_key(|pattern| pattern.sequences.len());
    for pattern in patterns.iter()
    {
        if pattern.sequences.len() > 5
        {
            println!("{}  counter: {}", pattern.template.into_string(), pattern.sequences.len());
        }
    }
    println!("Amount of patterns: {}", patterns.len());
}
