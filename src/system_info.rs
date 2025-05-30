use sysinfo::{System, SystemExt, ProcessExt};

/// Get OS information (name and version)
pub fn get_os_info() -> (String, String) {
    let system = System::new_with_specifics(
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
    );

    let os_name = system.name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = system.os_version().unwrap_or_else(|| "Unknown".to_string());
    (os_name, os_version)
}

/// Get current shell information
pub fn get_shell_info() -> String {
    let system = System::new_with_specifics(
        sysinfo::RefreshKind::new().with_processes(sysinfo::ProcessRefreshKind::new()),
    );

    sysinfo::get_current_pid()
        .ok()
        .and_then(|pid| system.process(pid))
        .and_then(|proc| proc.parent())
        .and_then(|parent_pid| system.process(parent_pid))
        .map(|proc| proc.name().to_string())
        .unwrap_or_else(|| "Unknown shell".to_string())
}