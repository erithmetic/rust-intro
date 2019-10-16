use serde::{Deserialize};
use std::collections::HashMap;
use std::default::Default;
use std::fs::File;
use std::io::{self, BufReader};
use std::io::prelude::*;


#[derive(Deserialize)]
struct LogLine {
    app: String,
    cpu_time: Option<i64>,
    io_wait: Option<i64>,
}

#[derive(Default, Clone)]
pub struct ApplicationStat {
    pub name: String,
    pub total_time: i64,
    pub cpu_time: i64,
    pub io_wait: i64,
}
impl ApplicationStat {
    /// Update CPU and IO times
    /// 
    /// ```
    /// use file_io::log_parser::{ApplicationStat};
    /// 
    /// let mut stat = ApplicationStat{
    ///   name: String::from("foo"),
    ///   cpu_time: 16,
    ///   io_wait: 78,
    ///   total_time: 94
    /// };
    /// stat.update(22,82);
    /// ```
    /// 
    pub fn update(&mut self, cpu_time: i64, io_wait: i64) {
        self.cpu_time += cpu_time;
        self.io_wait += io_wait;
        self.total_time = self.cpu_time + self.io_wait;
    }
}


pub struct Statistics {
    applicatons: HashMap<String, ApplicationStat>,
}
impl Statistics {
    fn new() -> Statistics {
        Statistics {
          applicatons: HashMap::new()
        }
    }

    fn add(&mut self, name: &String, cpu_time: i64, io_wait: i64) {
        let mut stat: ApplicationStat = self.applicatons.entry(name.clone()).or_insert(Default::default()).clone();
                // entry().or_insert() returns a reference, so we have to deref, which requires Copy
        stat.name = name.clone();
        stat.update(cpu_time, io_wait);
        self.applicatons.insert(name.clone(), stat);
    }

    pub fn application_stats(&mut self) -> std::collections::hash_map::Values<String, ApplicationStat> {
        self.applicatons.values()
    }
}

/// read JSON logs and group by source, adding up the total times
/// 
pub fn group_totals(json_path: String) -> io::Result<Statistics> {
    let jsons = File::open(json_path)?;
    let f = BufReader::new(jsons);
    let mut stats = Statistics::new();

    for line in f.lines() {
        let raw: String = line?;
        let log_line: LogLine = serde_json::from_str(raw.as_ref())?;
        stats.add(&log_line.app, log_line.cpu_time.unwrap_or_default(), log_line.io_wait.unwrap_or_default());
    }

    Ok(stats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_application_stat_update_adds_times() {
        let mut stat: ApplicationStat = Default::default();
        assert_eq!(stat.cpu_time, 0);
        assert_eq!(stat.io_wait, 0);
        assert_eq!(stat.total_time, 0);

        stat.update(1, 2);
        assert_eq!(stat.cpu_time, 1);
        assert_eq!(stat.io_wait, 2);
        assert_eq!(stat.total_time, 3);

        stat.update(10, 11);
        assert_eq!(stat.cpu_time, 11);
        assert_eq!(stat.io_wait, 13);
        assert_eq!(stat.total_time, 24);
    }
}