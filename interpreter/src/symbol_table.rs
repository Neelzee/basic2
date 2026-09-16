use std::{collections::HashMap};

pub type Key<'a> = (usize, &'a str);

#[derive(Debug, Clone)]
pub struct SymbolTable<'a, T: Clone> {
    symbols: HashMap<Key<'a>, T>,
}

impl<'a, T: Clone> SymbolTable<'a, T> {
    pub fn new() -> Self {
        Self {
            symbols: HashMap::new(),
        }
    }

    pub fn lookup(&self, key @ (scope, ident): &'_ Key<'a>) -> Option<&T> {
        match scope {
            0 => self.symbols.get(key),
            _ => self
                .symbols
                .get(key)
                .or_else(|| self.lookup(&(scope - 1, ident))),
        }
    }

    pub fn insert(mut self, key: Key<'a>, value: T) -> Self {
        self.symbols.insert(key, value);
        self
    }
}

impl<'a, T: Clone> IntoIterator for SymbolTable<'a, T> {
    type Item = (Key<'a>, T);
    
    type IntoIter = SymbolTableIter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        SymbolTableIter {
            symbols: self.symbols.into_iter().collect()
        }
    }
}


pub struct SymbolTableIter<'a, T> {
    symbols: Vec<(Key<'a>, T)>,
}

impl<'a, T: Clone> Iterator for SymbolTableIter<'a, T> {
    type Item = (Key<'a>, T);

    fn next(&mut self) -> Option<Self::Item> {
        let mut symbols = self.symbols.clone().into_iter();
        let next = symbols.next();
        self.symbols = symbols.collect();
        next
    }
}