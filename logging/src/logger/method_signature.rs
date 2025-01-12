use std::fmt::Display;

pub struct LogHeader {
    pub file_name: String,
    pub line_number: u32,
}

impl Default for LogHeader {
    fn default() -> Self {
        LogHeader {
            file_name: "UNKNOWN".to_string(),
            line_number: 0,
        }
    }
}

impl Display for LogHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[File Name: {}, line number: {}]",
            self.file_name, self.line_number
        )
    }
}

impl LogHeader {
    pub fn build(name: &str, line_number: u32) -> Self {
        LogHeader {
            file_name: name.to_string(),
            line_number,
        }
    }
}

