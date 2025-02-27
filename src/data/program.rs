use rusqlite::{Connection};
use std::io::Write;
use crate::io::SelectableEnum;
use crate::log;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Program {
    pub id: i64,
    pub name: String,
}

impl Program {
    pub fn get_all(conn: &Connection) -> Vec<Program> {
        conn.prepare("SELECT * FROM Program").unwrap()
            .query_map([], |row| {
                Ok(Program {
                    id: row.get(0)?,
                    name: row.get(1)?,
                })
            }).unwrap().map(|result| result.unwrap()).collect::<Vec<Program>>()
    }

    pub fn add(conn: &Connection, name: &str) -> i64 {
        let result = conn.execute("INSERT INTO Program(name) values(?)", [name]).unwrap();
        if result != 1 {
            panic!("Could not add new program");
        } else {
            log!("Add new program {}", name);
            println!("Thêm chương trình học mới '{name}' thành công");
            return conn.last_insert_rowid();
        }
    }

    pub fn update(conn: &Connection, program: &Program) {
        let result = conn.execute("UPDATE Program SET name = ? WHERE id = ?", rusqlite::params![program.name, program.id]).unwrap();
        if result != 1 {
            panic!("Could not update program");
        } else {
            log!("Change program name with id {} to {}", program.id, program.name);
            println!("Đổi tên chương trình học thành công");
        }
    }

    pub fn add_many(conn: &Connection, programs: &[Program]) {
        let mut stmt = conn.prepare("INSERT INTO Program(id, name) VALUES(?, ?)").unwrap();
        for program in programs {
            if let Err(_) = stmt.insert(rusqlite::params![program.id, program.name]) {
                println!("Không thể thêm chương trình '{}' và database", program.name);
            } else {
                log!("Inserted program with id {} and name {} into database", program.id, program.name);
            }
        }
    }
}

impl SelectableEnum for Program {
    fn print_choices(conn: &Connection) -> usize {
        let mut stmt = conn.prepare("SELECT id, name FROM Program").unwrap();
        let iter = stmt.query_map([], |row| {
            Ok(Program {
                id: row.get(0)?,
                name: row.get(1)?
            })
        }).unwrap();
        let mut count = 0;
        for program in iter {
            let program = program.unwrap();
            println!("{}. {}", program.id, program.name);
            count += 1;
        }
        count
    }
    fn parse_choice(choice: i32, conn: &Connection) -> Option<Self> {
        if let Ok(program) = conn.query_row(
            "SELECT id, name FROM Program WHERE id = ?",
            [choice],
            |row| Ok(Program {id: row.get(0)?, name: row.get(1)?}) ) {
            Some(program)
        } else {
            None
        }
    }
}


