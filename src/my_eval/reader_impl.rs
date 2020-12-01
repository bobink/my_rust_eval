use std::str::Chars;
use super::reader::{Reader, ReaderIterator};

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
    fn chars<'a>(&'a self) -> Box<dyn ReaderIterator + 'a> {
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

impl<'a> ReaderIterator for StringReaderIterator<'a> {
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

    #[test]
    fn string_reader_bobink() {
        let reader = string_reader("bobink");
        let mut b = reader.chars();
        let chars = b.deref_mut();
        assert_eq!(Some('b'), chars.next());
        assert_eq!(Some('o'), chars.next());
        assert_eq!(Some('b'), chars.next());
        assert_eq!(Some('i'), chars.next());
        assert_eq!(Some('n'), chars.next());
        assert_eq!(Some('k'), chars.next());
        assert_eq!(None, chars.next());
    }

    #[test]
    fn string_reader_empty_string() {
        let reader = string_reader("");
        let mut b = reader.chars();
        let chars = b.deref_mut();
        assert_eq!(None, chars.next());
    }
}