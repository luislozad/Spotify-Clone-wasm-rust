pub mod router;

#[derive(Clone, PartialEq, Debug)]
pub enum WidthColumn {
    Equal,
    Relative,
}