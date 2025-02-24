use super::*;
use crate::io::SelectableEnum;
use rusqlite::{Connection, Row};
use std::io::Write;
use crate::log;

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Student {
    pub id: String,
    pub name: String,
    pub dob: String,
    pub phone: String,
    pub address: String,
    pub email: String,
    pub status: Status,
    pub gender: Gender,
    pub faculty: Faculty,
    pub enrolled_year: i32,
    pub program: Program,
}

impl Student {
    pub fn new() -> Self {
        Self {
            id: String::new(),
            name: String::new(),
            dob: String::new(),
            phone: String::new(),
            address: String::new(),
            email: String::new(),
            status: Status { id: 0, name: String::new() },
            gender: Gender::Male,
            faculty: Faculty { id: 0, name: String::new() },
            enrolled_year: 2025,
            program: Program { id: 0, name: String::new() },
        }
    }

    pub fn print(&self) {
        println!("Mã số sinh viên: {}", self.id);
        println!("Họ tên: {}", self.name);
        println!("Số điện thoại: {}", self.phone);
        println!("Ngày tháng năm sinh: {}", self.dob);
        println!("Giới tính: {}", self.gender.value());
        println!("Khoa: {}", self.faculty.name);
        println!("Khóa: {}", self.enrolled_year);
        println!("Chương trình: {}", self.program.name);
        println!("Địa chỉ: {}", self.address);
        println!("Email: {}", self.email);
        println!("Tình trạng sinh viên: {}", self.status.name);
    }

    pub fn get_all(conn: &Connection) -> Vec<Student> {
        conn.prepare("SELECT * FROM Student").unwrap()
            .query_map([], |row| {
                Ok(Student {
                    id: row.get(0)?,
                    name: row.get(1)?,
                    dob: row.get(2)?,
                    phone: row.get(3)?,
                    address: row.get(4)?,
                    email: row.get(5)?,
                    status: Status::parse_choice(row.get(6)?, conn).unwrap(),
                    gender: Gender::parse_choice(row.get(7)?, conn).unwrap(),
                    faculty: Faculty::parse_choice(row.get(8)?, conn).unwrap(),
                    enrolled_year: row.get(9)?,
                    program: Program::parse_choice(row.get(10)?, conn).unwrap(),
                })
            }).unwrap().map(|result| result.unwrap()).collect::<Vec<Student>>()
    }

    pub fn delete(conn: &Connection, id: &str) -> bool {
        let result = conn.execute("DELETE FROM Student WHERE id = ?", [id]).unwrap();
        return if result == 1 {
            log!("Delete student with id {}", id);
            true
        } else {
            false
        }
    }

    pub fn add(conn: &Connection, new_student: &Student) {
        let result = conn.execute("INSERT INTO Student(id, name, dob, phone, address, email, status, gender, faculty, enrolled_year, program) 
                 VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)", rusqlite::params![
                 new_student.id, new_student.name, new_student.dob, new_student.phone, new_student.address, new_student.email, new_student.status.id, new_student.gender, new_student.faculty.id, new_student.enrolled_year, new_student.program.id
                 ]).unwrap();
        if result != 1 {
            panic!("Could not insert new student");
        } else {
            log!("Added new student with id {}, name {}", new_student.id, new_student.name);
            println!("Đã thêm 1 sinh viên");
            new_student.print();
            println!();
        }
    }

    pub fn add_many(conn: &Connection, students: &[Student]) {
        let mut stmt = conn.prepare("INSERT INTO Student(id, name, dob, phone, address, email, status, gender, faculty, enrolled_year, program) 
                 VALUES(?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)").unwrap();
        for student in students {
            if let Err(_) = stmt.insert(rusqlite::params![student.id, student.name, student.dob, student.phone, student.address, student.email, student.status.id, student.gender, student.faculty.id, student.enrolled_year, student.program.id]) {
                println!("Không thể thêm học sinh với mã số {} vào database", student.id);
            } else {
                log!("Inserted student with id {} into database", student.id);
            }
        }
    }

    pub fn update(conn: &Connection, id: &str, student: &Student) {
        log!("Update info for student with id {}", id);
        conn.execute("UPDATE Student SET name = ?, dob = ?, phone = ?, address = ?, email = ?, status = ?, gender = ?, faculty = ?, enrolled_year = ?, program = ? WHERE id = ?", rusqlite::params![
            student.name, student.dob, student.phone, student.address, student.email, student.status.id, student.gender, student.faculty.id, student.enrolled_year, student.program.id,
            id
        ]).unwrap();
    }

    pub fn map_row(conn: &Connection, row: &Row) -> Result<Student, rusqlite::Error> {
        Ok(Student {
            id: row.get(0)?,
            name: row.get(1)?,
            dob: row.get(2)?,
            phone: row.get(3)?,
            address: row.get(4)?,
            email: row.get(5)?,
            status: Status::parse_choice(row.get(6)?, conn).unwrap(),
            gender: Gender::parse_choice(row.get(7)?, conn).unwrap(),
            faculty: Faculty::parse_choice(row.get(8)?, conn).unwrap(),
            enrolled_year: row.get(9)?,
            program: Program::parse_choice(row.get(10)?, conn).unwrap(),
        })
    }
}


