use rusqlite::{Connection};
use super::selectable_enum::SelectableEnum;

pub enum FileFormat {
    Json,
    Xml,
}
impl FileFormat {
    pub fn extension(&self) -> &'static str {
        match *self {
            FileFormat::Json => "json",
            FileFormat::Xml => "xml",
        }
    }
}

impl SelectableEnum for FileFormat {
    fn print_choices(_conn: &Connection) -> usize {
        let ops = [
            "JSON Format",
            "XML Format",
        ];
        for (i, op) in ops.iter().enumerate() {
            println!("{}. {op}", i+1);
        }
        ops.len()
    }
    fn parse_choice(choice: i32, _conn: &Connection) -> Option<Self> where Self: Sized {
        match choice {
            1 => Some(FileFormat::Json),
            2 => Some(FileFormat::Xml),
            _ => None,
        }
    }
}


