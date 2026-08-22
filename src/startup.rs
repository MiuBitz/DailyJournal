use std::env;
use std::fs;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::process::Command;

const CREATE_NO_WINDOW: u32 = 0x08000000;

pub fn startup_shortcut_path() -> Option<PathBuf> {
    env::var_os("APPDATA").map(|appdata| {
        PathBuf::from(appdata)
            .join("Microsoft")
            .join("Windows")
            .join("Start Menu")
            .join("Programs")
            .join("Startup")
            .join("DailyJournal.lnk")
    })
}

pub fn current_exe_path() -> Option<PathBuf> {
    env::current_exe().ok()
}

pub fn set_startup(enabled: bool) -> Result<(), String> {
    let shortcut = match startup_shortcut_path() {
        Some(s) => s,
        None => return Err("Could not locate Windows Startup directory".to_string()),
    };

    if enabled {
        let exe = match current_exe_path() {
            Some(e) => e,
            None => return Err("Could not locate current executable path".to_string()),
        };

        if let Some(parent) = shortcut.parent() {
            let _ = fs::create_dir_all(parent);
        }

        let script = format!(
            "$s = (New-Object -ComObject WScript.Shell).CreateShortcut('{}'); $s.TargetPath = '{}'; $s.Arguments = '--autostart'; $s.WorkingDirectory = '{}'; $s.Save()",
            shortcut.to_string_lossy().replace('\'', "''"),
            exe.to_string_lossy().replace('\'', "''"),
            exe.parent().unwrap_or(&exe).to_string_lossy().replace('\'', "''")
        );

        let status = Command::new("powershell")
            .creation_flags(CREATE_NO_WINDOW)
            .arg("-NoProfile")
            .arg("-NonInteractive")
            .arg("-Command")
            .arg(&script)
            .status()
            .map_err(|e| format!("Failed to run PowerShell startup command: {}", e))?;

        if !status.success() {
            return Err("PowerShell failed to create startup shortcut".to_string());
        }
    } else {
        if shortcut.exists() {
            if let Err(e) = fs::remove_file(&shortcut) {
                return Err(format!("Failed to remove startup shortcut: {}", e));
            }
        }
    }
    Ok(())
}
