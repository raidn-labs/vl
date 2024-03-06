// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Conditionally compile `rppal` dependency only when `hardware-support` feature is enabled
#[cfg(feature = "hardware-support")]
use rppal::gpio::Gpio;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn turn_on() {
    #[cfg(feature = "hardware-support")]
    {
        const LED_PIN: u8 = 17; // Use GPIO 17 as an example; adjust as necessary for your setup.

        let mut pin = Gpio::new().expect("Failed to access GPIO").get(LED_PIN).expect("Failed to access pin").into_output();

        println!("Flashing the LED on pin {}.", LED_PIN);
        for _ in 0..5 {
            pin.set_high();
            std::thread::sleep(std::time::Duration::from_secs(1));
            pin.set_low();
            std::thread::sleep(std::time::Duration::from_secs(1));
        }
        println!("Done flashing the LED.");
    }

    #[cfg(not(feature = "hardware-support"))]
    {
        println!("Hardware support is not enabled. Running in stub mode.");
        // Implement any stub behavior you need here.
    }
}

#[tauri::command]
fn turn_off() {
    // turn off
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, turn_on, turn_off])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
