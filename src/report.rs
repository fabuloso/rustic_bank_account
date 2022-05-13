use mockall::automock;

#[automock]
pub trait Report {
    fn print(&self) -> &'static str;
}
