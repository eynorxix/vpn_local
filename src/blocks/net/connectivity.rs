use std::process::Command;
use std::time::{Duration, Instant};

pub fn has_internet() -> bool {
    if let Ok(out) = Command::new("ping")
        .args(["-c", "1", "-W", "2", "1.1.1.1"])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
    {
        if out.status.success() {
            return true;
        }
    }
    if let Ok(out) = Command::new("curl")
        .args([
            "-s",
            "--connect-timeout",
            "2",
            "--max-time",
            "3",
            "-o",
            "/dev/null",
            "https://1.1.1.1",
        ])
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .output()
    {
        if out.status.success() {
            return true;
        }
    }
    false
}

pub fn wait_for_internet(timeout_secs: u64) -> bool {
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(timeout_secs) {
        if has_internet() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(500));
    }
    false
}
