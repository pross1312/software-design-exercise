use crate::*;
// NOTE: RUN THESES TEST WITH 1 THREAD ONLY
#[test]
fn test_validate_email_domain_valid() {
    if let Some(_) = validate_email_domain("fit.hcmus.edu.vn") {
        panic!("Email domain must match here");
    }
}

#[test]
fn test_validate_email_domain_invalid() {
    if let None = validate_email_domain("@fit.hcmus.edu.vn") {
        panic!("Email domain must not match here");
    }
}

#[test]
fn test_validate_phone_pattern_invalid() {
    if let None = validate_phone_number_pattern("zxc123") {
        panic!("Phone pattern must not match here");
    }
}

#[test]
fn test_validate_phone_pattern_valid() {
    if let Some(_) = validate_phone_number_pattern("0[3|5|7|8|9]xxxxxxxx") {
        panic!("Phone pattern must match here");
    }
}

#[test]
#[should_panic(expected = "Invalid phone number pattern '0[3|5|7|8|9]ABxCxxDx'")]
fn test_validate_phone_when_invalid_pattern() {
    BusinessRule::set_phone_number_pattern("0[3|5|7|8|9]ABxCxxDx".to_string());
    validate_phone("0908063538");
}

#[test]
fn test_validate_phone_valid() {
    BusinessRule::set_phone_number_pattern("0[3|5|7|8|9]xxxxxxxx".to_string());
    if let Some(_) = validate_phone("0908063538") {
        panic!("Phone number must match");
    }
}

#[test]
fn test_validate_phone_invalid() {
    let test = "0[3|5|7|8|9]xxxxxxxx".to_owned();
    BusinessRule::set_phone_number_pattern(test);
    if let None = validate_phone("432") {
        panic!("Phone number must not match here");
    }
}

#[test]
fn test_validate_email_invalid() {
    BusinessRule::set_email("fit.hcmus.edu.vn".to_string());
    if None == validate_email("tuong@gmail.com") {
        panic!("Email must not match here");
    }
}

#[test]
fn test_validate_email_valid() {
    BusinessRule::set_email("fit.hcmus.edu.vn".to_string());
    if let Some(_) = validate_email("tuong@fit.hcmus.edu.vn") {
        panic!("Email must match here");
    }
}

#[test]
fn test_validate_date_invalid() {
    if let None = validate_date("130/12/2003") {
        panic!("Must be invalid date");
    }
}

#[test]
fn test_validate_date_valid() {
    if let Some(_) = validate_date("13/12/2003") {
        panic!("Must be invalid date");
    }
}

#[test]
#[should_panic(expected = "Could not find end tag")]
fn test_template_render_no_end_tag() {
    Template::render("${}", std::collections::HashMap::from([
    ]));
}

#[test]
#[should_panic(expected = "Could not find value for `tuong`")]
fn test_tempate_render_missing_parametter() {
    Template::render("${tuong}$", std::collections::HashMap::from([
    ]));
}

#[test]
fn test_tempate_render_valid() {
    let rendered = Template::render("${tuong}$", std::collections::HashMap::from([
        ("tuong", "123")
    ]));
    assert_eq!(rendered, "123");
}

struct TestDb {
    conn: Connection,
    db_file_path: String,
}
impl TestDb {
    fn new(path: String) -> Self {
        let conn = Connection::open(&path).unwrap();
        conn.execute_batch(&fs::read_to_string(crate::MIGRATION_SCRIPT).unwrap()).unwrap();
        Self {
            conn,
            db_file_path: path,
        }
    }
}

// remove db file on drop
impl Drop for TestDb {
    fn drop(&mut self) {
        std::fs::remove_file(&self.db_file_path).unwrap();
    }
}

#[test]
fn test_default_data() {
    let db = TestDb::new("test_default_data.db".to_string());
    assert!(Status::get_all(&db.conn).len() != 0, "Expect default data");
    assert!(Program::get_all(&db.conn).len() != 0, "Expect default data");
    assert!(Faculty::get_all(&db.conn).len() != 0, "Expect default data");
}

#[test]
fn test_faculty_insert() {
    let db = TestDb::new("test_faculty_insert.db".to_string());
    assert!(Faculty::add(&db.conn, "Test Faculty") != 0);
}

#[test]
fn test_status_insert() {
    let db = TestDb::new("test_status_insert.db".to_string());
    assert!(Status::add(&db.conn, "Test Status") != 0);
}

#[test]
fn test_program_insert() {
    let db = TestDb::new("test_program_insert.db".to_string());
    assert!(Program::add(&db.conn, "Test Program") != 0);
}

#[test]
fn test_student_insert() {
    // NOTE: faculty, status, and program all have some initial values already so foreign
    // constraint is satisfied if id = 1
    let db = TestDb::new("test_student_insert_default.db".to_string());
    let mut student = Student::new();
    student.id = "21127720".to_string();
    student.name = "Tuong".to_string();
    Student::add(&db.conn, &student);
    let students = Student::get_all(&db.conn);
    assert_eq!(students.len(), 1);
    assert_eq!(&students[0].name, "Tuong");
}

#[test]
fn test_search_student_by_name_or_id() {
    const mock_id: &str = "21127720";
    const mock_name: &str = "Tuong";
    let db = TestDb::new("test_student_insert_default.db".to_string());
    let mut student = Student::new();
    student.id = mock_id.to_string();
    student.name = mock_name.to_string();
    Student::add(&db.conn, &student);
    let searched_student = crate::search_student(&db.conn, mock_id).unwrap();
    assert_eq!(searched_student.id, mock_id);
    assert_eq!(searched_student.name, mock_name);
}
