use std::{error, fmt};

///
#[derive(Debug)]
pub enum DistributionError {}

impl fmt::Display for DistributionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl error::Error for DistributionError {}
