mod app;
mod process_data;
mod tui;

use crate::app::App;
use crate::process_data::ProcessData;
use clap::Parser;
use crossterm::event::{self, Event};
use std::collections::HashMap;
use std::error::Error;
use std::thread;
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

    /// Minimum CPU usage (%) to trigger an alert
    #[arg(long, default_value_t = 10.0)]
    min_cpu_alert: f32,

    /// Minimum memory usage (in MB) to trigger an alert
    #[arg(long, default_value_t = 100)]
    min_mem_alert: u64,

    /// Monitoring interval in seconds
    #[arg(short, long, default_value_t = 5)]
    interval: u64,
}

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::parse();
    let mut sys = System::new_all();
    let mut process_map: HashMap<Pid, ProcessData> = HashMap::new();
    let mut app = App::new();

    // Prime the CPU usage measurements
    sys.refresh_processes();
    thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_processes();

    let mut terminal = tui::init()?;
    let tick_rate = Duration::from_secs(config.interval);
    let mut last_tick = Instant::now();

    loop {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        let mut needs_redraw = false;

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if tui::handle_key_event(key, &mut app)? {
                    break;
                }
                needs_redraw = true;
            }
        }

        if last_tick.elapsed() >= tick_rate {
            sys.refresh_processes();
            app.processes = sys.processes().keys().cloned().collect();
            app.sort_processes(&sys);

            let mut new_alerts: Vec<String> = Vec::new();
            for pid in &app.processes {
                if let Some(process) = sys.process(*pid) {
                    let data = process_map.entry(*pid).or_insert_with(ProcessData::new);
                    data.add_sample(process.cpu_usage(), process.memory());

                    let current_cpu = process.cpu_usage();
                    let cpu_avg = data.cpu_average();
                    if current_cpu > config.min_cpu_alert
                        && current_cpu > cpu_avg * config.cpu_threshold
                    {
                        new_alerts.push(format!(
                            "High CPU usage for {} ({}): {:.2}%",
                            process.name(),
                            process.pid(),
                            current_cpu
                        ));
                    }

                    let current_mem = process.memory();
                    let mem_avg = data.memory_average();
                    let min_mem_bytes = config.min_mem_alert * 1024 * 1024;
                    if current_mem > min_mem_bytes
                        && current_mem > (mem_avg as f32 * config.mem_threshold) as u64
                    {
                        new_alerts.push(format!(
                            "High Memory usage for {} ({}): {}",
                            process.name(),
                            process.pid(),
                            humansize::format_size(current_mem, humansize::BINARY)
                        ));
                    }
                }
            }
            for alert in new_alerts {
                app.add_alert(alert);
            }
            last_tick = Instant::now();
            needs_redraw = true;
        }

        if needs_redraw {
            terminal.draw(|frame| {
                tui::draw_ui(frame, &mut app, &sys, &process_map);
            })?;
        }
    }

    tui::restore()?;
    Ok(())
}
