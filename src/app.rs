use std::collections::VecDeque;
use sysinfo::{Pid, System};

#[derive(PartialEq, Eq)]
pub enum SortBy {
    Pid,
    Name,
    Cpu,
    Memory,
}

pub enum SortOrder {
    Asc,
    Desc,
}

pub struct App {
    pub processes: Vec<Pid>,
    pub alerts: VecDeque<String>,
    pub sort_by: SortBy,
    pub sort_order: SortOrder,
}

impl App {
    pub fn new() -> Self {
        Self {
            processes: Vec::new(),
            alerts: VecDeque::with_capacity(3),
            sort_by: SortBy::Cpu,
            sort_order: SortOrder::Desc,
        }
    }

    pub fn add_alert(&mut self, alert: String) {
        if self.alerts.len() == 3 {
            self.alerts.pop_back();
        }
        self.alerts.push_front(alert);
    }

    pub fn sort_processes(&mut self, sys: &System) {
        self.processes.sort_by(|a, b| {
            let proc_a = sys.process(*a).unwrap();
            let proc_b = sys.process(*b).unwrap();
            let ordering = match self.sort_by {
                SortBy::Pid => proc_a.pid().cmp(&proc_b.pid()),
                SortBy::Name => proc_a.name().cmp(&proc_b.name()),
                SortBy::Cpu => proc_a.cpu_usage().partial_cmp(&proc_b.cpu_usage()).unwrap(),
                SortBy::Memory => proc_a.memory().cmp(&proc_b.memory()),
            };

            match self.sort_order {
                SortOrder::Asc => ordering,
                SortOrder::Desc => ordering.reverse(),
            }
        });
    }

    pub fn set_sort_by(&mut self, sort_by: SortBy) {
        if self.sort_by == sort_by {
            self.sort_order = match self.sort_order {
                SortOrder::Asc => SortOrder::Desc,
                SortOrder::Desc => SortOrder::Asc,
            };
        } else {
            self.sort_by = sort_by;
            self.sort_order = SortOrder::Desc;
        }
    }
}
