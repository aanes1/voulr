use utils::log_err;

mod cli;
mod create;
mod utils;

fn main() {
    if let Err(e) = cli::run() {
        log_err(&e);
    }
}
