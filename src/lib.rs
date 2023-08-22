pub const COLOR_RESET: &str = "\x1b[0m";
pub const COLOR_RED: &str = "\x1b[31m";
pub const COLOR_GREEN: &str = "\x1b[32m";
pub const COLOR_YELLOW: &str = "\x1b[33m";
pub const COLOR_BLUE: &str = "\x1b[34m";
pub const COLOR_MAGENTA: &str = "\x1b[35m";
pub const COLOR_CYAN: &str = "\x1b[36m";

pub const COLOR_BOLD: &str = "\x1b[1m";
pub const COLOR_BOLD_RED: &str = "\x1b[1;31m";
pub const COLOR_BOLD_GREEN: &str = "\x1b[1;32m";
pub const COLOR_BOLD_YELLOW: &str = "\x1b[1;33m";
pub const COLOR_BOLD_BLUE: &str = "\x1b[1;34m";
pub const COLOR_BOLD_MAGENTA: &str = "\x1b[1;35m";
pub const COLOR_BOLD_CYAN: &str = "\x1b[1;36m";

pub const PREFIX_DEBUG: &str = "\x1b[1;34m[DEBUG]\x1b[0m";
pub const PREFIX_INFO: &str = "\x1b[1;32m[INFO]\x1b[0m";
pub const PREFIX_WARN: &str = "\x1b[1;33m[WARN]\x1b[0m";
pub const PREFIX_ERROR: &str = "\x1b[1;31m[ERROR]\x1b[0m";

pub fn log(prefix: &str, message: &str) {
    println!("{} {}", prefix, message);
}