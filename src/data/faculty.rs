use rusqlite::{Connection};
use crate::log;
use std::io::Write;
use crate::io::SelectableEnum;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Faculty {
    pub id: i32,
    pub name: String,
}

impl Faculty {
    pub fn add(conn: &Connection, name: &str) {
        let result = conn.execute("INSERT INTO Faculty(name) values(?)", [name]).unwrap();
        if result != 1 {
            panic!("Could not add new faculty");
        } else {
            log!("Add new faculty {}", name);
            println!("Thêm khoa mới '{name}' thành công");
        }
    }

    pub fn update(conn: &Connection, faculty: &Faculty) {
        let result = conn.execute("UPDATE Faculty SET name = ? WHERE id = ?", rusqlite::params![faculty.name, faculty.id]).unwrap();
        if result != 1 {
            panic!("Could not update faculty");
        } else {
            log!("Change faculty name with id {} to {}", faculty.id, faculty.name);
            println!("Đổi tên khoa thành công");
        }
    }

    pub fn add_many(conn: &Connection, faculties: &[Faculty]) {
        let mut stmt = conn.prepare("INSERT INTO Faculty(id, name) VALUES(?, ?)").unwrap();
        for faculty in faculties {
            if let Err(_) = stmt.insert(rusqlite::params![faculty.id, faculty.name]) {
                println!("Không thể thêm '{}' và database", faculty.name);
            } else {
                log!("Inserted faculty with id {} and name {} into database", faculty.id, faculty.name);
            }
        }
    }
}

impl SelectableEnum for Faculty {
    fn print_choices(conn: &Connection) -> usize {
        let mut stmt = conn.prepare("SELECT id, name FROM Faculty").unwrap();
        let iter = stmt.query_map([], |row| {
            Ok(Faculty {
                id: row.get(0)?,
                name: row.get(1)?
            })
        }).unwrap();
        let mut count = 0;
        for faculty in iter {
            let faculty = faculty.unwrap();
            println!("{}. {}", faculty.id, faculty.name);
            count += 1;
        }
        count
    }
    fn parse_choice(choice: i32, conn: &Connection) -> Option<Self> {
        if let Ok(faculty) = conn.query_row(
            "SELECT id, name FROM Faculty WHERE id = ?",
            [choice],
            |row| Ok(Faculty {id: row.get(0)?, name: row.get(1)?}) ) {
            Some(faculty)
        } else {
            None
        }
    }
}
