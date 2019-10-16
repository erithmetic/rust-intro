extern crate file_io;

use file_io::log_parser;
use std::env;

fn main() -> Result<(), std::io::Error> {
  let mut args: Vec<String> = env::args().collect();
  args.remove(0);

  for json_path in args {
    println!("Opening {}", json_path);

    let mut stats = log_parser::group_totals(json_path)?;

    for stat in stats.application_stats() {
      println!("{}: {}", stat.name, stat.total_time);
    }
  }

  Ok(())
}