use std::{io};
use std::env;
use hidapi::{HidDevice, HidApi, HidError};

fn find_keyboard(pid: u16, vid: u16, usage: u16, usage_page: u16) -> Result<HidDevice, HidError> {
    return match HidApi::new() {
        Ok(api) => {
            for device in api.device_list() {
                if device.vendor_id() == vid && device.product_id() == pid && device.usage() == usage && device.usage_page() == usage_page {
                    return api.open_path(device.path());
                }
            }
            return Err(HidError::HidApiError { message: "No device found".parse().unwrap() })
        },
        Err(e) => Err(e),
    };
}

fn send_string_to_device(device: &HidDevice, string: &str) -> Result<(), String> {
    let trimmed = string.trim();
    if trimmed.len() <= 0 {
        return Ok(());
    }
    let split = trimmed.split(' ');
    let mut data_to_send = vec![0x00]; //
    for v in split {
        let data: u8 = match v.parse::<u8>() {
            Ok(v) => v,
            Err(e) => return Err(e.to_string()),
        };
        data_to_send.push(data);
    }
    match device.write(&data_to_send[..]) {
        Ok(_) => {},
        Err(e) => return Err(e.to_string()),
    }
    Ok(())
}

static mut SHOULD_EXIT: bool = false;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 5 {
        println!("Error: please specify all args {}/5", args.len());
        return;
    }

    let pid = match args[1].parse::<u16>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: Invalid pid {}\n{}", args[1], e);
            0
        },
    };
    let vid = match args[2].parse::<u16>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: Invalid vid {}\n{}", args[2], e);
            0
        },
    };
    let usage = match args[3].parse::<u16>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: Invalid usage {}\n{}", args[3], e);
            0
        },
    };
    let usage_page = match args[4].parse::<u16>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: Invalid usage page {}\n{}", args[4], e);
            0
        },
    };

    if pid == 0 || vid == 0 || usage == 0 || usage_page == 0 {
        return;
    }

    ctrlc::set_handler(move || unsafe {
        print!("Press enter");
        SHOULD_EXIT = true;
    }).expect("Failed setting sigint handler");

    let keyboard: HidDevice = match find_keyboard(pid, vid, usage, usage_page) {
        Ok(device) => device,
        Err(e) => {
            println!("Error: Opening Device: {}", e);
            return;
        }
    };

    println!("ok {} {} {} {}", pid, vid, usage, usage_page);
    loop {
        unsafe {
            if SHOULD_EXIT {
                return;
            }
        }
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_n) => {
                if input.starts_with("stop") {
                    println!("Exiting!");
                    return;
                }
                match send_string_to_device(&keyboard, &*input) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("Error: {}", e);
                    }
                }
            }
            Err(error) => println!("Error: {}", error),
        }
    }
}

