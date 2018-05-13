extern crate crypto;
use crypto::sha3::Sha3;
use std::vec::Vec;

#[derive(Debug)]
struct Fig {
    previous_hash: String,
    changes:       Vec<String>,
    hash:          String
}

impl Fig {
    fn previous_hash(&self) -> &str {
        &self.previous_hash
    }

    fn hash(&self) -> &str {
        &self.hash
    }
}

pub fn new(previous_block: Option<&Fig>, changes: Vec<String>) -> Fig {
    previous_hash = match previous_block {
        Some(fig) => fig.previous_hash(),
        None      => String::from("0")
    };
    let joined_str = format!("{}{}",changes.join(""),previous_hash);
    let mut hasher = Sha3::sha3_512();
    hasher.input_str(joined_str);
    
    Fig { previous_hash: previous_hash, hash: hasher.result_str(), changes: changes }
}
