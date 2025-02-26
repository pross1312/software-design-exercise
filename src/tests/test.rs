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
