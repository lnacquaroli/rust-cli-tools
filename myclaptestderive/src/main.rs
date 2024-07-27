use clap::Parser;
use myclaptestderive::Register;

fn main() {
    let args = Register::parse();
    println!("{:#?}", args);
}
