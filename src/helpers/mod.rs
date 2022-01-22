use crate::types::*;

pub fn get_class_width_column(fix_width: WidthColumn, width: u8) -> String {
    match fix_width {
        WidthColumn::Relative => {
            let width_max = get_world_number(width);
            get_to_class_width_column(width_max, width)
        },
        WidthColumn::Equal => {
            format!("w-column-{}", width)
        }
    }

}

pub fn get_to_class_width_column(width_max: String, number: u8) -> String {
    match number {
        1 => format!("w-column-{width_max}"),
        2 => format!("w-column-{width_max}"),
        3 => format!("w-column-{width_max}"),
        4 => format!("w-column-{width_max}"),
        5 => format!("w-column-{width_max}"),
        6 => format!("w-column-{width_max}"),
        7 => format!("w-column-{width_max}"),
        8 => format!("w-column-{width_max}"),
        9 => format!("w-column-{width_max}"),
        10 => format!("w-column-{width_max}"),
        11 => format!("w-column-{width_max}"),
        12 => format!("w-column-{width_max}"),
        13 => format!("w-column-{width_max}"),
        14 => format!("w-column-{width_max}"),
        15 => format!("w-column-{width_max}"),
        16 => format!("w-column-{width_max}"),
        _ => "w-1/3".to_owned(),
    }
}

pub fn get_world_number(width: u8) -> String {
    match width {
        1 => "one".to_owned(),
        2 => "two".to_owned(),
        3 => "three".to_owned(),
        4 => "four".to_owned(),
        5 => "five".to_owned(),
        6 => "six".to_owned(),
        7 => "seven".to_owned(),
        8 => "eight".to_owned(),
        9 => "nine".to_owned(),
        10 => "ten".to_owned(),
        11 => "eleven".to_owned(),
        12 => "twelve".to_owned(),
        13 => "thirteen".to_owned(),
        14 => "fourteen".to_owned(),
        15 => "fifteen".to_owned(),
        16 => "sixteen".to_owned(),
        _ => "".to_owned(),
    }
}