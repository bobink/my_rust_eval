use std::str::Chars;
use super::reader::Reader;

pub struct StringReader {
    str: String
}

impl StringReader {
    fn new(s: &str) -> StringReader {
        StringReader {
            str: String::from(s)
        }
    }
}

impl<'a> Reader<'a, Chars<'a>> for StringReader {
    fn chars(&'a self) -> Chars<'a> {
        return self.str.chars()
    }
}

pub fn string_reader(s: &str) -> StringReader {
    return StringReader::new(s);
}

#[cfg(test)]
mod tests {
    use super::*;

    fn char_vec_of_string(s: &str) -> Vec<char> {
        return string_reader(s).chars().collect();
    }

    #[test]
    fn string_reader_bobink() {
        let actual = char_vec_of_string("bobink");
        assert_eq!(vec!['b', 'o', 'b', 'i', 'n', 'k'], actual);
    }

    #[test]
    fn string_reader_empty_string() {
        let actual = char_vec_of_string("");
        let expected : Vec<char> = vec![];
        assert_eq!(expected, actual);
    }
}