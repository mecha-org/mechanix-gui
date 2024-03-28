use sysinfo::{Disks, MacAddr, Networks, System};

pub struct HostMetrics {}

pub struct MemoryInfo {
    pub total_memory: u64,
    pub used_memory: u64,
    pub total_swap: u64,
    pub used_swap: u64,
}

pub struct DiskInfo {
    pub available_space: u64,
    pub total_space: u64,
    pub used_space: u64,
}

pub struct LoadAverage {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

pub struct NetworkDataInfo {
    pub interface_name: String,
    pub mac_address: MacAddr,
    pub received: u64,
    pub transmitted: u64,
}

impl HostMetrics {
    pub fn new() -> HostMetrics {
        HostMetrics {}
    }

    pub fn cpu_usage(&self) -> f32 {
        let mut sys = System::new();
        sys.refresh_cpu();
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        cpu_usage
    }

    pub fn cpu_freq(&self) -> u64 {
        let mut sys = System::new();
        sys.refresh_cpu();
        let cpu_freq = sys.global_cpu_info().frequency();
        cpu_freq
    }

    pub fn memory_usage(&self) -> u64 {
        let mut sys = System::new();
        sys.refresh_memory();
        let memory_usage = sys.used_memory();
        memory_usage
    }

    pub fn swap_usage(&self) -> u64 {
        let mut sys = System::new();
        sys.refresh_memory();
        let swap_usage = sys.used_swap();
        swap_usage
    }

    pub fn disk_info(&self) -> DiskInfo {
        let sys = Disks::new_with_refreshed_list();
        let disk_info = sys.iter().next().unwrap();

        DiskInfo {
            available_space: disk_info.available_space(),
            total_space: disk_info.total_space(),
            used_space: disk_info.total_space() - disk_info.available_space(),
        }
    }

    pub fn memory_info(&self) -> MemoryInfo {
        let mut sys = System::new();
        sys.refresh_memory();
        let total_memory = sys.total_memory();
        let used_memory = sys.used_memory();
        let total_swap = sys.total_swap();
        let used_swap = sys.used_swap();

        MemoryInfo {
            total_memory,
            used_memory,
            total_swap,
            used_swap,
        }
    }

    pub fn swap_info(&self) -> u64 {
        let mut sys = System::new();
        sys.refresh_memory();
        let swap_info = sys.total_swap();
        swap_info
    }

    pub fn uptime(&self) -> u64 {
        let uptime = System::uptime();
        uptime
    }

    //load average
    pub fn load_average(&self) -> LoadAverage {
        let sys = System::load_average();

        LoadAverage {
            one: sys.one,
            five: sys.five,
            fifteen: sys.fifteen,
        }
    }

    //network usage
    pub fn network_usage(&self) -> Vec<NetworkDataInfo> {
        let mut networks = Networks::new_with_refreshed_list();
        networks.refresh();
        let mut collected_data = Vec::new();
        for (interface_name, data) in networks.iter() {
            let network_data = NetworkDataInfo {
                interface_name: interface_name.clone(),
                mac_address: data.mac_address(),
                received: data.received(),
                transmitted: data.transmitted(),
            };
            collected_data.push(network_data);
        }
        collected_data
    }
}
