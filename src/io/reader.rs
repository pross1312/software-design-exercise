use rusqlite::{Connection};
use std::io::{self, Write};
use super::selectable_enum::SelectableEnum;

pub fn read_boolean(prompt_yes: &str, prompt_no: &str) -> bool {
    let mut input = String::new();
    loop {
        println!("1. {}", prompt_yes);
        println!("2. {}", prompt_no);
        match read_string(&"Chọn một trong những giá trị trên", &mut input) {
            Ok(_) => match input.parse::<i32>() {
                Ok(choice) => {
                    if choice == 1 {
                        return true;
                    } else if choice == 2 {
                        return false;
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

pub fn read_string(prompt: &str, buffer: &mut String) -> Result<usize, std::io::Error> {
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

pub fn read_string_new(prompt: &str) -> String {
    let mut buffer = String::new();
    read_string(prompt, &mut buffer).unwrap();
    return buffer;
}

pub fn read_enum_until_correct<T: SelectableEnum>(prompt: &str, conn: &Connection) -> T {
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

pub fn read_number_until_correct(prompt: &str, start: i32, end: i32) -> i32 {
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

pub type ValidateFn = fn(&str) -> Option<String>;
pub fn read_string_until_correct(prompt: &str, buffer: &mut String, validate: ValidateFn) -> usize {
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
