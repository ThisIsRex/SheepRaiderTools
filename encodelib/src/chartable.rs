use std::borrow::Cow;
use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use serde_json::Result;

#[derive(Serialize, Deserialize)]
pub struct CharTable {
    pub replace_table: HashMap<char, char>,
}
impl CharTable {
    pub fn replace_letters<'a>(&self, input: &'a str) -> Cow<'a, str> {
        if !input.is_empty() {
            return input
                .chars()
                .map(|c| match self.replace_table.get(&c) {
                    None => c,
                    Some(r) => *r,
                })
                .collect::<String>()
                .into();
        }

        input.into()
    }

    pub fn from_json(json: &str) -> Result<CharTable> {
        serde_json::from_str(json)
    }
}
