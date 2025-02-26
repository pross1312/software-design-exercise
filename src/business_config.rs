#![allow(static_mut_refs)]
use enum_count::EnumCount;
use crate::static_assert;
use rusqlite::{Connection};
use crate::io::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(EnumCount)] // create const ENUM_CONFIGOPTION_COUNT = len(ConfigOption)
pub enum ConfigOption {
    EmailDomain(String),
    PhonePattern(String),
    StatusRule,
}

pub fn validate_email_domain(domain: &str) -> Option<String> {
    const domain_pattern: &str = r"^[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    let regex = Regex::new(domain_pattern).unwrap();
    if regex.is_match(domain) {
        None
    } else {
        Some("Tên miền không hợp lệ, vui lòng nhập lại".to_string())
    }
}

pub fn validate_phone_number_pattern(pattern: &str) -> Option<String> {
    const phone_rule_pattern: &str = r"^(?:\[\d(?:\|\d)*\]|x|\d)+$";
    let regex = Regex::new(phone_rule_pattern).unwrap();
    if regex.is_match(pattern) {
        None
    } else {
        Some("Định dạng số điện thoại không hợp lệ, vui lòng nhập lại".to_string())
    }
}

impl SelectableEnum for ConfigOption {
    fn print_choices(_conn: &Connection) -> usize {
        static_assert!(ENUM_CONFIGOPTION_COUNT == 3, "Changed this");
        println!("1. Đổi tên miền email");
        println!("2. Đổi định dạng số điện thoại");
        println!("3. Đổi luật khi thay đổi tình trạng sinh viên");
        3
    }

    fn parse_choice(choice: i32, _conn: &Connection) -> Option<Self> where Self: Sized {
        static_assert!(ENUM_CONFIGOPTION_COUNT == 3, "Changed this");
        let mut buffer = String::new();
        match choice {
            1 => Some({
                read_string_until_correct("Nhập tên miền mới", &mut buffer, validate_email_domain);
                ConfigOption::EmailDomain(buffer)
            }),
            2 => Some({
                read_string_until_correct("Nhập định dạng mới cho số điện thoại (vd 0[3|5|7|8|9]xxxxxxxx)", &mut buffer, validate_phone_number_pattern);
                ConfigOption::PhonePattern(buffer)
            }),
            3 => todo!(),
            _ => None
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessRule {
    email_domain: Option<String>,
    phone_pattern: Option<String>,
}
// NOTE: MUST NOT RUN MULTITHREADED
static mut business_rule: BusinessRule = BusinessRule {
    email_domain: None,
    phone_pattern: None,
};
impl BusinessRule {
    pub fn print(&self) {
        println!("Tên miền email cho phép: {}", self.email_domain.as_deref().unwrap_or("None"));
        println!("Định dạng số điện thoại: {}", self.email_domain.as_deref().unwrap_or("None"));
    }

    pub fn import(path: &str) {
        unsafe {
            business_rule = serde_json::from_reader(File::open(&path).and_then(|file| Ok(BufReader::new(file))).unwrap()).unwrap();
        }
    }

    pub fn export(path: &str) {
        unsafe {
            serde_json::to_writer(BufWriter::new(File::create(&path).unwrap()), &business_rule).unwrap();
        }
    }

    pub fn phone_regex() -> Option<String> {
        let phone_pattern = unsafe {
            let Some(phone_pattern) = &business_rule.phone_pattern else {
                return None;
            };
            phone_pattern
        };
        let mut regex = "^".to_string();
        let mut slices = phone_pattern.trim().chars();
        while let Some(ch) = slices.next() {
            if ch == '|' {
                continue;
            } else if ch == '[' || ch == ']' || ch.is_numeric() {
                regex.push(ch);
            } else if ch == 'x' {
                regex.push_str(r"\d");
            } else {
                panic!("Invalid phone number pattern '{phone_pattern}' {} {}", ch, phone_pattern.len());
            }
        }
        regex.push('$');
        // println!("{regex}");
        Some(regex)
    }

    pub fn email() -> Option<&'static String> {
        unsafe {
            return business_rule.email_domain.as_ref();
        }
    }

    pub fn set_email(email: String) {
        unsafe {
            business_rule.email_domain = Some(email);
        }
    }

    pub fn set_phone_number_pattern(pattern: String) {
        unsafe {
            business_rule.phone_pattern = Some(pattern);
        }
    }
}
