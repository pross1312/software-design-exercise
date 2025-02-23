use rusqlite::{Connection};
use std::io::Write;
use crate::io::SelectableEnum;
use crate::log;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Status {
    pub id: i32,
    pub name: String,
}

impl Status {
    pub fn add(conn: &Connection, name: &str) {
        let result = conn.execute("INSERT INTO Status(name) values(?)", [name]).unwrap();
        if result != 1 {
            panic!("Could not add new status");
        } else {
            log!("Add new status {}", name);
            println!("Thêm trạng thái mới '{name}' thành công");
        }
    }

    pub fn update(conn: &Connection, status: &Status) {
        let result = conn.execute("UPDATE Status SET name = ? WHERE id = ?", rusqlite::params![status.name, status.id]).unwrap();
        if result != 1 {
            panic!("Could not update status");
        } else {
            log!("Change status name with id {} to {}", status.id, status.name);
            println!("Đổi tên trạng thái thành công");
        }
    }

    pub fn add_many(conn: &Connection, statuses: &[Status]) {
        let mut stmt = conn.prepare("INSERT INTO Status(id, name) VALUES(?, ?)").unwrap();
        for status in statuses {
            if let Err(_) = stmt.insert(rusqlite::params![status.id, status.name]) {
                println!("Không thể thêm trạng thái '{}' và database", status.name);
            } else {
                log!("Inserted status with id {} and name {} into database", status.id, status.name);
            }
        }
    }
}

impl SelectableEnum for Status {
    fn print_choices(conn: &Connection) -> usize {
        let mut stmt = conn.prepare("SELECT id, name FROM Status").unwrap();
        let iter = stmt.query_map([], |row| {
            Ok(Status {
                id: row.get(0)?,
                name: row.get(1)?
            })
        }).unwrap();
        let mut count = 0;
        for status in iter {
            let status = status.unwrap();
            println!("{}. {}", status.id, status.name);
            count += 1;
        }
        count
    }
    fn parse_choice(choice: i32, conn: &Connection) -> Option<Self> {
        if let Ok(status) = conn.query_row(
            "SELECT id, name FROM Status WHERE id = ?",
            [choice],
            |row| Ok(Status {id: row.get(0)?, name: row.get(1)?}) ) {
            Some(status)
        } else {
            None
        }
    }
}


