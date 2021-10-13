mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;

fn main() {
    println!("Daily Rust Exercise");
    let d = day_4::is_valid("055 444 285");
    println!("{}", d);

    let d = day_5::check("055 444 285");
    println!("{}", d);
}
