use std::collections::HashMap;

pub type Key<'a> = (usize, &'a str);
#[derive(Debug, Clone)]
pub struct SymbolTable<'a, T> {
    symbols: HashMap<usize, HashMap<&'a str, T>>,
}

impl<'a, T> SymbolTable<'a, T> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn lookup(&self, (scope, ident): Key<'a>) -> Option<&T> {
        match scope {
            0 => self.symbols.get(&scope).and_then(|st| st.get(ident)),
            _ => self
                .symbols
                .get(&scope)
                .and_then(|st| st.get(ident))
                .or_else(|| self.lookup((scope - 1, ident))),
        }
    }

    pub fn insert(mut self, (scope, ident): Key<'a>, value: T) -> Self {
        self.symbols.entry(scope).or_default().insert(ident, value);
        self
    }

    pub fn drop_scope(mut self, scope: usize) -> Self {
        self.symbols.remove(&scope);
        self
    }
}
