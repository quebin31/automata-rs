use std::collections::BTreeSet;

pub type Set<T> = BTreeSet<T>;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct State {
    tags: Set<String>,
}

impl State {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn tags(&self) -> &Set<String> {
        &self.tags
    }
}

impl From<Set<String>> for State {
    fn from(set: Set<String>) -> State {
        State { tags: set }
    }
}

impl From<&[&str]> for State {
    fn from(set: &[&str]) -> State {
        State {
            tags: set.iter().map(|s| s.to_owned().to_owned()).collect(),
        }
    }
}

impl From<&str> for State {
    fn from(tag: &str) -> State {
        let mut tags = Set::new();
        tags.insert(tag.to_owned());

        State { tags }
    }
}

impl From<String> for State {
    fn from(tag: String) -> State {
        let mut tags = Set::new();
        tags.insert(tag);

        State { tags }
    }
}
