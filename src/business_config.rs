#![allow(static_mut_refs)]
use enum_count::EnumCount;
use crate::static_assert;
use rusqlite::{Connection};
use crate::io::*;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(EnumCount)] // create const ENUM_CONFIGOPTION_COUNT = len(ConfigOption)
pub enum ConfigOption {
    EmailDomain(String),
    PhonePattern(String),
    StatusRule,
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
        match choice {
            1 => Some(ConfigOption::EmailDomain(read_string_new("Nhập tên miền mới"))),
            2 => Some(ConfigOption::PhonePattern(read_string_new("Nhập định dạng mới cho số điện thoại (vd 0[3|5|7|8|9]xxxxxxxx)"))),
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
        unsafe {
            let Some(_phone_pattern) = &business_rule.phone_pattern else {
                return None;
            };
        }
        todo!();
    }

    pub fn email() -> Option<&'static String> {
        unsafe {
            return business_rule.email_domain.as_ref();
        }
    }

    pub fn set_email(_email: &str) {
        todo!();
    }

    pub fn set_phone_number_pattern(_pattern: &str) {
        todo!();
    }
}
