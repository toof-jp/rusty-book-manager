use async_trait::async_trait;
use shared::error::AppResult;

use crate::model::{
    checkout::{
        event::{CreateCheckout, UpdateReturned},
        Checkout,
    },
    id::{BookId, UserId},
};

#[async_trait]
pub trait CheckoutRepository: Send + Sync {
    async fn create(&self, event: CreateCheckout) -> AppResult<()>;
    async fn update_returned(&self, event: UpdateReturned) -> AppResult<()>;
    async fn find_unreturned_all(&self) -> AppResult<Vec<Checkout>>;
    async fn find_unreturned_by_user_id(&self, user_id: UserId) -> AppResult<Vec<Checkout>>;
    async fn find_history_by_book_id(&self, book_id: BookId) -> AppResult<Vec<Checkout>>;
}
