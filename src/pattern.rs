use crate::{atgc::ATGC};



pub struct Pattern
{
    pub template: ATGC,
    pub sequences: Vec<usize>,
}


impl Pattern
{
    pub fn new(template: ATGC) -> Self
    {
        Self
        {
            template: template,
            sequences: Vec::with_capacity(2),
        }
    }
    
}