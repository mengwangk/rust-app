use mylib1;
use mylib2;

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

    mylib2::eat_at_restaurant();
}
