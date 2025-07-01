# Galactic Tamer - Development Plan

## Overview

Build a Windows 11+ x64 app in Rust that monitors running processes and system services, alerting when any exceed their average CPU or RAM usage over the past 15 minutes. The goal: keep your system harmonious and free from resource-hungry invaders.

## Core Features

- Real-time monitoring of all processes and system services.
- Track CPU and RAM usage per process/service.
- Calculate rolling 15-minute averages for each.
- Alert (notification, log, or popup) when usage exceeds average.
- Simple, user-friendly interface (CLI or minimal GUI).

## Steps

1. **Environment Setup**

   - Set up Rust toolchain for Windows x64.
   - Choose libraries for process monitoring (e.g., `sysinfo`, `windows-rs`).

2. **Process & Service Monitoring**

   - Enumerate all running processes and system services.
   - Collect CPU and RAM usage at regular intervals (e.g., every 5 seconds).

3. **Data Storage**

   - Store usage data in memory (ring buffer or similar) for each process/service for the last 15 minutes.

4. **Average Calculation**

   - Calculate rolling averages for CPU and RAM per process/service.

5. **Alert System**

   - Compare current usage to 15-minute average.
   - Trigger alert if usage exceeds average by configurable threshold.
   - Implement notification (system tray, popup, or log).

6. **User Interface**

   - Start with CLI for MVP.
   - Optionally, add a minimal GUI (e.g., with `egui` or `native-windows-gui`).

7. **Configuration**

   - Allow user to set thresholds and monitoring intervals.
   - Optionally, allow process/service exclusion.

8. **Testing**

   - Test on Windows 11 x64.
   - Simulate high resource usage for validation.

9. **Packaging**
   - Build release binaries.
   - Write README and usage instructions.

## Stretch Goals

- Historical logging and visualization.
- Remote monitoring.
- Auto-throttling or process control.

---

_Remember: Don’t panic. And always bring a towel (or at least a backup of your registry)._ 🚀
