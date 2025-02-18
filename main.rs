#![allow(dead_code)]
use std::io;
use rusqlite::{Connection, Result};
use std::io::Write;
use std::fs;

trait SelectableEnum {
    fn print_choices(conn: &Connection) -> usize;
    fn parse_choice(choice: i32, conn: &Connection) -> Option<Self> where Self: Sized;
}

#[derive(Clone)]
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

enum Operation {
    AddNewStudent(Student),
    DeleteStudent(String),
    UpdateStudent(String),
    SearchStudent(String),
}

impl Operation {
    fn new_operation_update() -> Self {
        let mut id = String::new();
        read_string("Nhập mã số của sinh viên cần cập nhật", &mut id).unwrap();

        Operation::UpdateStudent(id)
    }

    fn new_operation_search() -> Self {
        let mut search = String::new();
        read_string("Nhập mã số hoặc tên của sinh viên cần tìm", &mut search).unwrap();

        Operation::SearchStudent(search)
    }

    fn new_operation_delete() -> Self {
        let mut id = String::new();
        read_string_until_correct("Nhập Mã số sinh viên cần xóa", &mut id, &validate_id);

        Operation::DeleteStudent(id)
    }

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

impl SelectableEnum for Operation {
    fn print_choices(_conn: &Connection) -> usize {
        println!("1. Thêm sinh viên mới");
        println!("2. Xóa sinh viên");
        println!("3. Cập nhật thông tin sinh viên");
        println!("4. Tìm kiếm sinh viên");

        4
    }
    fn parse_choice(choice: i32, conn: &Connection) -> Option<Self> where Self: Sized {
        match choice {
            1 => Some(Operation::new_operation_add(conn)),
            2 => Some(Operation::new_operation_delete()),
            3 => Some(Operation::new_operation_update()),
            4 => Some(Operation::new_operation_search()),
            _ => None,
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
        println!("Đã thêm 1 sinh viên");
        new_student.print();
        println!();
    }
}

fn search_student(conn: &Connection, id_or_name: &str) -> Option<Student> {
    if let Ok(student) = conn.query_row(
        "SELECT id, name, dob, phone, address, email, status, gender, faculty, enrolled_year, program FROM Student WHERE id = ? OR name LIKE ?",
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
    conn.execute("UPDATE Student SET name = ?, dob = ?, phone = ?, address = ?, email = ?, status = ?, gender = ?, faculty = ?, enrolled_year = ?, program = ? WHERE id = ?", rusqlite::params![
                 student.name, student.dob, student.phone, student.address, student.email, student.status.id, student.gender, student.faculty.id, student.enrolled_year, student.program.id,
                 student.id
                 ]).unwrap();
}

fn delete_student(conn: &Connection, id: &str) -> bool {
    let result = conn.execute("DELETE FROM Student WHERE id = ?", [id]).unwrap();
    result == 1
}

fn main() {
    let conn = Connection::open("data.db").unwrap();
    conn.execute_batch(&fs::read_to_string("migrate.sql").unwrap()).unwrap();

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
        }
        println!();
    }
}
