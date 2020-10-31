use mylib1;

fn main() {
    let num = 888;
    println!(
        "Hello, world! {} plus one is {}!",
        num,
        mylib1::add_one(num)
    );
    println!(
        "Hello, world! {} plus two is {}!",
        num,
        mylib1::add_two(num)
    );
}
