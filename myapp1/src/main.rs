use mylib1;

fn main() {
    let num = 888;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        mylib1::add_one(num)
    );
}