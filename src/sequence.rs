
use crate::{atgc::ATGC, pattern::Pattern};

pub struct Sequence
{
    pub id: usize,
    pub name: String,
    pub content: ATGC,
}

impl Sequence
{
    pub fn new(id: usize, name: String, content: ATGC) -> Self
    {
        Self
        {
            id: id,
            name: name,
            content: content,
        }
    }
    ///compares two sequences, not templates
    pub fn compare(&self, patterns: &mut Vec<Pattern>, target: &Self, minimal_similarity: usize)
    {
        let mut pattern_already_exists: bool = false;
        if self.id != target.id
        {
            match self.content.find_similarity(&target.content, minimal_similarity)
            {
                None => (),
                Some(templates) =>
                {
                    for template in templates
                    {
                        pattern_already_exists = false;
                        for pattern in patterns.iter()
                        {
                            if template.compare(&pattern.template)
                            {
                                pattern_already_exists = true
                            }
                        }
                        if !pattern_already_exists
                        {
                            let mut new_pattern = Pattern::new(template);
                            new_pattern.sequences.push(self.id);
                            patterns.push(new_pattern);
                        }
                    }
                }
            }
        }
    }
}
