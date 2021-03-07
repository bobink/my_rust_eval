use std::str::Chars;
use super::reader::Reader;

struct StringReader {
    str: String
}

struct StringReaderIterator<'a> {
    chars: Chars<'a>,
}

impl StringReader {
    fn new(str: String) -> StringReader {
        StringReader {
            str
        }
    }
}

impl Reader for StringReader {
    fn chars<'a>(&'a self) -> Box<dyn Iterator<Item=char> + 'a> {
        return Box::new(StringReaderIterator::new(self.str.chars()));
    }
}

impl<'a> StringReaderIterator<'a> {
    fn new(chars: Chars<'a>) -> StringReaderIterator {
        StringReaderIterator {
            chars
        }
    }
}

impl<'a> Iterator for StringReaderIterator<'a> {
    type Item = char;

    fn next(&mut self) -> Option<char> {
        return self.chars.next();
    }
}

pub fn string_reader(s: &str) -> Box<dyn Reader> {
    return Box::new(StringReader::new(String::from(s)));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ops::DerefMut;

    fn char_vec_of_string(s: &str) -> Vec<char> {
        return string_reader(s).chars().deref_mut().collect();
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