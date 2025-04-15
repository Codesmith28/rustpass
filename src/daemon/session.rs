use log::{ info, error, debug };
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Clone, Debug)]
pub enum SessionStatus {
    Active,
    Inactive,
}

pub fn get_session_status() -> SessionStatus {
    // This is a simplified approach - in production, you'd want to use
    // platform-specific APIs or libraries for better session detection

    #[cfg(target_os = "linux")]
    {
        // On Linux, check for X session or active systemd user session
        let output = Command::new("loginctl").arg("show-user").arg("--property=State").output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("active") {
                    return SessionStatus::Active;
                }
            }
            Err(e) => {
                error!("Failed to check session status: {}", e);
            }
        }
    }

    #[cfg(target_os = "macos")]
    {
        // On macOS, check if screen is unlocked
        let output = Command::new("bash")
            .arg("-c")
            .arg("ioreg -n Root -d1 -a | grep 'IOConsoleUsers' | grep 'CGSSessionScreenIsLocked'")
            .output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if !stdout.contains("true") {
                    return SessionStatus::Active;
                }
            }
            Err(e) => {
                error!("Failed to check session status: {}", e);
            }
        }
    }

    #[cfg(target_os = "windows")]
    {
        // On Windows, check if a session is active
        // This is simplified; in production you'd use the Windows API
        let output = Command::new("cmd").arg("/c").arg("query session").output();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("Active") {
                    return SessionStatus::Active;
                }
            }
            Err(e) => {
                error!("Failed to check session status: {}", e);
            }
        }
    }

    SessionStatus::Inactive
}

pub fn monitor_session_changes<F>(callback: F) where F: Fn(SessionStatus) + Send + 'static {
    thread::spawn(move || {
        let mut last_status = get_session_status();
        info!("Initial session status: {:?}", last_status);
        callback(last_status.clone());

        loop {
            thread::sleep(Duration::from_secs(10));
            let current_status = get_session_status();

            if std::mem::discriminant(&current_status) != std::mem::discriminant(&last_status) {
                info!("Session status changed from {:?} to {:?}", last_status, current_status);
                callback(current_status.clone());
                last_status = current_status;
            } else {
                debug!("Session status check: still {:?}", current_status);
            }
        }
    });
}
