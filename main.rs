#![allow(dead_code)]
use std::io;
use std::env;
use chrono::Local;
use rusqlite::{Connection};
use std::io::{BufWriter, BufReader, Write};
use std::fs::{self, File};
use std::path::Path;

trait SelectableEnum {
    fn print_choices(conn: &Connection) -> usize;
    fn parse_choice(choice: i32, conn: &Connection) -> Option<Self> where Self: Sized;
}

#[derive(Clone, serde::Serialize, serde::Deserialize)]
enum Gender {
    Male = 1, Female
}
impl rusqlite::ToSql for Gender {
    fn to_sql(&self) -> Result<rusqlite::types::ToSqlOutput<'_>, rusqlite::Error> {
        Ok(rusqlite::types::ToSqlOutput::from(self.clone() as i32))
    }
}
impl Gender {
    fn value(&self) -> &'static str {
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

#[derive(serde::Serialize, serde::Deserialize)]
struct Student {
    id: String,
    name: String,
    dob: String,
    phone: String,
    address: String,
    email: String,
    status: Status,
    gender: Gender,
    faculty: Faculty,
    enrolled_year: i32,
    program: Program,
}

impl Student {
    fn new() -> Self {
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

    fn print(&self) {
        println!("Mã số sinh viên: {}", self.id);
        println!("Họ tên: {}", self.name);
        println!("Ngày tháng năm sinh: {}", self.dob);
        println!("Giới tính: {}", self.gender.value());
        println!("Khoa: {}", self.faculty.name);
        println!("Khóa: {}", self.enrolled_year);
        println!("Chương trình: {}", self.program.name);
        println!("Địa chỉ: {}", self.address);
        println!("Email: {}", self.email);
        println!("Tình trạng sinh viên: {}", self.status.name);
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct Faculty {
    id: i32,
    name: String,
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


#[derive(serde::Serialize, serde::Deserialize)]
struct Status {
    id: i32,
    name: String,
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

#[derive(serde::Serialize, serde::Deserialize)]
struct Program {
    id: i32,
    name: String,
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


fn validate_id(_id: &str) -> Option<&'static str> {
    None
}
fn validate_phone(phone: &str) -> Option<&'static str> {
    if let Err(_) = phone.parse::<i32>() {
        Some("Số điện thoại phải là số, vui long nhập lại.")
    } else if phone.len() != 10 {
        Some("Số điện thoại phải có 10 chữ số, vui lòng nhập lại.")
    } else {
        None
    }
}

fn validate_email(email: &str) -> Option<&'static str> {
    let valid_email = if let Some(at_pos) = email.find('@') {
        let (local, domain) = email.split_at(at_pos);
        let domain = &domain[1..];
        if local.is_empty() || domain.is_empty() || domain.find('.').is_none() {
            false
        } else {
            true
        }
    } else {
        false
    };
    if valid_email {
        None
    } else {
        Some("Email không hợp lệ vui lòng nhập lại")
    }
}

fn check_date(day: u32, month: u32, year: u32) -> bool {
    if year < 1 || month < 1 || month > 12 || day < 1 {
        return false;
    }
    let days_in_month = match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11 => 30,
        2 => if (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0) { 29 } else { 28 },
        _ => return false,
    };
    day <= days_in_month
}

// dd/mm/yyyy
fn validate_date(date: &str) -> Option<&'static str> {
    let mut data = [0u32; 3];
    for (i, number) in date.splitn(3, |c| c == '/').enumerate() {
        if let Ok(n) = number.parse::<u32>() {
            data[i] = n;
        } else {
            return Some("Sai định dạng dd/mm/yyyy, vui lòng nhập lại");
        }
    }
    if !check_date(data[0], data[1], data[2]) {
        Some("Ngày không hợp lệ, vui lòng nhập lại")
    } else {
        None
    }
}

fn read_string(prompt: &str, buffer: &mut String) -> Result<usize, std::io::Error> {
    buffer.clear();
    print!("{}: ", prompt);
    io::stdout().flush().unwrap();
    let result = match io::stdin().read_line(buffer) {
        Ok(n) => {
            if n > 0 {
                buffer.pop();
                Ok(n-1)
            } else {
                Ok(n)
            }
        },
        Err(err) => Err(err),
    };
    println!();
    result
}

fn read_string_new(prompt: &str) -> String {
    let mut buffer = String::new();
    read_string(prompt, &mut buffer).unwrap();
    return buffer;
}

fn read_enum_until_correct<T: SelectableEnum>(prompt: &str, conn: &Connection) -> T {
    let mut input = String::new();
    loop {
        println!("{}", prompt);
        T::print_choices(conn);
        match read_string(&"Chọn một trong những giá trị trên", &mut input) {
            Ok(_) => match input.parse::<i32>() {
                Ok(choice) => {
                    if let Some(value) = T::parse_choice(choice, conn) {
                        return value;
                    } else {
                        println!("Lụa chọn '{}' không hợp lệ, vui lòng chọn lại", input)
                    }
                },
                Err(_) => {
                    println!("Lựa chọn '{}' không hợp lệ, vui lòng chọn lại", input)
                },
            },
            Err(err) => panic!("{}", err),
        };
        println!();
    }
}

fn read_number_until_correct(prompt: &str, start: i32, end: i32) -> i32 {
    let mut input = String::new();
    loop {
        match read_string(prompt, &mut input) {
            Ok(_) => match input.parse::<i32>() {
                Ok(number) => {
                    if number >= start && number <= end { return number; }
                    else { println!("Số {} không hợp lệ, vui lòng chọn lại", number); }
                },
                Err(_) => {
                    println!("Dữ liệu {} không hợp lệ, vui lòng chọn lại", input)
                },
            },
            Err(err) => panic!("{}", err),
        };
        println!();
    }
}

type ValidateFn = dyn Fn(&str) -> Option<&'static str>;
fn read_string_until_correct(prompt: &'static str, buffer: &mut String, validate: &ValidateFn) -> usize {
    loop {
        let n = read_string(prompt, buffer).unwrap();
        match validate(&buffer) {
            Some(err) => {
                println!("{}", err);
            },
            None => return n,
        }
        println!();
    }
}


enum UpdateStudentOption {
    UpdateName,
    UpdateDob,
    UpdatePhone,
    UpdateAddress,
    UpdateEmail,
    UpdateStatus,
    UpdateGender,
    UpdateFaculty,
    UpdateEnrolledYear,
    UpdateProgram,
    Done,
}

impl SelectableEnum for UpdateStudentOption {
    fn print_choices(_conn: &Connection) -> usize {
        println!("1. Cập nhật tên");
        println!("2. Cập nhật ngày sinh");
        println!("3. Cập nhật số điện thoại");
        println!("4. Cập nhật địa chỉ");
        println!("5. Cập nhật email");
        println!("6. Cập nhật trạng thái");
        println!("7. Cập nhật giới tính");
        println!("8. Cập nhật khoa");
        println!("9. Cập nhật khóa");
        println!("10. Cập nhật chương trình");
        println!("11.Hoàn thành");
        11
    }
    fn parse_choice(choice: i32, _conn: &Connection) -> Option<Self> where Self: Sized {
        match choice {
            1 => Some(UpdateStudentOption::UpdateName),
            2 => Some(UpdateStudentOption::UpdateDob),
            3 => Some(UpdateStudentOption::UpdatePhone),
            4 => Some(UpdateStudentOption::UpdateAddress),
            5 => Some(UpdateStudentOption::UpdateEmail),
            6 => Some(UpdateStudentOption::UpdateStatus),
            7 => Some(UpdateStudentOption::UpdateGender),
            8 => Some(UpdateStudentOption::UpdateFaculty),
            9 => Some(UpdateStudentOption::UpdateEnrolledYear),
            10 => Some(UpdateStudentOption::UpdateProgram),
            11 => Some(UpdateStudentOption::Done),
            _ => None,
        }
    }
}

enum FileFormat {
    Json,
    Xml,
}
impl FileFormat {
    fn extension(&self) -> &'static str {
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

macro_rules! log {
    ($($arg:tt)*) => { File::options().create(true).append(true).open(LOG_FILE).unwrap().write_all(
            format!("{} : {}\n", Local::now(), (format!($($arg)*))).as_bytes()
        ).unwrap()
    };
}

enum Operation {
    AddNewStudent(Student),
    DeleteStudent(String),
    UpdateStudent(String),
    SearchStudent(String),
    AddNewFaculty(String),
    AddNewStatus(String),
    AddNewProgram(String),
    UpdateFaculty(Faculty),
    UpdateStatus(Status),
    UpdateProgram(Program),
    SearchByFaculty(Faculty),
    SearchByFacultyAndName(Faculty, String),
    Export(FileFormat, String),
    Import(FileFormat, String),
    Quit,
}

impl Operation {
    fn new_operation_add(conn: &Connection) -> Self {
        let mut new_student = Student::new();
        read_string_until_correct("Nhập mã số sinh viên", &mut new_student.id, &validate_id);
        read_string("Nhập họ tên", &mut new_student.name).unwrap();
        read_string_until_correct("Nhập ngày tháng năm sinh (dd/mm/yyyy)", &mut new_student.dob, &validate_date);
        read_string_until_correct("Nhập số điện thoại", &mut new_student.phone, &validate_phone);
        new_student.enrolled_year = read_number_until_correct("Nhập khóa (1990, 2025)", 1990, 2025);
        new_student.gender = read_enum_until_correct("Nhập giới tính", conn);
        new_student.faculty = read_enum_until_correct("Nhập khoa", conn);
        new_student.program = read_enum_until_correct("Nhập chương trình", conn);
        new_student.status = read_enum_until_correct("Nhập tình trạng", conn);
        read_string("Nhập địa chỉ", &mut new_student.address).unwrap();
        read_string_until_correct("Nhập Email", &mut new_student.email, &validate_email);

        Operation::AddNewStudent(new_student)
    }
}

impl SelectableEnum for Operation {
    fn print_choices(_conn: &Connection) -> usize {
        let ops = [
            "Thêm sinh viên mới",
            "Xóa sinh viên",
            "Cập nhật thông tin sinh viên",
            "Tìm kiếm sinh viên",
            "Thêm khoa mới",
            "Thêm trạng thái mới",
            "Thêm loại chương trình học mới",
            "Đổi tên khoa",
            "Đổi tên trạng thái",
            "Đổi tên chương trình học",
            "Tìm theo khoa",
            "Tìm theo khoa và tên sinh viên",
            "Export",
            "Import",
            "Kết thúc",
        ];
        for (i, op) in ops.iter().enumerate() {
            println!("{}. {op}", i+1);
        }
        ops.len()
    }
    fn parse_choice(choice: i32, conn: &Connection) -> Option<Self> where Self: Sized {
        match choice {
            1 => Some(Operation::new_operation_add(conn)),
            2 => Some(Operation::DeleteStudent(read_string_new("Nhập Mã số sinh viên cần xóa"))),
            3 => Some(Operation::UpdateStudent(read_string_new("Nhập mã số của sinh viên cần cập nhật"))),
            4 => Some(Operation::SearchStudent(read_string_new("Nhập mã số hoặc tên của sinh viên cần tìm"))),
            5 => Some(Operation::AddNewFaculty(read_string_new("Nhập tên khoa mới"))),
            6 => Some(Operation::AddNewStatus(read_string_new("Nhập tên trạng thái mới"))),
            7 => Some(Operation::AddNewProgram(read_string_new("Nhập tên chương trình học mới"))),
            8 => Some(Operation::UpdateFaculty({
                let mut faculty: Faculty = read_enum_until_correct("Chọn khoa cần đổi", conn);
                read_string("Nhập tên khoa mới", &mut faculty.name).unwrap();
                faculty
            })),
            9 => Some(Operation::UpdateStatus({
                let mut status: Status = read_enum_until_correct("Chọn trạng thái đổi", conn);
                read_string("Nhập tên trạng thái mới", &mut status.name).unwrap();
                status
            })),
            10 => Some(Operation::UpdateProgram({
                let mut program: Program = read_enum_until_correct("Chọn chương trình học đổi", conn);
                read_string("Nhập tên chương trình học mới", &mut program.name).unwrap();
                program
            })),
            11 => Some(Operation::SearchByFaculty(read_enum_until_correct::<Faculty>("Chọn khoa muốn tìm", conn))),
            12 => Some(Operation::SearchByFacultyAndName(read_enum_until_correct::<Faculty>("Chọn khoa muốn tìm", conn), read_string_new("Nhập tên sinh viên cần tìm"))),
            13 => Some(Operation::Export(read_enum_until_correct("Chọn file format", conn), read_string_new("Nhập tên file"))),
            14 => Some(Operation::Import(read_enum_until_correct("Chọn file format", conn), read_string_new("Nhập tên file"))),
            15 => Some(Operation::Quit),
            _ => todo!(),
        }
    }
}

fn update_student_fields(student: &mut Student, conn: &Connection) {
    loop {
        let option = read_enum_until_correct::<UpdateStudentOption>("", conn);
        match option {
            UpdateStudentOption::UpdateName => { read_string("Nhập tên mới", &mut student.name).unwrap(); },
            UpdateStudentOption::UpdateDob => { read_string_until_correct("Nhập ngày sinh mới (dd/mm/yyyy)", &mut student.dob, &validate_date); },
            UpdateStudentOption::UpdatePhone => { read_string_until_correct("Nhập số điện thoại mới", &mut student.phone, &validate_phone); },
            UpdateStudentOption::UpdateAddress => { read_string("Nhập địa chỉ mới", &mut student.address).unwrap(); },
            UpdateStudentOption::UpdateEmail => { read_string_until_correct("Nhập email mới", &mut student.email, &validate_email); },
            UpdateStudentOption::UpdateStatus => { student.status = read_enum_until_correct("Nhập trạng thái mới", conn); },
            UpdateStudentOption::UpdateGender => { student.gender = read_enum_until_correct("Nhập giới tính mới", conn); },
            UpdateStudentOption::UpdateFaculty => { student.faculty = read_enum_until_correct("Nhập khoa mới", conn); },
            UpdateStudentOption::UpdateEnrolledYear => { student.enrolled_year = read_number_until_correct("Nhập khóa mới (1990, 2025)", 1990, 2025); },
            UpdateStudentOption::UpdateProgram => { student.program = read_enum_until_correct("Nhập khoa mới", conn); },
            UpdateStudentOption::Done => { break; },
        }
    }
}

fn add_student(conn: &Connection, new_student: &Student) {
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

fn search_student(conn: &Connection, id_or_name: &str) -> Option<Student> {
    log!("Search for student with id or name {}", id_or_name);
    if let Ok(student) = conn.query_row(
        "SELECT id, name, dob, phone, address, email, status, gender, faculty, enrolled_year, program FROM Student WHERE LOWER(id) = LOWER(?) OR LOWER(name) LIKE LOWER(?)",
        [id_or_name, id_or_name],
        |row| Ok(Student {
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
        }) ) {
        Some(student)
    } else {
        None
    }
}

fn update_student(conn: &Connection, _id: String, student: &Student) {
    log!("Update info for student with id {}", student.id);
    conn.execute("UPDATE Student SET name = ?, dob = ?, phone = ?, address = ?, email = ?, status = ?, gender = ?, faculty = ?, enrolled_year = ?, program = ? WHERE id = ?", rusqlite::params![
                 student.name, student.dob, student.phone, student.address, student.email, student.status.id, student.gender, student.faculty.id, student.enrolled_year, student.program.id,
                 student.id
                 ]).unwrap();
}

fn delete_student(conn: &Connection, id: &str) -> bool {
    let result = conn.execute("DELETE FROM Student WHERE id = ?", [id]).unwrap();
    return if result == 1 {
        log!("Delete student with id {}", id);
        true
    } else {
        false
    }
}

fn add_faculty(conn: &Connection, name: &str) {
    let result = conn.execute("INSERT INTO Faculty(name) values(?)", [name]).unwrap();
    if result != 1 {
        panic!("Could not add new faculty");
    } else {
        log!("Add new faculty {}", name);
        println!("Thêm khoa mới '{name}' thành công");
    }
}

fn add_status(conn: &Connection, name: &str) {
    let result = conn.execute("INSERT INTO Status(name) values(?)", [name]).unwrap();
    if result != 1 {
        panic!("Could not add new status");
    } else {
        log!("Add new status {}", name);
        println!("Thêm trạng thái mới '{name}' thành công");
    }
}

fn add_program(conn: &Connection, name: &str) {
    let result = conn.execute("INSERT INTO Program(name) values(?)", [name]).unwrap();
    if result != 1 {
        panic!("Could not add new program");
    } else {
        log!("Add new program {}", name);
        println!("Thêm chương trình học mới '{name}' thành công");
    }
}

fn update_faculty(conn: &Connection, faculty: &Faculty) {
    let result = conn.execute("UPDATE Faculty SET name = ? WHERE id = ?", rusqlite::params![faculty.name, faculty.id]).unwrap();
    if result != 1 {
        panic!("Could not update faculty");
    } else {
        log!("Change faculty name with id {} to {}", faculty.id, faculty.name);
        println!("Đổi tên khoa thành công");
    }
}

fn update_status(conn: &Connection, status: &Status) {
    let result = conn.execute("UPDATE Status SET name = ? WHERE id = ?", rusqlite::params![status.name, status.id]).unwrap();
    if result != 1 {
        panic!("Could not update status");
    } else {
        log!("Change status name with id {} to {}", status.id, status.name);
        println!("Đổi tên trạng thái thành công");
    }
}

fn update_program(conn: &Connection, program: &Program) {
    let result = conn.execute("UPDATE Program SET name = ? WHERE id = ?", rusqlite::params![program.name, program.id]).unwrap();
    if result != 1 {
        panic!("Could not update program");
    } else {
        log!("Change program name with id {} to {}", program.id, program.name);
        println!("Đổi tên chương trình học thành công");
    }
}

fn search_by_faculty(conn: &Connection, Faculty{id, name}: &Faculty, student_name: Option<&str>) {
    let (mut stmt, args) = if None == student_name {
        log!("Searching for all students in faculty {}", name);
        (conn.prepare("SELECT * FROM Student WHERE faculty = ?").unwrap(), rusqlite::params![id])
    } else {
        log!("Searching for student in faculty {} with name or id {}", name, student_name.unwrap());
        (conn.prepare("SELECT * FROM Student WHERE Faculty = ? AND LOWER(name) LIKE LOWER(?)").unwrap(), rusqlite::params![id, student_name.unwrap()])
    };
    let iter = stmt.query_map(args, |row| {
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
    }).unwrap();
    if student_name == None {
        println!("Các học sinh trong {name} là");
    } else {
        println!("Các học sinh trong {name} có tên '{}' là", student_name.unwrap());
    }
    for student in iter {
        student.unwrap().print();
        println!();
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
struct DataFormat {
    statuses: Vec<Status>,
    programs: Vec<Program>,
    faculties: Vec<Faculty>,
    students: Vec<Student>,
}

fn insert_multiple_faculties(conn: &Connection, faculties: &[Faculty]) {
    let mut stmt = conn.prepare("INSERT INTO Faculty(id, name) VALUES(?, ?)").unwrap();
    for faculty in faculties {
        if let Err(_) = stmt.insert(rusqlite::params![faculty.id, faculty.name]) {
            println!("Không thể thêm '{}' và database", faculty.name);
        } else {
            log!("Inserted faculty with id {} and name {} into database", faculty.id, faculty.name);
        }
    }
}
fn insert_multiple_statuses(conn: &Connection, statuses: &[Status]) {
    let mut stmt = conn.prepare("INSERT INTO Status(id, name) VALUES(?, ?)").unwrap();
    for status in statuses {
        if let Err(_) = stmt.insert(rusqlite::params![status.id, status.name]) {
            println!("Không thể thêm trạng thái '{}' và database", status.name);
        } else {
            log!("Inserted status with id {} and name {} into database", status.id, status.name);
        }
    }
}
fn insert_multiple_programs(conn: &Connection, programs: &[Program]) {
    let mut stmt = conn.prepare("INSERT INTO Program(id, name) VALUES(?, ?)").unwrap();
    for program in programs {
        if let Err(_) = stmt.insert(rusqlite::params![program.id, program.name]) {
            println!("Không thể thêm chương trình '{}' và database", program.name);
        } else {
            log!("Inserted program with id {} and name {} into database", program.id, program.name);
        }
    }
}
fn insert_multiple_students(conn: &Connection, students: &[Student]) {
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

fn export_data(conn: &Connection, file_name: &str, format: FileFormat) {
    log!("Export data to {}", file_name);
    let path = Path::new(&file_name).with_extension(format.extension());
    let all_faculties = conn.prepare("SELECT * FROM Faculty").unwrap()
        .query_map([], |row| {
            Ok(Faculty {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        }).unwrap().map(|result| result.unwrap()).collect::<Vec<Faculty>>();
    let all_statuses = conn.prepare("SELECT * FROM Status").unwrap()
        .query_map([], |row| {
            Ok(Status {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        }).unwrap().map(|result| result.unwrap()).collect::<Vec<Status>>();
    let all_programs = conn.prepare("SELECT * FROM Program").unwrap()
        .query_map([], |row| {
            Ok(Program {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        }).unwrap().map(|result| result.unwrap()).collect::<Vec<Program>>();
    let all_students = conn.prepare("SELECT * FROM Student").unwrap()
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
        }).unwrap().map(|result| result.unwrap()).collect::<Vec<Student>>();

    let mut writer = BufWriter::new(File::create(&path).unwrap());
    match format {
        FileFormat::Json => {
            serde_json::to_writer(writer, &DataFormat {
                statuses: all_statuses,
                programs: all_programs,
                faculties: all_faculties,
                students: all_students,
            }).unwrap();
        },
        FileFormat::Xml => {
            writer.write_all(
                quick_xml::se::to_string(&DataFormat {
                    statuses: all_statuses,
                    programs: all_programs,
                    faculties: all_faculties,
                    students: all_students,
                }).unwrap().as_bytes()
            ).unwrap();
        },
    };
}

fn import_data(conn: &Connection, file_name: &str, format: FileFormat) {
    log!("Import data from {}", file_name);
    let path = Path::new(file_name).with_extension(format.extension());
    if let Ok(reader) = File::open(&path).and_then(|file| Ok(BufReader::new(file))) {
        let data: DataFormat = match format {
            FileFormat::Json => serde_json::from_reader(reader).unwrap(),
            FileFormat::Xml => quick_xml::de::from_reader(reader).unwrap(),
        };
        insert_multiple_faculties(conn, &data.faculties);
        insert_multiple_statuses(conn, &data.statuses);
        insert_multiple_programs(conn, &data.programs);
        insert_multiple_students(conn, &data.students);
    } else {
        println!("{file_name} does not exist");
    }
}

const DB_PATH: &str = "data.db";
const MIGRATION_SCRIPT: &str = "migrate.sql";
const VERSION: &str = "2.0";
const BUILD_DATE: &str = "19/02/2025";
const LOG_FILE: &str = "data.log";

fn main() {
    for arg in env::args() {
        if arg == "--version" {
            println!("Version: {} - {}", VERSION, BUILD_DATE);
            return;
        }
    }
    let conn = Connection::open(DB_PATH).unwrap();
    conn.execute_batch(&fs::read_to_string(MIGRATION_SCRIPT).unwrap()).unwrap();

    loop {
        match read_enum_until_correct("Chọn hành động", &conn) {
            Operation::AddNewStudent(new_student) => {
                if let Some(student) = search_student(&conn, &new_student.id) {
                    println!("Học sinh với mã số {} đã tồn tại, không thể thêm học sinh mới vào", student.id);
                } else {
                    add_student(&conn, &new_student);
                }
            },
            Operation::UpdateStudent(id) => {
                if let Some(mut student) = search_student(&conn, &id) {
                    println!("Cập nhập thông tin mới cho sinh viên");
                    update_student_fields(&mut student, &conn);
                    println!("Thông tin của sinh viên sau khi sửa");
                    student.print();
                    update_student(&conn, id, &student);
                } else {
                    println!("Không thể tìm thấy sinh viên có mã số {}", id);
                }
            },
            Operation::SearchStudent(search) => {
                if let Some(student) = search_student(&conn, &search) {
                    println!("Sinh viên cần tìm là");
                    student.print();
                    println!();
                } else {
                    println!("Không thể tìm thấy sinh viên có mã số hoặc tên {}", search);
                }
            },
            Operation::DeleteStudent(id) => {
                if delete_student(&conn, &id) {
                    println!("Xóa thành công sinh viên với mã số {}", id);
                } else {
                    println!("Không thể tìm thấy sinh viên với mã số {}", id);
                }
            },
            Operation::AddNewFaculty(name) => {
                add_faculty(&conn, &name);
            },
            Operation::AddNewStatus(name) => {
                add_status(&conn, &name);
            },
            Operation::AddNewProgram(name) => {
                add_program(&conn, &name);
            },
            Operation::UpdateFaculty(faculty) => {
                update_faculty(&conn, &faculty);
            },
            Operation::UpdateStatus(status) => {
                update_status(&conn, &status);
            },
            Operation::UpdateProgram(program) => {
                update_program(&conn, &program);
            },
            Operation::SearchByFaculty(faculty) => {
                search_by_faculty(&conn, &faculty, None);
            },
            Operation::SearchByFacultyAndName(faculty, name) => {
                search_by_faculty(&conn, &faculty, Some(&name));
            },
            Operation::Export(format, file_name) => {
                export_data(&conn, &file_name, format);
            },
            Operation::Import(format, file_name) => {
                import_data(&conn, &file_name, format);
            },
            Operation::Quit => break,
        }
        println!();
    }
}
