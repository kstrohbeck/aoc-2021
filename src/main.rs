use std::{env, fs::File, io::Read};

mod day_01;
mod day_02;
// mod day_03;
// mod day_04;
// mod day_05;
// mod day_06;
// mod day_07;
// mod day_08;
// mod day_09;
// mod day_10;
// mod day_11;
// mod day_12;
// mod day_13;
// mod day_14;
// mod day_15;
// mod day_16;
// mod day_17;
// mod day_18;
// mod day_19;
// mod day_20;
// mod day_21;
// mod day_22;
// mod day_23;
// mod day_24;
// mod day_25;

fn main() {
    let mut args = env::args();
    let _name = args.next().unwrap();
    let day = args.next().unwrap().parse::<u32>().unwrap();
    let star = args.next().unwrap().parse::<u32>().unwrap();
    let filename = args.next().unwrap();

    let mut data = String::new();
    let mut file = File::open(filename).unwrap();
    file.read_to_string(&mut data).unwrap();

    let func = match (day, star) {
        (1, 1) => day_01::star_1,
        (1, 2) => day_01::star_2,
        (2, 1) => day_02::star_1,
        (2, 2) => day_02::star_2,
        // (3, 1) => day_03::star_1,
        // (3, 2) => day_03::star_2,
        // (4, 1) => day_04::star_1,
        // (4, 2) => day_04::star_2,
        // (5, 1) => day_05::star_1,
        // (5, 2) => day_05::star_2,
        // (6, 1) => day_06::star_1,
        // (6, 2) => day_06::star_2,
        // (7, 1) => day_07::star_1,
        // (7, 2) => day_07::star_2,
        // (8, 1) => day_08::star_1,
        // (8, 2) => day_08::star_2,
        // (9, 1) => day_09::star_1,
        // (9, 2) => day_09::star_2,
        // (10, 1) => day_10::star_1,
        // (10, 2) => day_10::star_2,
        // (11, 1) => day_11::star_1,
        // (11, 2) => day_11::star_2,
        // (12, 1) => day_12::star_1,
        // (12, 2) => day_12::star_2,
        // (13, 1) => day_13::star_1,
        // (13, 2) => day_13::star_2,
        // (14, 1) => day_14::star_1,
        // (14, 2) => day_14::star_2,
        // (15, 1) => day_15::star_1,
        // (15, 2) => day_15::star_2,
        // (16, 1) => day_16::star_1,
        // (16, 2) => day_16::star_2,
        // (17, 1) => day_17::star_1,
        // (17, 2) => day_17::star_2,
        // (18, 1) => day_18::star_1,
        // (18, 2) => day_18::star_2,
        // (19, 1) => day_19::star_1,
        // (19, 2) => day_19::star_2,
        // (20, 1) => day_20::star_1,
        // (20, 2) => day_20::star_2,
        // (21, 1) => day_21::star_1,
        // (21, 2) => day_21::star_2,
        // (22, 1) => day_22::star_1,
        // (22, 2) => day_22::star_2,
        // (23, 1) => day_23::star_1,
        // (23, 2) => day_23::star_2,
        // (24, 1) => day_24::star_1,
        // (24, 2) => day_24::star_2,
        // (25, 1) => day_25::star_1,
        // (25, 2) => day_25::star_2,
        _ => {
            println!("Invalid day and/or star.");
            return;
        }
    };

    func(data);
}
