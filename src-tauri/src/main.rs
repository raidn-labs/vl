// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Conditionally compile `rppal` dependency only when `hardware-support` feature is enabled
#[cfg(feature = "hardware-support")]
use rppal::gpio::Gpio;
#[cfg(feature = "hardware-support")]
use rppal::uart::{Uart, Parity};

use serialport::{self, SerialPortType};

use std::io::{self, Read}; // Import the Read trait
use std::fs::File;
#[tauri::command]
fn get_temperature() -> Result<f32, String> {
    #[cfg(feature = "hardware-support")]
    {
        // Path to the temperature sensor's device file
        let path = "/sys/bus/w1/devices/28-01191ee2142f/w1_slave"; // Replace '28-xxxx' with your sensor's ID
        let mut file = File::open(path).map_err(|e| e.to_string())?; // Convert the error to a String;

        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| e.to_string())?;

        // The temperature data is usually at the end of the second line, after 't='
        let temp_str = contents.split_whitespace().last().unwrap().split("=").nth(1).unwrap();
        let temp_raw: i32 = temp_str.parse().unwrap();

        // Convert raw temperature to Celsius
        Ok(temp_raw as f32 / 1000.0)
    }

    #[cfg(not(feature = "hardware-support"))]
    {
        println!("Hardware support is not enabled. Temp running in stub mode.");
        Ok(69.69)
    }
}

#[tauri::command]
fn set_fan_speed(speed: u8) {
    #[cfg(feature = "hardware-support")]
    {
        let port_name = "/dev/ttyACM0"; // Adjust this to match your Arduino's serial port
        let baud_rate = 9600;

        let mut port = serialport::new(port_name, baud_rate)
            .timeout(Duration::from_millis(1000))
            .open()
            .expect("Failed to open port");

        println!("Successfully opened port {}", port_name);
        let status = if speed == 0 { "OFF" } else { "ON" };
        let message = format!("LED_{}\n", status); // Change this to send different commands
        println!("Sending: {}", message);
        port.write(message.as_bytes()).expect("Failed to write to serial port");

        // Read response
        let mut serial_buf: Vec<u8> = vec![0; 32]; // A small buffer is enough
        match port.read(serial_buf.as_mut_slice()) {
            Ok(bytes) => {
                let response = String::from_utf8_lossy(&serial_buf[..bytes]);
                println!("Received: {}", response);
            },
            Err(e) => println!("Error reading: {:?}", e),
        }
    }

    #[cfg(not(feature = "hardware-support"))]
    {
        let message = format!("Hardware support is not enabled. Speed: {}", speed);
        println!("{}", message);
        // Implement any stub behavior you need here.
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![set_fan_speed, get_temperature])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
