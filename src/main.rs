mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;

fn main() {
    println!("Daily Rust Exercise");
    let d = day_4::is_valid("055 444 285");
    println!("day4: {}", d);

    let d = day_5::check("055 444 285");
    println!("day5: {}", d);

    let d = day_7::movie(18, 40, 0.47);
    println!("day7: {}", d);

    let d = day_8::annotate(&["  *  ", "  *  ", "*****", "  *  ", "  *  "]);
    println!("day8: {:?}", d);
}
