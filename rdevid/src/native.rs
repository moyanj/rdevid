use sysinfo::{Components, Disks, System};

pub fn get_device_info() -> String {
    let mut sys = System::new_all();
    sys.refresh_all();

    let mut info = String::new();

    // 操作系统信息
    info.push_str(&format!("OS:{};", System::name().unwrap_or_default()));
    info.push_str(&format!(
        "OSVer:{};",
        System::os_version().unwrap_or_default()
    ));
    info.push_str(&format!(
        "Kernel:{};",
        System::kernel_version().unwrap_or_default()
    ));
    info.push_str(&format!(
        "Host:{};",
        System::host_name().unwrap_or_default()
    ));
    info.push_str(&format!("Arch:{};", std::env::consts::ARCH));

    // CPU 信息
    if let Some(cpu) = sys.cpus().first() {
        info.push_str(&format!("CPU:{};", cpu.brand()));
    }
    info.push_str(&format!("Cores:{};", sys.cpus().len()));

    // 内存信息
    info.push_str(&format!("Mem:{};", sys.total_memory()));

    // 磁盘信息
    for disk in &Disks::new_with_refreshed_list() {
        info.push_str(&format!(
            "Disk:{}-{};",
            disk.name().to_str().unwrap_or("unknown"),
            disk.total_space()
        ));
    }

    // PCI 设备信息
    let mut pci_devices = Components::new_with_refreshed_list();
    pci_devices.sort_by(|a, b| a.label().cmp(&b.label())); // 按设备名称排序
    for device in pci_devices.iter() {
        info.push_str(&format!("PCI:{};", device.label()));
    }

    info
}
