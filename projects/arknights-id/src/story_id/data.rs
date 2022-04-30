
use super::*;

impl Story<'static> {
    pub fn database() -> BTreeMap<&'static str, Story<'static>> {
        BTreeMap::from_iter([
            ("", Self::new("", "", "", ""))
        ].into_iter())
    }
}