
mod atgc;
mod sequence;
mod loader;
mod pattern;
mod config;

use std::thread;
use config::Config;
use pattern::Pattern;


fn main()
{
    let config: Config = Config::get_config("config.txt");
    let mut sequences_full: Vec<sequence::Sequence> = loader::read_file(&config.filepath);
    // let mut patterns: Vec<Pattern>;
    let items_to_process: usize;
    if config.items_to_process == 0
    {items_to_process = sequences_full.len();}
    else 
    {items_to_process = config.items_to_process;}
    println!("Amount of items to process: {}", items_to_process);
    let mut sequences: Vec<sequence::Sequence> = sequences_full.drain(0..items_to_process).collect();
    let sequences_copy = sequences.to_vec();
    println!("comparing sequences:");
    let mut patterns_output: Vec<Pattern> = Vec::with_capacity(2000);
    while sequences.len() > 0
    {
        let mut group: Vec<sequence::Sequence>;
        if sequences.len() > config.comparison_group_size
        {
            group = sequences.split_off(sequences.len()-config.comparison_group_size);
        }
        else
        {
            group = sequences.drain(..).collect();
        }
        let seq_group: Vec<sequence::Sequence> = group.drain(..).collect();
        patterns_output.append(&mut thread::spawn(move ||
        {
            let mut patterns: Vec<Pattern> = Vec::with_capacity(100000);
            for seq_n in 0..seq_group.len()
            {
                for target_n in 0..seq_group.len()
                {
                    seq_group[seq_n].compare(&mut patterns, &seq_group[target_n], config.minimal_similarity)
                }
            }
            for pattern in patterns.iter_mut()
            {
                for seq in seq_group.iter()
                {
                    if !pattern.sequences.contains(&seq.id)
                    {
                        if pattern.template.apply_template(&seq.content)
                        {
                            pattern.sequences.push(seq.id);
                        }
                    }
                }
            }
            patterns.sort_by_key(|pattern| pattern.sequences.len());
            print!(".");
            patterns.drain((&patterns.len()-50)..).collect::<Vec<Pattern>>()
        }).join().unwrap())
    }
    println!("Processing groups finished. Counting resulting patterns:");
    for pattern in patterns_output.iter_mut()
    {
        for seq_id in 0..items_to_process
        {
            if !pattern.sequences.contains(&seq_id)
            {
                if pattern.template.apply_template(&sequences_copy[seq_id].content)
                {
                    pattern.sequences.push(seq_id);
                }
            }
        }
    }
    patterns_output.sort_by_key(|pattern| pattern.sequences.len());
    for pattern in patterns_output.iter()
    {
        println!("{}  counter: {}", pattern.template.into_string(), pattern.sequences.len());
    }
    // for seq_id in 0..items_to_process
    // {
    //     for target_id in 0..items_to_process
    //     {
    //         sequences[seq_id].compare(&mut patterns, &sequences[target_id], config.minimal_similarity)
    //     }
    //     print!(".");
    // }
    // println!("");
    // println!("counting patterns:");
    // for pattern in patterns.iter_mut()
    // {
    //     for seq_id in 0..items_to_process
    //     {
    //         if !pattern.sequences.contains(&seq_id)
    //         {
    //             if pattern.template.apply_template(&sequences[seq_id].content)
    //             {
    //                 pattern.sequences.push(seq_id);
    //             }
    //         }
    //     }
    // }
    // patterns.sort_by_key(|pattern| pattern.sequences.len());
    // for pattern in patterns.iter()
    // {
    //     if pattern.sequences.len() > 0
    //     {
    //         println!("{}  counter: {}", pattern.template.into_string(), pattern.sequences.len());
    //     }
    // }
    // println!("Amount of patterns: {}", patterns.len());
    println!("Completed.");
    std::io::stdin().read_line(&mut String::new()).unwrap();
}
