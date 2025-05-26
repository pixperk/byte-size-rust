pub trait InfoProvider {
    fn key(&self) -> &'static str;
    fn value(&self) -> String;
}
