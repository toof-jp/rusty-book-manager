use kernel::model::{
    book::Book,
    id::{BookId, UserId},
    user::BookOwner,
};

pub struct BookRow {
    pub book_id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owned_by: UserId,
    pub owner_name: String,
}

impl From<BookRow> for Book {
    fn from(value: BookRow) -> Self {
        let BookRow {
            book_id,
            title,
            author,
            isbn,
            description,
            owned_by,
            owner_name,
        } = value;
        Self {
            id: book_id,
            title,
            author,
            isbn,
            description,
            owner: BookOwner {
                id: owned_by,
                name: owner_name,
            },
        }
    }
}

pub struct PaginatedBookRow {
    pub total: i64,
    pub id: BookId,
}
