use scheda_parser::parser::Parser;

fn main() {
    dbg!(Parser::new("when month feb to dec, day 12 or 16 to 17").parse_schedule().unwrap());
}
