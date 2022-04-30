mod data;
use crate::database::Database;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Copy, Clone, Debug)]
pub struct Story<'a> {
    pub id: &'a str,
    pub chapter: &'a str,
    pub name: &'a str,
    pub code: &'a str,
}

impl<'a> Story<'a> {
    pub fn new(id: &'a str, chapter: &'a str, name: &'a str, code: &'a str) -> Self {
        Self { id, chapter, name, code }
    }
}
