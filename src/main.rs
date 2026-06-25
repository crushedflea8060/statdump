//gameplan: get ram amount, cpu info and core count, disk size on the biggest disk, get gpu info if possible, get architecture, get display info
static DARK_RED_ANSII: &'static str = "\x1b[38;5;9m";
static RED_ANSII: &'static str = "\x1b[38;5;9m";
static GREEN_ANSII: &'static str = "\x1b[38;5;10m";
static DEFAULT_ANSII: &'static str = "\x1b[38;5;7m";
static BLUE_ANSII: &'static str = "\x1b[38;5;12m";
static YELLOW_ANSII: &'static str = "\x1b[38;5;11m"; //future projects should probably use a const struct
fn main() {
    if !cfg!(target_os = "linux")
    {
        println!("{DARK_RED_ANSII}Critical, OS is not Linux.{DEFAULT_ANSII}");
        std::process::exit(1);
    }
    match get_hostname()
    {
        Some(host) => {
            println!("{host}:");
        }
        None => {
            println!("{RED_ANSII}Failed to retrieve Host Information.{DEFAULT_ANSII}");
        }
    }
    println!("");
    match get_cpu_information()
    {
        Some(info) => {
            println!("{}CPU Information{}", YELLOW_ANSII, DEFAULT_ANSII);
            println!("{}<------------------->{}", BLUE_ANSII, DEFAULT_ANSII);
            println!("{GREEN_ANSII}CPU Model: {}", info.0);
            println!("{GREEN_ANSII}CPU Cores: {}", info.1);
            println!("{GREEN_ANSII}CPU Frequency: {} GHz", (info.2 as f32).floor() / 1000.0);
            println!("{GREEN_ANSII}CPU Architecture: {}{DEFAULT_ANSII}", info.3);
            println!("{}<------------------->{}\n", BLUE_ANSII, DEFAULT_ANSII);
        },
        None => {
            println!("{}Failed to retrieve CPU information.{}", RED_ANSII, DEFAULT_ANSII);
        }
    }

    match get_ram_information()
    {
        Some(ram_size) => {
            println!("{}RAM Information{}", YELLOW_ANSII, DEFAULT_ANSII);
            println!("{}<------------------->{}", BLUE_ANSII, DEFAULT_ANSII);
            let ram_gb = ((ram_size as f64 / (1000_f64 * 1000_f64)) * 100_f64).floor() / 100_f64; //use 1000 instead of 1024 for cleaner numbers
            println!("{GREEN_ANSII}RAM Total Amount: {}GB{DEFAULT_ANSII}", ram_gb);
            println!("{}<------------------->{}\n", BLUE_ANSII, DEFAULT_ANSII);
        },
        None => {
            println!("{}Failed to retrieve RAM information.{}", RED_ANSII, DEFAULT_ANSII);
        }
    }

    match get_disk_info()
    {
        Some(disk_size) => {
            println!("{}Disk Information{}", YELLOW_ANSII, DEFAULT_ANSII);
            println!("{}<------------------->{}", BLUE_ANSII, DEFAULT_ANSII);
            let disk_gb = ((disk_size as f64 / (1000_f64 * 1000_f64)) * 100_f64).floor() / 100_f64; //use 1000 instead of 1024 for cleaner numbers
            println!("{GREEN_ANSII}Disk Total Amount: {}GB{DEFAULT_ANSII}", disk_gb);
            println!("{}<------------------->{}\n", BLUE_ANSII, DEFAULT_ANSII);
        }
        None => {
            println!("{RED_ANSII}Failed to retrieve Disk information.{DEFAULT_ANSII}");
        }
    }
    match get_gpu_info()
    {
        Some(gpu) => {
            println!("{}GPU Driver / Model{}", YELLOW_ANSII, DEFAULT_ANSII);
            println!("{}<------------------->{}", BLUE_ANSII, DEFAULT_ANSII);
            println!("{GREEN_ANSII}GPU{gpu}{DEFAULT_ANSII}");
            println!("{}<------------------->{}\n", BLUE_ANSII, DEFAULT_ANSII);
        }
        None => {
            println!("{RED_ANSII}Failed to retrieve GPU information. Try installing \"pciutils\"{DEFAULT_ANSII}");
        }
    }
}


struct CPU(String, u8, u64, String);

fn get_cpu_information() -> Option<CPU> {
    let content = std::fs::read_to_string("/proc/cpuinfo").ok()?;

    let model = content.lines().filter(|l| l.starts_with("model name"))
        .map(|l| l.split(':').nth(1).unwrap_or("").trim().to_string())
        .collect::<Vec<String>>()
        .join(", ")
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect::<Vec<String>>()
        .first()
        .cloned()?;
    let cores = content.lines().filter(|l| l.starts_with("processor")).count();
    let frequency_strings = content.lines().filter(|l| l.starts_with("cpu MHz")).map(|l|l.trim().to_string()).collect::<Vec<String>>();
    let mut frequency_nums = vec![];
    for frequency_string in frequency_strings {
    if let Some(parts) = frequency_string.split(":").nth(1) {
        if let Ok(freq) = parts.trim().parse::<f32>() {
            frequency_nums.push(freq.floor() as u64);        }
    }
    }
    let frequency_sum: u64 = frequency_nums.iter().sum();
    let frequency = frequency_sum / (cores as u64);

    let architecture = std::env::consts::ARCH.to_string();

    return Some(CPU(model, cores as u8, frequency, architecture));

}

fn get_ram_information() -> Option<u64> {
    let content = std::fs::read_to_string("/proc/meminfo").ok()?;
    return content.lines().filter(|l| l.starts_with("MemTotal")).next().unwrap_or("MemTotal: 1 kB").split(':').nth(1)?.trim().split_whitespace().nth(0)?.trim().parse::<u64>().ok() //filter the memtotal line, split the : then split spaces and parse
}

fn get_disk_info() -> Option<u64> { //in KB
    // read /proc/partitions and get the largest partition size (in KB)
    let content = std::fs::read_to_string("/proc/partitions").ok()?;
    let mut max_size: Option<u64> = None;
    for line in content.lines() {
        match line.trim().is_empty() || line.trim().starts_with("major") {
            true => continue,
            false => (),
        }
        let parts: Vec<&str> = line.trim().split_whitespace().collect();
        // expected format: major minor  #blocks  name
        if parts.len() < 3 {
            continue;
        }
        if let Ok(size) = parts[2].parse::<u64>() {
            max_size = Some(std::cmp::max(max_size.unwrap_or(0), size));
        }
    }
    max_size

}

fn get_gpu_info() -> Option<String> {
    if let Ok(lspci) = std::process::Command::new("lspci").output() {
        if lspci.status.success() {
            let specs = String::from_utf8_lossy(&lspci.stdout);
            let mut result: String = "".to_string();
            for line in specs.lines() {
                if line.to_lowercase().contains("vga") || line.to_lowercase().contains("3d") {
                    if let Some(text) = line.find("controller:") {
                        result = line[text + 11..].trim().to_string()
                    } else {
                        result = line.to_string();
                    }
                }
            }
            Some(result)
        } else {
            None
        }
    } else {
        None
    }
}

fn get_hostname() -> Option<String> {
    let hostname = std::fs::read_to_string("/etc/hostname").ok()?;
    let host_cmd = std::process::Command::new("whoami").output().ok()?;
    let host = String::from_utf8_lossy(&host_cmd.stdout);

    Some(format!("{}@{}", host.trim(), hostname.trim()))



}

