use anyhow::Result;

pub struct File {
    pub path: String, // /path/to/file123.ext
    pub name: String, // file123.ext
    pub number: u32, // 123
    pub data: Vec<u8>,
}

// Starts_with and ends_width should be lowercase. Eg 'vgagr' and '.dat' respectively.
pub fn find(folder: &str, starts_with: &str, ends_width: &str) -> Result<Vec<File>> {
    let mut files: Vec<File> = Vec::new();
    let entries = std::fs::read_dir(folder)?;
    for entry in entries {
        let Ok(entry) = entry else { continue };
        let path = entry.path().to_string_lossy().to_string();
        let name = entry.file_name().to_string_lossy().to_string();
        let lower_name = name.to_lowercase();
        if lower_name.starts_with(starts_with) && lower_name.ends_with(ends_width) {
            let number = number_ignoring_non_digits(&name);
            let data = std::fs::read(&path)?;
            files.push(File { path, name, number, data });
        }
    }
    Ok(files)
}

fn number_ignoring_non_digits(s: &str) -> u32 {
    let mut value: u32 = 0;
    for char in s.chars() {
        if char.is_ascii_digit() {
            let digit = char as u32 - '0' as u32;
            value = value * 10 + digit as u32;
        }
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_number_ignoring_non_digits() {
        assert_eq!(number_ignoring_non_digits("foobar"), 0);
        assert_eq!(number_ignoring_non_digits("1foobar"), 1);
        assert_eq!(number_ignoring_non_digits("foo1bar"), 1);
        assert_eq!(number_ignoring_non_digits("foobar1"), 1);
        assert_eq!(number_ignoring_non_digits("123foobar"), 123);
        assert_eq!(number_ignoring_non_digits("foo123bar"), 123);
        assert_eq!(number_ignoring_non_digits("foobar123"), 123);
    }
}
