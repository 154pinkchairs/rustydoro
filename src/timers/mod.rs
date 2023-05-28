pub const WORK_DURATIONS: [u8; 9] = [15, 20, 25, 30, 35, 40, 45, 50, 60];
pub const BREAK_DURATIONS: [u8; 7] = [3, 5, 7, 10, 15, 20, 30];
pub const LONG_BREAK_DURATIONS: [u8; 4] = [15, 20, 30, 45];
pub const LONG_BREAK_INTERVALS: [u8; 4] = [6, 5, 4, 3];
// ((60*60) + (45*60)) * 6 + (30*60) = 19800
// u16 max value is 65535
