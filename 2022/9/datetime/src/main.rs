mod datetime;
use datetime::Date;
use datetime::DateTime;
use datetime::Time;

fn main() {
    let date_time = DateTime::create(2022, 09, 17, 02, 53, 00);
    println!("Datetime: {}", date_time);
    println!("Date: {}", date_time.date);
    println!("Time: {}", date_time.time);
    let date = Date::new(2022u16, 09, 17);
    println!("Date: {}", date);
    let time = Time::new(03, 51, 00);
    println!("Time: {}", time);

    // error[E0283]: type annotations needed
    //     --> src/main.rs:17:16
    //     |
    // 17 |     let time = Time::new("03".parse().unwrap(), "51".parse().unwrap(), "00".parse().unwrap());
    //     |                ^^^^^^^^^ cannot infer type for type parameter `impl Into<u8>` declared on the associated function `new`
    //     |
    //     = note: cannot satisfy `_: Into<u8>`
    // note: required by a bound in `Time::new`
    let time = Time::new(
        "03".parse().unwrap(),
        "51".parse().unwrap(),
        "00".parse().unwrap(),
    );
    println!("Time: {}", time);
}
