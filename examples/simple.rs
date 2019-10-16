extern crate file_io;

use file_io::log_parser;

fn main() {
  let file_path = String::from("simple-data.json");
  
  let mut stats = log_parser::group_totals(file_path).unwrap();

  for stat in stats.application_stats() {
    println!("{}: {}", stat.name, stat.total_time);
  }
}