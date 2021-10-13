use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    sync::Arc,
};

type Ident = Arc<str>;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
pub struct Pos {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(PartialEq, Eq, Hash)]
struct LocationRef {
    file: Ident,
    pos: Pos,
}

impl LocationRef {
    fn location(&self) -> Location {
        Location {
            file: &self.file,
            pos: self.pos,
        }
    }
}

#[derive(Debug)]
pub struct Location<'a> {
    pub file: &'a str,
    pub pos: Pos,
}

impl<'a> Location<'a> {
    pub fn new(file: &'a str, pos: Pos) -> Self {
        Self { file, pos }
    }

    fn location_ref(&self, interner: &mut Interner) -> LocationRef {
        LocationRef {
            file: interner.get_ident(self.file),
            pos: self.pos,
        }
    }
}

impl<'a> Display for Location<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{}:{}", self.file, self.pos.line, self.pos.column)
    }
}

#[derive(Default)]
struct Interner {
    idents: HashSet<Ident>,
}

impl Interner {
    fn get_ident(&mut self, item: &str) -> Ident {
        if let Some(item_ident) = self.idents.get(item) {
            item_ident.clone()
        } else {
            let new: Arc<str> = item.into();
            self.idents.insert(new.clone());
            new
        }
    }
}

#[derive(Default)]
pub struct ReverseIndex {
    interner: Interner,
    items: HashMap<Ident, Vec<LocationRef>>,
}

impl ReverseIndex {
    /// Add a single item to this index, afterwards it can be fetched with the query method
    ///
    /// Note that is does not check for duplicated, so add the same entry twice will also return it
    /// twice when using the query method
    pub fn add_item(&mut self, item: &str, location: Location) {
        let item_ident = self.interner.get_ident(item);
        let entry = self.items.entry(item_ident.clone()).or_default();

        entry.push(location.location_ref(&mut self.interner));
    }

    /// Add multiple items, see add_item for more details
    pub fn add_items<'a>(&mut self, items: impl Iterator<Item = (&'a str, Location<'a>)>) {
        for (item, location) in items {
            self.add_item(item, location)
        }
    }

    /// Query this index for for the item
    ///
    /// The order of the locations returned are the same as the insertion order.
    pub fn query(&self, item: &str) -> impl Iterator<Item = Location> {
        self.items
            .get(item)
            .into_iter()
            .flatten()
            .map(LocationRef::location)
    }
}
