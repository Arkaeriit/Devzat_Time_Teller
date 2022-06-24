mod tz;

fn main() {
    println!("{}", tz::time_at_tz("GBB").expect("nono"));
    println!("Hello, world!");
}

