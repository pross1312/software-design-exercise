use rusqlite::{Connection};
use crate::io::SelectableEnum;

#[derive(Clone, serde::Serialize, serde::Deserialize)]
pub enum Gender {
    Male = 1, Female
}

impl rusqlite::ToSql for Gender {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>, rusqlite::Error> {
        Ok(rusqlite::types::ToSqlOutput::from(self.clone() as i32))
    }
}
impl Gender {
    pub fn value(&self) -> &'static str {
        match *self {
            Gender::Male => "Nam",
            Gender::Female => "Nữ",
        }
    }
}

impl SelectableEnum for Gender {
    fn print_choices(_conn: &Connection) -> usize {
        println!("1. Nam");
        println!("2. Nữ");

        2
    }
    fn parse_choice(choice: i32, _conn: &Connection) -> Option<Self> {
        match choice {
            1 => Some(Gender::Male),
            2 => Some(Gender::Female),
            _ => None,
        }
    }
}
