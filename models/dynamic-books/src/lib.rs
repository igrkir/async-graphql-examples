mod model;
use async_graphql::ID;

use futures_util::lock::Mutex;
pub use model::schema;
use slab::Slab;
use std::{collections::HashMap, sync::Arc};

type Storage = Arc<Mutex<Slab<Book>>>;
type Mapping = Arc<Mutex<HashMap<String, usize>>>;

#[derive(Clone)]
pub struct Book {
    id: ID,
    name: String,
    author: String,
}

#[derive(Clone)]
pub struct BookStore {
    store: Storage,
    books_by_id: Mapping,
    value: usize,
}
impl BookStore {
    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        let mut store = Slab::new();
        let key_1 = store.insert(Book {
            id: "10".into(),
            name: "Luke Skywalker".to_string(),
            author: "Tatooine".to_string(),
        });
        let key_2 = store.insert(Book {
            id: 1001.into(),
            name: "Anakin Skywalker".to_string(),
            author: "Tatooine".to_string(),
        });

        let mut books_by_id = HashMap::new();
        books_by_id.insert("10".to_string(), key_1);
        books_by_id.insert("1001".to_string(), key_2);

        Self {
            store: Arc::new(Mutex::new(store)),
            books_by_id: Arc::new(Mutex::new(books_by_id)),
            value: 10,
        }
    }

    // pub fn get_book_id(&self, id: &str) -> Option<usize> {
    //     self.books_by_id.get(id).copied()
    // }
}
