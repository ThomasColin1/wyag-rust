use std::collections::HashMap;
use crate::repo::WyagError;

pub fn kvlm_parse(
    raw: Vec<u8>,
    start: usize,
    dct: Option<HashMap<String, String>>,
) -> Result<HashMap<String, String>, WyagError> {
    let dict: HashMap<String, String> = 
        match dct {
            Some(hashmap) => hashmap,
            None => HashMap::new(),
        };
    let spc: Option<usize> = raw // TODO : commencer à start
        .iter()
        .position(|&b| b == b' ');
    let nl: Option<usize> = raw
        .iter()
        .position(|&b| b == b'\n');

    let condition = match (spc, nl) {
        (_, None) => false,
        (None, Some(_)) => true,
        (Some(s), Some(n)) => n < s,
    };

    if condition {
        assert!(nl.unwrap() == start);
        // dict[None] = raw[start + 1..];
        return dict;
    }

    dict
}
