use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
    thread::LocalKey,
};

type Ident = Arc<str>;

#[derive(PartialEq, Eq, Hash)]
struct LocationRef {
    file: Ident,
    offset: usize,
}

impl LocationRef {
    fn location(&self) -> Location {
        Location {
            file: &self.file,
            offset: self.offset,
        }
    }
}

#[derive(Debug)]
pub struct Location<'a> {
    pub file: &'a str,
    pub offset: usize,
}

#[derive(Default)]
pub struct ReverseIndex {
    idents: HashSet<Ident>,
    items: HashMap<Ident, HashSet<LocationRef>>,
}

impl ReverseIndex {
    fn get_ident(&mut self, item: &str) -> Ident {
        if let Some(item_ident) = self.idents.get(item) {
            item_ident.clone()
        } else {
            let new: Arc<str> = item.into();
            self.idents.insert(new.clone());
            new
        }
    }

    pub fn add_item(&mut self, item: &str, file: &str, offset: usize) {
        let item_ident = self.get_ident(item);
        let file_ident = self.get_ident(file);

        let entry = self.items.entry(item_ident.clone()).or_default();

        entry.insert(LocationRef {
            file: file_ident,
            offset,
        });
    }

    pub fn add_items<'a>(&mut self, items: impl Iterator<Item = (&'a str, usize)>, file: &str) {
        for (item, offset) in items {
            self.add_item(item, file, offset)
        }
    }

    pub fn query(&self, item: &str) -> impl Iterator<Item = Location> {
        self.items
            .get(item)
            .into_iter()
            .flatten()
            .map(LocationRef::location)
    }
}
