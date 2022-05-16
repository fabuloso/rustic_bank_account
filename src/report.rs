use mockall::automock;

use crate::transactions::Transaction;

#[automock]
pub trait Report {
    fn print(&self, transactions: Vec<Transaction>);
}
