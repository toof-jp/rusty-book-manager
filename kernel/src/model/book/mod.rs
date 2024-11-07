use chrono::{DateTime, Utc};

use crate::model::{
    id::{BookId, CheckoutId},
    user::{BookOwner, CheckoutUser},
};

pub mod event;

#[derive(Debug)]
pub struct Book {
    pub id: BookId,
    pub title: String,
    pub author: String,
    pub isbn: String,
    pub description: String,
    pub owner: BookOwner,
    pub checkout: Option<Checkout>,
}

#[derive(Debug)]
pub struct BookListOptions {
    pub limit: i64,
    pub offset: i64,
}

#[derive(Debug)]
pub struct Checkout {
    pub checkout_id: CheckoutId,
    pub checked_out_by: CheckoutUser,
    pub checked_out_at: DateTime<Utc>,
}
