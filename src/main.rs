mod lib;

fn build_user(email: String, username: String) -> lib::User {
    lib::User {
        email: email,
        username: username,
        active: true,
        sign_in_count: 1,
    }
}

fn main() {
    lib::hello_world();

    let user1 = build_user(String::from("myemail@gmail.com"), String::from("userabc"));
    println!("{}", user1.email);

    let home_page = lib::WebPage::new("my home page ");
    print!("{}", home_page.contents)
}
