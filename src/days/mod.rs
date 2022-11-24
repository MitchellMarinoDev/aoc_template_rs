mod d01;
mod d02;
mod d03;
mod d04;
mod d05;
mod d06;
mod d07;
mod d08;
mod d09;
mod d10;
mod d11;
mod d12;
mod d13;
mod d14;
mod d15;
mod d16;
mod d17;
mod d18;
mod d19;
mod d20;
mod d21;
mod d22;
mod d23;
mod d24;
mod d25;
mod day;
#[cfg(test)]
mod test;

pub use day::{Day, Solution};

pub const DAYS: [Day; 25] = [
    Day::new(01, d01::solve),
    Day::new(02, d02::solve),
    Day::new(03, d03::solve),
    Day::new(04, d04::solve),
    Day::new(05, d05::solve),
    Day::new(06, d06::solve),
    Day::new(07, d07::solve),
    Day::new(08, d08::solve),
    Day::new(09, d09::solve),
    Day::new(10, d10::solve),
    Day::new(11, d11::solve),
    Day::new(12, d12::solve),
    Day::new(13, d13::solve),
    Day::new(14, d14::solve),
    Day::new(15, d15::solve),
    Day::new(16, d16::solve),
    Day::new(17, d17::solve),
    Day::new(18, d18::solve),
    Day::new(19, d19::solve),
    Day::new(20, d20::solve),
    Day::new(21, d21::solve),
    Day::new(22, d22::solve),
    Day::new(23, d23::solve),
    Day::new(24, d24::solve),
    Day::new(25, d25::solve),
];
