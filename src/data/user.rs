pub trait UserData {
    fn get(name: String) -> Option<String>;
}
