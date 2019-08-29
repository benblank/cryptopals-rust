mod cryptopals;
mod exercise1;
mod exercise2;
mod exercise3;
mod exercise4;
mod exercise5;
mod exercise6;
mod exercise7;
mod rijndael;

use std::env;

fn main() {
    for arg in env::args().skip(1) {
        match arg.as_ref() {
            "1" => exercise1::run_and_print(),
            "2" => exercise2::run_and_print(),
            "3" => exercise3::run_and_print(),
            "4" => exercise4::run_and_print(),
            "5" => exercise5::run_and_print(),
            "6" => exercise6::run_and_print(),
            "7" => exercise7::run_and_print(),
            _ => eprintln!("Could not find exercise number {}.", arg),
        }
    }
}
