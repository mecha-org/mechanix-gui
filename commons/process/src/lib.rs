use sysinfo::{System, SystemExt};

pub fn is_app_already_running(process_name: &str) -> bool {
    let system = System::new_all();
    system
        .processes_by_exact_name(process_name)
        .into_iter()
        .count()
        >= 2
}
