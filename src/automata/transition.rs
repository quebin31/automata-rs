#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Transition {
    symbol: String,
    end_state: usize,
}

impl Transition {
    pub fn new(symbol: &str, end: usize) -> Self {
        Self {
            symbol: symbol.to_owned(),
            end_state: end,
        }
    }

    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    pub fn end_state(&self) -> usize {
        self.end_state
    }
}
