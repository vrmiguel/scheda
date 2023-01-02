use scheda_parser::{parser::Parser, core::Schedule};

fn main() {
    dbg!(std::mem::size_of::<Schedule>());
    dbg!(Parser::new("when month feb to dec, day 12 or 16 to 17, weekday monday, hour 14 or 15 to 19, minute 22 to 45").parse_schedule().unwrap());
}
