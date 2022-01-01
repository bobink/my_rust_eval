pub trait Reader<'a, T> where
    T : Iterator<Item=char> + 'a {
    fn chars(&'a self) -> T;
}