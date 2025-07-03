use std::collections::VecDeque;

const MAX_SAMPLES: usize = 180; // 15 minutes * 60 seconds / 5 seconds per sample

pub struct ProcessData {
    pub cpu_usage_history: VecDeque<f32>,
    pub memory_usage_history: VecDeque<u64>,
}

impl ProcessData {
    pub fn new() -> Self {
        Self {
            cpu_usage_history: VecDeque::with_capacity(MAX_SAMPLES),
            memory_usage_history: VecDeque::with_capacity(MAX_SAMPLES),
        }
    }

    pub fn add_sample(&mut self, cpu_usage: f32, memory_usage: u64) {
        if self.cpu_usage_history.len() == MAX_SAMPLES {
            self.cpu_usage_history.pop_front();
        }
        self.cpu_usage_history.push_back(cpu_usage);

        if self.memory_usage_history.len() == MAX_SAMPLES {
            self.memory_usage_history.pop_front();
        }
        self.memory_usage_history.push_back(memory_usage);
    }

    pub fn cpu_average(&self) -> f32 {
        if self.cpu_usage_history.is_empty() {
            return 0.0;
        }
        self.cpu_usage_history.iter().sum::<f32>() / self.cpu_usage_history.len() as f32
    }

    pub fn memory_average(&self) -> u64 {
        if self.memory_usage_history.is_empty() {
            return 0;
        }
        self.memory_usage_history.iter().sum::<u64>() / self.memory_usage_history.len() as u64
    }
} 