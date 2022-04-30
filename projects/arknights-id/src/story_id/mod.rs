mod data;
use crate::database::Database;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Copy, Clone, Debug)]
pub struct Story<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub code: &'a str,
    pub cash: usize,
    pub exp: usize,
}

impl<'a> Story<'a> {
    pub fn new(id: &'a str, name: &'a str, code: &'a str, cash: usize, exp: usize) -> Self {
        Self { id, name, code, cash, exp }
    }
}
