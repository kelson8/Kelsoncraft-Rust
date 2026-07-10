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

// Basic function to check what OS a user is on.
pub fn get_current_os() {
    let info = os_info::get();

    match info.os_type() {
        os_info::Type::Windows => {
            // println!("Windows OS version: {}", info.version());
            // This gets the current edition of windows, such as Windows 11 Home or Pro.
            println!("Windows OS version: {}", info.edition().unwrap_or("Unknown"));
        }

        os_info::Type::Linux => {
            println!("Linux OS version: {}", info.version());
        }

        os_info::Type::Macos => {
            println!("Mac OS version: {}", info.version());
        }

        _ => {
            println!("OS information is not supported");
        }
    }
}