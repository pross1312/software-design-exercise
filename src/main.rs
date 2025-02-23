#![allow(dead_code)]
mod io {
    mod reader;
    mod file_format;
    mod writer;
    mod selectable_enum;
    pub use reader::*;
    pub use file_format::*;
    pub use selectable_enum::*;
}
mod data {
    mod faculty;
    mod student;
    mod program;
    mod status;
    mod gender;
    pub use faculty::*;
    pub use student::*;
    pub use status::*;
    pub use program::*;
    pub use gender::*;
}
use std::env;
use rusqlite::{Connection};
use std::io::{BufWriter, BufReader, Write};
use std::fs::{self, File};
use std::path::Path;

use io::*;
use data::*;

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
        read_string_until_correct("Nhập mã số sinh viên", &mut new_student.id, validate_id);
        read_string("Nhập họ tên", &mut new_student.name).unwrap();
        read_string_until_correct("Nhập ngày tháng năm sinh (dd/mm/yyyy)", &mut new_student.dob, validate_date);
        read_string_until_correct("Nhập số điện thoại", &mut new_student.phone, validate_phone);
        new_student.enrolled_year = read_number_until_correct("Nhập khóa (1990, 2025)", 1990, 2025);
        new_student.gender = read_enum_until_correct("Nhập giới tính", conn);
        new_student.faculty = read_enum_until_correct("Nhập khoa", conn);
        new_student.program = read_enum_until_correct("Nhập chương trình", conn);
        new_student.status = read_enum_until_correct("Nhập tình trạng", conn);
        read_string("Nhập địa chỉ", &mut new_student.address).unwrap();
        read_string_until_correct("Nhập Email", &mut new_student.email, validate_email);

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
            UpdateStudentOption::UpdateDob => { read_string_until_correct("Nhập ngày sinh mới (dd/mm/yyyy)", &mut student.dob, validate_date); },
            UpdateStudentOption::UpdatePhone => { read_string_until_correct("Nhập số điện thoại mới", &mut student.phone, validate_phone); },
            UpdateStudentOption::UpdateAddress => { read_string("Nhập địa chỉ mới", &mut student.address).unwrap(); },
            UpdateStudentOption::UpdateEmail => { read_string_until_correct("Nhập email mới", &mut student.email, validate_email); },
            UpdateStudentOption::UpdateStatus => { student.status = read_enum_until_correct("Nhập trạng thái mới", conn); },
            UpdateStudentOption::UpdateGender => { student.gender = read_enum_until_correct("Nhập giới tính mới", conn); },
            UpdateStudentOption::UpdateFaculty => { student.faculty = read_enum_until_correct("Nhập khoa mới", conn); },
            UpdateStudentOption::UpdateEnrolledYear => { student.enrolled_year = read_number_until_correct("Nhập khóa mới (1990, 2025)", 1990, 2025); },
            UpdateStudentOption::UpdateProgram => { student.program = read_enum_until_correct("Nhập khoa mới", conn); },
            UpdateStudentOption::Done => { break; },
        }
    }
}

fn search_student(conn: &Connection, id_or_name: &str) -> Option<Student> {
    log!("Search for student with id or name {}", id_or_name);
    if let Ok(student) = conn.query_row(
        "SELECT id, name, dob, phone, address, email, status, gender, faculty, enrolled_year, program FROM Student WHERE LOWER(id) = LOWER(?) OR LOWER(name) LIKE LOWER(?)",
        [id_or_name, id_or_name],
        |row| Student::map_row(conn, row)) {
        Some(student)
    } else {
        None
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
        Student::map_row(conn, row)
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

fn export_data(conn: &Connection, file_name: &str, format: FileFormat) {
    log!("Export data to {}", file_name);
    let path = Path::new(&file_name).with_extension(format.extension());
    let data = DataFormat {
        statuses: Status::get_all(conn),
        programs: Program::get_all(conn),
        faculties: Faculty::get_all(conn),
        students: Student::get_all(conn),
    };

    let mut writer = BufWriter::new(File::create(&path).unwrap());
    match format {
        FileFormat::Json => {
            serde_json::to_writer(writer, &data).unwrap();
        },
        FileFormat::Xml => {
            writer.write_all(
                quick_xml::se::to_string(&data).unwrap().as_bytes()
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
        Faculty::add_many(conn, &data.faculties);
        Status::add_many(conn, &data.statuses);
        Program::add_many(conn, &data.programs);
        Student::add_many(conn, &data.students);
    } else {
        println!("{file_name} does not exist");
    }
}

const DB_PATH: &str = "data.db";
const MIGRATION_SCRIPT: &str = "migrate.sql";
const VERSION: &str = "2.0";
const BUILD_DATE: &str = "19/02/2025";
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
                    Student::add(&conn, &new_student);
                }
            },
            Operation::UpdateStudent(id) => {
                if let Some(mut student) = search_student(&conn, &id) {
                    println!("Cập nhập thông tin mới cho sinh viên");
                    update_student_fields(&mut student, &conn);
                    println!("Thông tin của sinh viên sau khi sửa");
                    student.print();
                    Student::update(&conn, &id, &student);
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
                if Student::delete(&conn, &id) {
                    println!("Xóa thành công sinh viên với mã số {}", id);
                } else {
                    println!("Không thể tìm thấy sinh viên với mã số {}", id);
                }
            },
            Operation::AddNewFaculty(name) => {
                Faculty::add(&conn, &name);
            },
            Operation::AddNewStatus(name) => {
                Status::add(&conn, &name);
            },
            Operation::AddNewProgram(name) => {
                Program::add(&conn, &name);
            },
            Operation::UpdateFaculty(faculty) => {
                Faculty::update(&conn, &faculty);
            },
            Operation::UpdateStatus(status) => {
                Status::update(&conn, &status);
            },
            Operation::UpdateProgram(program) => {
                Program::update(&conn, &program);
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
