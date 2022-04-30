use crate::Story;
use std::collections::BTreeMap;

pub struct Database {
    pub stories: BTreeMap<&'static str, Story<'static>>,
}

impl Default for Database {
    fn default() -> Self {
        Self { stories: Self::story_id() }
    }
}

impl Database {
    pub fn search_by_story_id(&self, id: &str) -> Option<&Story<'static>> {
        let mut key = id;
        if  { }

        self.stories.get(id)
    }
}
