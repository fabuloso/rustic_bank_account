use crate::{report::Report, transactions::Transactions};

pub trait AccountService {
    fn deposit(&self, amount: i32);
    fn withdraw(&self, amount: i32);
    fn print_statement(&self);
}

pub struct BankAccount<'a> {
    transactions: Box<&'a dyn Transactions>,
    report: Box<&'a dyn Report>,
}

impl AccountService for BankAccount<'_> {
    fn deposit(&self, amount: i32) {
        self.transactions.as_ref().deposit(amount);
    }
    fn withdraw(&self, amount: i32) {
        self.transactions.as_ref().withdraw(amount);
    }
    fn print_statement(&self) {
        self.report.as_ref().print();
    }
}

#[cfg(test)]
mod tests {
    use crate::report::MockReport;
    use crate::{transactions::MockTransactions, AccountService, BankAccount};
    use mockall::predicate::eq;

    #[test]
    fn withdraw_100_dollars() {
        let mut transactions = MockTransactions::new();
        let report = MockReport::new();

        transactions.expect_withdraw().times(1).return_const(());

        let account = BankAccount {
            transactions: Box::new(&transactions),
            report: Box::new(&report),
        };

        account.withdraw(100);
    }

    #[test]
    fn deposit_100_dollars() {
        let mut transactions = MockTransactions::new();
        let report = MockReport::new();

        transactions.expect_deposit().times(1).return_const(());

        let account = BankAccount {
            transactions: Box::new(&transactions),
            report: Box::new(&report),
        };

        account.deposit(100);
    }

    #[test]
    fn print_statement() {
        let mut report = MockReport::new();
        let transactions = MockTransactions::new();

        report.expect_print().times(1).return_const("REPORT");

        let account = BankAccount {
            transactions: Box::new(&transactions),
            report: Box::new(&report),
        };

        account.print_statement();
    }
}
