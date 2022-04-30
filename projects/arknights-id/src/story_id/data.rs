use super::*;

impl Database {
    pub fn story_id() -> BTreeMap<&'static str, Story<'static>> {
        BTreeMap::from_iter([("", Story::new("", "", "", ""))].into_iter())
    }
}
