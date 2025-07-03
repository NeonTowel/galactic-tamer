mod app;
mod process_data;
mod tui;

use crate::app::App;
use crate::process_data::ProcessData;
use clap::Parser;
use std::collections::HashMap;
use std::error::Error;
use std::time::{Duration, Instant};
use sysinfo::{Pid, System};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Config {
    /// CPU usage threshold multiplier (e.g., 1.2 for 120%)
    #[arg(short, long, default_value_t = 1.2)]
    cpu_threshold: f32,

    /// Memory usage threshold multiplier (e.g., 1.2 for 120%)
    #[arg(short, long, default_value_t = 1.2)]
    mem_threshold: f32,

    /// Monitoring interval in seconds
    #[arg(short, long, default_value_t = 1)]
    interval: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
    let mut sys = System::new_all();
    let mut process_map: HashMap<Pid, ProcessData> = HashMap::new();
    let mut app = App::new();

    let mut terminal = tui::init()?;
    let tick_rate = Duration::from_secs(config.interval);
    let mut last_tick = Instant::now();

    loop {
        if tui::handle_events(&mut app)? {
            break;
        }

        if last_tick.elapsed() >= tick_rate {
            sys.refresh_all();
            app.processes = sys.processes().keys().cloned().collect();
            app.sort_processes(&sys);

            let mut new_alerts: Vec<String> = Vec::new();
            for pid in &app.processes {
                if let Some(process) = sys.process(*pid) {
                    let data = process_map.entry(*pid).or_insert_with(ProcessData::new);
                    data.add_sample(process.cpu_usage(), process.memory());

                    let cpu_avg = data.cpu_average();
                    if cpu_avg > 0.0 && process.cpu_usage() > cpu_avg * config.cpu_threshold {
                        new_alerts.push(format!(
                            "High CPU usage for {} ({}): {:.2}%",
                            process.name(),
                            process.pid(),
                            process.cpu_usage()
                        ));
                    }

                    let mem_avg = data.memory_average();
                    if mem_avg > 0
                        && process.memory() > (mem_avg as f32 * config.mem_threshold) as u64
                    {
                        new_alerts.push(format!(
                            "High Memory usage for {} ({}): {}",
                            process.name(),
                            process.pid(),
                            humansize::format_size(process.memory(), humansize::BINARY)
                        ));
                    }
                }
            }
            for alert in new_alerts {
                app.add_alert(alert);
            }
            last_tick = Instant::now();
        }

        terminal.draw(|frame| {
            tui::draw_ui(frame, &mut app, &sys, &process_map);
        })?;
    }

    tui::restore()?;
    Ok(())
}
