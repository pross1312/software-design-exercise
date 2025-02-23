use rusqlite::{Connection};
pub trait SelectableEnum {
    fn print_choices(conn: &Connection) -> usize;
    fn parse_choice(choice: i32, conn: &Connection) -> Option<Self> where Self: Sized;
}
