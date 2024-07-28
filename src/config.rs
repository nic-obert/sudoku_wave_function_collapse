

pub const DIGITS_IN_ROW_PER_BOX: usize = 3;
pub const DIGITS_IN_COLUMN_PER_BOX: usize = DIGITS_IN_ROW_PER_BOX;
pub const BOXES_PER_ROW: usize = DIGITS_IN_ROW_PER_BOX;
pub const DIGITS_IN_ROW_PER_CELL: usize = BOXES_PER_ROW;

pub const CERTAIN_DIGIT_ROW_IN_BOX: usize = DIGITS_IN_ROW_PER_BOX / 2;

pub const DIGIT_BASE: usize = DIGITS_IN_ROW_PER_BOX * BOXES_PER_ROW;

pub const ROW_COUNT: usize = DIGIT_BASE;
pub const COLUMN_COUNT: usize = ROW_COUNT;

pub const CELL_COUNT: usize = DIGIT_BASE * DIGIT_BASE;

pub const DEFAULT_HINT_COUNT: u8 = 30;

