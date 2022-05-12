use mockall::automock;

#[automock]
pub trait Transactions {
    fn deposit(&self, amount: i32) -> ();
}
