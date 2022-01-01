pub trait Reader {
    fn chars<'a>(&'a self) -> Box<dyn Iterator<Item=char> + 'a>;
}
