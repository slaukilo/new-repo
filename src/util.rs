use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

mut sys = System::new_all();
sys.refresh_all();

// Display disk information
fn disks() {
    for disk in sys.disks() {
        println!("{:?", disks)
    }
}

// Display system information
fn systemInfo() {
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());
}