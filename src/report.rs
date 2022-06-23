use std::io::Write;

use mockall::{automock, mock};

use crate::transactions::Transaction;

#[automock]
pub trait Report {
    fn print(&mut self, transactions: Vec<Transaction>);
}

struct StdoutReport {
    writer: Box<dyn Write>,
}

impl Report for StdoutReport {
    fn print(&mut self, transactions: Vec<Transaction>) {
        self.writer.by_ref().write("versati 12 dollari".as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use mockall::predicate;

    use super::*;

    mock! {
        Writer{}

        impl Write for Writer {
            fn write(&mut self, text: &[u8]) -> Result<usize, std::io::Error> { todo!() }
            fn flush(&mut self) -> Result<(), std::io::Error> { todo!() }
        }
    }

    #[test]
    fn transform_a_list_of_transactions_into_a_report() {
        let mut writer = MockWriter::new();
        writer.expect_write()
        .with(predicate::eq("versati 12 dollari".as_bytes())).times(1).returning(|_| Ok(18));

        let mut report = StdoutReport {
            writer: Box::new(writer)};
        report.print(vec![Transaction { amount: 12 }]);
    }
}
