#![allow(static_mut_refs)]
use enum_count::EnumCount;
use crate::{static_assert, enum_count_of};
use rusqlite::{Connection};
use crate::io::*;
use regex::Regex;
use std::fs::File;
use std::io::{BufReader, BufWriter};

#[derive(EnumCount)] // create const ENUM_CONFIGOPTION_COUNT = len(ConfigOption)
pub enum ConfigOption {
    EmailDomain(String),
    PhonePattern(String),
    StudentDeletionTime(i64),
    StatusRule,
    ToggleEmailRule(bool),
    TogglePhoneRule(bool),
    ToggleStatusRule(bool),
    ToggleStudentDeletionRule(bool),
}

impl SelectableEnum for ConfigOption {
    fn print_choices(_conn: &Connection) -> usize {
        static_assert!(enum_count_of!(ConfigOption) == 8, "Changed this");
        const choices: &[&str] = &[
            "Đổi tên miền email",
            "Đổi định dạng số điện thoại",
            "Đổi luật khi thay đổi tình trạng sinh viên",
            "Đổi khoảng thời gian cho phép xóa học sinh từ khi mới tạo",
            "Bật/tắt tên miền email",
            "Bật/tắt cấm xóa học sinh sau một thời gian",
            "Bật/tắt định dạng số điện thoại",
            "Bật/tắt luật thay đổi tình trạng sinh viên",
        ];
        for i in 0..choices.len() {
            println!("{}. {}", i+1, choices[i]);
        }
        choices.len()
    }

    fn parse_choice(choice: i32, _conn: &Connection) -> Option<Self> where Self: Sized {
        static_assert!(enum_count_of!(ConfigOption) == 8);
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
            3 => Some(
                ConfigOption::StudentDeletionTime(read_number_until_correct("Nhập thời gian (phút) cho phép xóa học sinh (> 30)", 30, std::i64::MAX))
            ),
            4 => todo!(),
            5 => Some(ConfigOption::ToggleEmailRule(read_boolean("Bật", "Tắt"))),
            6 => Some(ConfigOption::TogglePhoneRule(read_boolean("Bật", "Tắt"))),
            7 => Some(ConfigOption::ToggleStatusRule(read_boolean("Bật", "Tắt"))),
            8 => Some(ConfigOption::ToggleStudentDeletionRule(read_boolean("Bật", "Tắt"))),
            _ => None
        }
    }
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

#[derive(serde::Serialize, serde::Deserialize, Clone)]
struct Rule<T> {
    rule: T,
    disabled: bool,
}
impl<T> std::ops::Deref for Rule<T> {
    type Target = T;
    fn deref(&self) -> &<Self as std::ops::Deref>::Target {
        &self.rule
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct BusinessRule {
    email_domain: Option<Rule<String>>,
    phone_pattern: Option<Rule<String>>,
    student_deletion_time: Option<Rule<i64>>,
}
// NOTE: MUST NOT RUN MULTITHREADED
static mut business_rule: BusinessRule = BusinessRule {
    email_domain: None,
    phone_pattern: None,
    student_deletion_time: None,
};
impl BusinessRule {
    pub fn print(&mut self) {
        println!("Tên miền email cho phép: {}", self.email_domain.as_deref().map_or("None", |x| x));
        println!("Định dạng số điện thoại: {}", self.email_domain.as_deref().map_or("None", |x| x));
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
            if phone_pattern.disabled {
                return None;
            }
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
                panic!("Invalid phone number pattern '{}'", phone_pattern.rule);
            }
        }
        regex.push('$');
        // println!("{regex}");
        Some(regex)
    }

    pub fn student_deletion_time() -> Option<i64> {
        unsafe {
            if let Some(x) = &business_rule.student_deletion_time {
                if !x.disabled {
                    return Some(x.rule);
                }
            }
            return None
        }
    }

    pub fn email() -> Option<&'static String> {
        unsafe {
            if let Some(x) = &business_rule.email_domain {
                if !x.disabled {
                    return Some(&x.rule);
                }
            }
            return None
        }
    }

    pub fn set_email(email: String) {
        unsafe {
            if let Some(x) = &mut business_rule.email_domain {
                x.rule = email;
            }
        }
    }

    pub fn set_phone_number_pattern(pattern: String) {
        unsafe {
            if let Some(x) = &mut business_rule.phone_pattern {
                x.rule = pattern;
            }
        }
    }

    pub fn set_email_rule_enable(enable: bool) {
        unsafe {
            if let Some(x) = &mut business_rule.email_domain {
                x.disabled = !enable;
            }
        }
    }

    pub fn set_phone_rule_enable(enable: bool) {
        unsafe {
            if let Some(x) = &mut business_rule.phone_pattern {
                x.disabled = !enable;
            }
        }
    }

    pub fn set_status_rule_enable(_enable: bool) {
        todo!();
    }

    pub fn set_student_deletion_time(time: i64) {
        unsafe {
            if let Some(x) = &mut business_rule.student_deletion_time {
                x.rule = time;
            }
        }
    }

    pub fn set_student_deletion_time_enable(enable: bool) {
        unsafe {
            if let Some(x) = &mut business_rule.student_deletion_time {
                x.disabled = !enable;
            }
        }
    }
}
