use std::collections::HashMap;

pub struct Template;

impl Template {
    pub fn render(file_content: &str, values: HashMap<&str, &str>) -> String {
        let mut result = String::with_capacity(file_content.len());
        let mut content_slice = &file_content[0..];
        while let Some(start_index) = content_slice.find("${") {
            result.push_str(&content_slice[..start_index]);

            content_slice = &content_slice[start_index+2..];
            let Some(end_index) = content_slice.find("}$") else {
                panic!("Could not find end tag");
            };
            let key = &content_slice[..end_index];
            let Some(value) = values.get(key) else {
                panic!("Could not find value for `{}`", key);
            };
            result.push_str(value);
            content_slice = &content_slice[end_index+2..];
        }
        if content_slice.len() > 0 {
            result.push_str(content_slice);
        }
        result
    }
}
