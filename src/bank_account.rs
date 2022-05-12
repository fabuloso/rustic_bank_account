use crate::transactions::Transactions;

pub trait AccountService {
    fn deposit(&self, amount: i32);
    fn withdraw(&self, amount: i32);
    fn print_statement(&self);
}

pub struct BankAccount<'a> {
    transactions: Box<&'a dyn Transactions>,
}

impl AccountService for BankAccount<'_> {
    fn deposit(&self, amount: i32) {
        self.transactions.as_ref().deposit(amount);
    }
    fn withdraw(&self, amount: i32) {
        todo!();
    }
    fn print_statement(&self) {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use crate::{transactions::MockTransactions, AccountService, BankAccount};
    use mockall::predicate::eq;

    #[test]
    fn deposit_100_dollars() {
        let mut transactions = MockTransactions::new();

        transactions.expect_deposit().times(1).return_const(());

        let account = BankAccount {
            transactions: Box::new(&transactions),
        };

        account.deposit(100);
    }
}
