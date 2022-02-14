use crate::types::*;

pub fn get_class_width_column(fix_width: WidthColumn, width: u8) -> String {
    match fix_width {
        WidthColumn::Relative => {
            class_width_relative(width)
        },
        WidthColumn::EqualPx => {
            class_width_equalpx(width)
        }
    }

}

pub fn class_width_equalpx(number: u8) -> String {
    match number {
        0 => format!("w-0"),
        1 => format!("w-1"),
        2 => format!("w-2"),
        3 => format!("w-3"),
        4 => format!("w-4"),
        5 => format!("w-5"),
        6 => format!("w-6"),
        7 => format!("w-7"),
        8 => format!("w-8"),
        9 => format!("w-9"),
        10 => format!("w-10"),
        11 => format!("w-11"),
        12 => format!("w-12"),
        13..=14 => format!("w-14"),
        15..=16 => format!("w-16"),
        17..=20 => format!("w-20"),
        21..=24 => format!("w-24"),
        25..=28 => format!("w-28"),
        29..=32 => format!("w-32"),
        33..=36 => format!("w-36"),
        37..=40 => format!("w-40"),
        41..=44 => format!("w-44"),
        45..=48 => format!("w-48"),
        49..=52 => format!("w-52"),
        53..=56 => format!("w-56"),
        57..=60 => format!("w-60"),
        61..=64 => format!("w-64"),
        65..=72 => format!("w-72"),
        73..=80 => format!("w-80"),
        81..=99 => format!("w-96"),
        _ => format!("w-auto"),
    }
}

pub fn class_width_relative(number: u8) -> String {
    match number {
        0 => format!("w-0"),
        //width: 8.333333%;
        1..=8 => format!("w-1/12"),
        // width: 16.666667%;
        9..=16 => format!("w-2/12"),
        // width: 20%;
        17..=20 => format!("w-1/5"),
        // width: 33.333333%;
        21..=39 => format!("w-1/3"),
        // width: 40%;
        40 => format!("w-2/5"),
        // width: 41.666667%;
        41 => format!("w-5/12"),
        // width: 50%;
        42..=50 => format!("w-6/12"),
        // width: 58.333333%;
        51..=58 => format!("w-7/12"),
        // width: 60%;
        59..=60 => format!("w-3/5"),
        // width: 66.666667%;
        61..=66 => format!("w-4/6"),
        // width: 75%;
        67..=75 => format!("w-9/12"),
        // width: 80%;
        76..=80 => format!("w-4/5"),
        // width: 83.333333%;
        81..=83 => format!("w-10/12"),
        // width: 91.666667%;
        84..=99 => format!("w-11/12"),
        // width: 100%;
        _ => format!("w-full"),
    }
}