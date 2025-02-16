#![allow(dead_code)]
use std::io;
use std::fmt::Display;
use std::io::Write;
trait SelectableEnum {
    fn print_choices() -> usize;
    fn parse_choice(choice: i32) -> Option<Self> where Self: Sized;
}

enum Faculty {
    Law,
    CommercialEnglish,
    Japanese,
    France,
}
impl Faculty {
    fn value(&self) -> &'static str {
        match *self {
            Faculty::Law => "Khoa Luật",
            Faculty::CommercialEnglish => "Khoa Tiếng Anh thương mại",
            Faculty::Japanese => "Khoa Tiếng Nhật",
            Faculty::France => "Khoa Tiếng Pháp",
        }
    }
}
impl SelectableEnum for Faculty {
    fn print_choices() -> usize {
        println!("1. Khoa Luật");
        println!("2. Khoa Tiếng Anh thương mại");
        println!("3. Khoa Tiếng Nhật");
        println!("4. Khoa Tiếng Pháp");

        4
    }
    fn parse_choice(choice: i32) -> Option<Self> {
        match choice {
            1 => Some(Faculty::Law),
            2 => Some(Faculty::CommercialEnglish),
            3 => Some(Faculty::Japanese),
            4 => Some(Faculty::France),
            _ => None,
        }
    }
}

enum Gender {
    Male, Female
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
    fn print_choices() -> usize {
        println!("1. Nam");
        println!("2. Nữ");

        2
    }
    fn parse_choice(choice: i32) -> Option<Self> {
        match choice {
            1 => Some(Gender::Male),
            2 => Some(Gender::Female),
            _ => None,
        }
    }
}

enum Program {
    Normal,
    HighQuality,
}
impl Program {
    fn value(&self) -> &'static str {
        match *self {
            Program::Normal => "Thường",
            Program::HighQuality => "Chất lượng cao",
        }
    }
}
impl SelectableEnum for Program {
    fn print_choices() -> usize {
        println!("1. Thường");
        println!("2. Chất lượng cao");

        2
    }
    fn parse_choice(choice: i32) -> Option<Self> {
        match choice {
            1 => Some(Program::Normal),
            2 => Some(Program::HighQuality),
            _ => None,
        }
    }
}


enum Status {
    Graduated,
    Suspended,
    DroppedOut,
    OnGoing,
}
impl Status {
    fn value(&self) -> &'static str {
        match *self {
            Status::Graduated => "Đã tốt nghiệp",
            Status::Suspended => "Tạm dừng học",
            Status::DroppedOut => "Đã thôi học",
            Status::OnGoing => "Đang học",
        }
    }
}
impl SelectableEnum for Status {
    fn print_choices() -> usize {
        println!("1. Đã tốt nghiệp");
        println!("2. Tạm dừng học");
        println!("3. Đã thôi học");
        println!("4. Đang học");

        4
    }
    fn parse_choice(choice: i32) -> Option<Self> {
        match choice {
            1 => Some(Status::Graduated),
            2 => Some(Status::Suspended),
            3 => Some(Status::DroppedOut),
            4 => Some(Status::OnGoing),
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
            status: Status::OnGoing,
            gender: Gender::Male,
            faculty: Faculty::Law,
            enrolled_year: 2025,
            program: Program::Normal,
        }
    }

