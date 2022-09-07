static mut X: Vec<String> = Vec::new();

// just add comment
pub fn append(s: impl Into<String>) {
    unsafe {
        X.push(s.into());
    }
}

pub fn get() -> &'static Vec<String> {
    unsafe { &X }
}
