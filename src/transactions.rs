use mockall::automock;

#[automock]
pub trait Transactions {
    fn deposit(&self, amount: i32) -> ();
    fn withdraw(&self, amount: i32) -> ();
    fn list(&self) -> Vec<Transaction>;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Transaction {
    pub amount: i32,
}
