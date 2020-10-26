pub struct User {
    pub username: String,
    pub email: String,
    pub sign_in_count: u64,
    pub active: bool,
}

pub fn hello_world() {
    println!("default lib")
}

pub struct WebPage<T> {
    pub contents: T,
}

impl<T> WebPage<T> {
    // A public constructor method
    pub fn new(contents: T) -> WebPage<T> {
        WebPage { contents: contents }
    }
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
