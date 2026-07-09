use os_info;

// Get the system information, such as the OS type, Version, and Architecture such as x86 or x86_64.

// https://github.com/stanislav-tkach/os_info

pub fn get_system_info() {
    let info = os_info::get();

    // Print full information:
    println!("OS information: {info}");

    // Print information separately:
    println!("Type: {}", info.os_type());
    println!("Version: {}", info.version());
    println!("Bitness: {}", info.bitness());
    println!("Architecture: {}", info.architecture().unwrap_or("Unknown"));
}