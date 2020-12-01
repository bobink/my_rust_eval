pub trait Reader {
    fn chars<'a>(&'a self) -> Box<dyn ReaderIterator + 'a>;
}

pub trait ReaderIterator {
    fn next(&mut self) -> Option<char>;
}