    fn print(&self) {
        println!("Mã số sinh viên: {}", self.id);
        println!("Họ tên: {}", self.name);
        println!("Ngày tháng năm sinh: {}", self.dob);
        println!("Giới tính: {}", self.gender.value());
        println!("Khoa: {}", self.faculty.value());
        println!("Khóa: {}", self.enrolled_year);
        println!("Chương trình: {}", self.program.value());
        println!("Địa chỉ: {}", self.address);
        println!("Email: {}", self.email);
        println!("Tình trạng sinh viên: {}", self.status.value());
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

fn read_string(prompt: &'static str, buffer: &mut String) -> Result<usize, std::io::Error> {
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

fn read_enum_until_correct<T: SelectableEnum>(prompt: impl Display) -> T {
    let mut input = String::new();
    loop {
        input.clear();
        println!("{}", prompt);
        T::print_choices();
        match read_string(&"Chọn một trong những giá trị trên", &mut input) {
            Ok(_) => match input.parse::<i32>() {
                Ok(choice) => {
                    if let Some(value) = T::parse_choice(choice) {
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

fn read_number_until_correct(prompt: &'static str, start: i32, end: i32) -> i32 {
    let mut input = String::new();
    loop {
        input.clear();
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

fn read_string_until_correct(prompt: &'static str, buffer: &mut String, validate: &dyn Fn(&str) -> Option<&'static str>) -> usize {
    loop {
        buffer.clear();
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

    fn new_operation_add() -> Self {
        let mut new_student = Student::new();
        read_string_until_correct("Nhập mã số sinh viên", &mut new_student.id, &validate_id);
        read_string("Nhập họ tên", &mut new_student.name).unwrap();
        read_string_until_correct("Nhập ngày tháng năm sinh (dd/mm/yyyy)", &mut new_student.dob, &validate_date);
        read_string_until_correct("Nhập số điện thoại", &mut new_student.phone, &validate_phone);
        new_student.enrolled_year = read_number_until_correct("Nhập khóa (1990, 2025)", 1990, 2025);
        new_student.gender = read_enum_until_correct("Nhập giới tính");
        new_student.faculty = read_enum_until_correct("Nhập khoa");
        new_student.program = read_enum_until_correct("Nhập chương trình");
        new_student.status = read_enum_until_correct("Nhập tình trạng");
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
    fn print_choices() -> usize {
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
    fn parse_choice(choice: i32) -> Option<Self> where Self: Sized {
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

fn update_student(student: &mut Student) {
    loop {
        let option = read_enum_until_correct::<UpdateStudentOption>("");
        match option {
            UpdateStudentOption::UpdateName => { read_string("Nhập tên mới", &mut student.name).unwrap(); },
            UpdateStudentOption::UpdateDob => { read_string_until_correct("Nhập ngày sinh mới (dd/mm/yyyy)", &mut student.dob, &validate_date); },
            UpdateStudentOption::UpdatePhone => { read_string_until_correct("Nhập số điện thoại mới", &mut student.phone, &validate_phone); },
            UpdateStudentOption::UpdateAddress => { read_string("Nhập địa chỉ mới", &mut student.address).unwrap(); },
            UpdateStudentOption::UpdateEmail => { read_string_until_correct("Nhập email mới", &mut student.email, &validate_email); },
            UpdateStudentOption::UpdateStatus => { student.status = read_enum_until_correct("Nhập trạng thái mới"); },
            UpdateStudentOption::UpdateGender => { student.gender = read_enum_until_correct("Nhập giới tính mới"); },
            UpdateStudentOption::UpdateFaculty => { student.faculty = read_enum_until_correct("Nhập khoa mới"); },
            UpdateStudentOption::UpdateEnrolledYear => { student.enrolled_year = read_number_until_correct("Nhập khóa mới (1990, 2025)", 1990, 2025); },
            UpdateStudentOption::UpdateProgram => { student.program = read_enum_until_correct("Nhập khoa mới"); },
            UpdateStudentOption::Done => { break; },
        }
    }
}

impl SelectableEnum for Operation {
    fn print_choices() -> usize {
        println!("1. Thêm sinh viên mới");
        println!("2. Xóa sinh viên");
        println!("3. Cập nhật thông tin sinh viên");
        println!("4. Tìm kiếm sinh viên");

        4
    }
    fn parse_choice(choice: i32) -> Option<Self> where Self: Sized {
        match choice {
            1 => Some(Operation::new_operation_add()),
            2 => Some(Operation::new_operation_delete()),
            3 => Some(Operation::new_operation_update()),
            4 => Some(Operation::new_operation_search()),
            _ => todo!(),
        }
    }
}
fn main() {
    "123".parse::<i32>().unwrap();
    let mut students: Vec<Student> = Vec::new();
    loop {
        match read_enum_until_correct("Chọn hành động") {
            Operation::AddNewStudent(new_student) => {
                println!("Đã thêm 1 sinh viên");
                new_student.print();
                println!();
                students.push(new_student);
            },
            Operation::UpdateStudent(id) => {
                if let Some(index) = students.iter().position(|student| student.id == id) {
                    println!("Cập nhập thông tin mới cho sinh viên");
                    students[index].print();
                    update_student(&mut students[index]);
                    println!("Thông tin của sinh viên sau khi sửa");
                    students[index].print();
                    println!();
                } else {
                    println!("Không thể tìm thấy sinh viên có mã số {}", id);
                }
            },
            Operation::SearchStudent(search) => {
                if let Some(index) = students.iter().position(|student| student.id == search || student.name == search) {
                    println!("Sinh viên cần tìm là");
                    students[index].print();
                    println!();
                } else {
                    println!("Không thể tìm thấy sinh viên có mã số hoặc tên {}", search);
                }
            },
            Operation::DeleteStudent(id) => {
                if let Some(index) = students.iter().position(|student| student.id == id) {
                    students.swap_remove(index);
                    println!("Xóa thành công sinh viên với mã số {}", id);
                } else {
                    println!("Không thể tìm thấy sinh viên với mã số {}",id);
                }
            },
        }
    }
}
