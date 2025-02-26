use crossterm::style::Stylize;

pub fn log_err(err: &anyhow::Error) {
    let err_msg = format!("✘  {}", err).red().bold();
    println!("{}", err_msg);
}
