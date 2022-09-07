// just add comment
pub fn append(s: impl Into<String>) {
    static mut X: Vec<String> = Vec::new();
    unsafe {
        X.push(s.into());
    }
}
