



pub struct ATGC
{
    content: Vec<u8>,
    length: usize,
}

impl ATGC
{
    pub fn from_string(text: String) -> Self
    {
        let length: usize = text.len();
        let mut output = vec![0; length];
        for n in text.chars().enumerate()
        {
            output[n.0] = match n.1
            {
                '_' => 0,
                'A' => 1,
                'T' => 2,
                'G' => 3,
                'C' => 4,
                _ => panic!("Encountered non-ATGC character in string: {}", text),
            }
        }
        Self {content: output, length: length}
    }
    pub fn into_string(&self) -> String
    {
        self.content.iter().map(|item| Self::into_char(*item)).collect()
    }
    fn into_char(item: u8) -> char
    {
        match item
        {
            0 => '_',
            1 => 'A',
            2 => 'T',
            3 => 'G',
            4 => 'C',
            _ => panic!("This can't happen. Really!")
        }
    }
    fn apply_template_symbol(template: u8, target: u8) -> bool
    {
        match template
        {
            0 => true,
            _ => template == target,
        }
    }
    /// Search this template in another sequence (including reverse) return true if found
    pub fn apply_template(&self, target: &Self) -> bool
    {
        if self.length < target.length
        {
            let mut matches_counter: usize;
            for start_position in 0..(target.length - self.length + 1)
            {
                matches_counter = 0;
                for n in 0..self.length
                {
                    if Self::apply_template_symbol(self.content[n], target.content[n+start_position])
                    {
                        matches_counter += 1;
                        if matches_counter == self.length
                        {
                            return true;
                        }
                    }
                }
                matches_counter = 0;
                for (n, m) in (0..self.length).rev().enumerate()
                {
                    if Self::apply_template_symbol(self.content[n], target.content[m+start_position])
                    {
                        matches_counter += 1;
                        if matches_counter == self.length
                        {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
    /// Strict comparison with no substitutions. (including reverse)
    pub fn compare(&self, target: &Self) -> bool
    {
        if self.length == target.length
        {
            if self.content.iter().eq(target.content.iter())
            {return true;}
            else if self.content.iter().rev().eq(target.content.iter())
            {return true;}
            else
            {return false;}
        }
        else
        {return false;}
    }
    
    /// Finds all similarity patterns between two sequences and outputs a vector of templates
    pub fn find_similarity(&self, target: &Self, minimal_similarity: usize) -> Option<Vec<Self>>
    {
        let mut output: Vec<Self> = Vec::with_capacity(1);
        let mut output_length: usize;
        let mut template: Vec<u8>;
        let mut started_matching: bool;
        let mut matches_written: usize;
        let max_ofset: usize = self.length - minimal_similarity + 1;
        let mut similarity_counter: usize;
        // for ofset in (0..max_ofset).map(|n: usize| (0usize, n)).chain((1..max_ofset).map(|n: usize| (n, 0usize)))
        for ofset in 0..max_ofset
        {
            for (shorter_seq, longer_seq) in [(&self.content, &target.content), (&target.content, &self.content)]
            {
                similarity_counter = 0;
                for n in 0..(self.length-ofset)
                {
                    if shorter_seq[n+ofset] == longer_seq[n]
                    {
                        similarity_counter += 1;
                    }
                }
                if similarity_counter >= minimal_similarity
                {
                    template = Vec::with_capacity(self.length - ofset);
                    started_matching = false;
                    matches_written = 0;
                    for n in 0..(self.length-ofset)
                    {
                        if shorter_seq[n+ofset] == longer_seq[n]
                        {
                            template.push(longer_seq[n]);
                            started_matching = true;
                            matches_written += 1;
                        }
                        else
                        {
                            if matches_written == similarity_counter
                            {
                                break;
                            }
                            else if started_matching
                            {
                                template.push(0);
                            }
                        }
                    }
                    output_length = template.len();
                    output.push(Self{content:template, length:output_length});
                }
                similarity_counter = 0;
                for (n, m) in (0..(self.length-ofset)).rev().enumerate()
                {
                    if shorter_seq[n+ofset] == longer_seq[m]
                    {
                        similarity_counter += 1;
                    }
                }
                if similarity_counter >= minimal_similarity
                {
                    template = Vec::with_capacity(self.length - ofset);
                    started_matching = false;
                    matches_written = 0;
                    for (n, m) in (0..(self.length-ofset)).rev().enumerate()
                    {
                        if shorter_seq[n+ofset] == longer_seq[m]
                        {
                            template.push(longer_seq[m]);
                            started_matching = true;
                            matches_written += 1;
                        }
                        else
                        {
                            if matches_written == similarity_counter
                            {
                                break;
                            }
                            else if started_matching
                            {
                                template.push(0);
                            }
                        }
                    }
                    output_length = template.len();
                    output.push(Self{content:template, length:output_length});
                }
            }
        }
        if output.len() > 0
        {return Some(output);}
        None
    }
}