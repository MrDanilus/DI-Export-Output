pub mod single;

#[derive(Debug, Clone, Default)]
pub enum Page{
    #[default]
    Single,
    Bulk
}