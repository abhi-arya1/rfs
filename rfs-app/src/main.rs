use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod app1;
mod app2;
mod consts;

// Define a telemetry struct for sending messages
struct Telemetry {
    id: u32,
    data: String,
}

// Define the function for app1
fn app1(tx: mpsc::Sender<Telemetry>) {
    let app_name = "App1";

    // Simulate app1 sending telemetry data
    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 0..5 {
            let telemetry = Telemetry {
                id: i,
                data: format!("Telemetry {} from {}", app2::app2_main(), app_name),
            };
            tx1.send(telemetry).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
}

// Define the function for app2
fn app2(tx: mpsc::Sender<Telemetry>) {
    let app_name = "App2";

    // Simulate app2 sending telemetry data
    let tx2 = tx.clone();
    thread::spawn(move || {
        for i in 0..5 {
            let telemetry = Telemetry {
                id: i,
                data: format!("Telemetry {} from {}", i, app_name),
            };
            tx2.send(telemetry).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });
}

// Main program to run both apps and send telemetry data to the ground station
fn main() {
    // Create a channel for telemetry messages
    let (tx, rx) = mpsc::channel();

    // Run app1 and app2 in separate threads
    let tx1 = tx.clone();
    let app1_thread = thread::spawn(move || {
        app1(tx1);
    });

    let tx2 = tx.clone();
    let app2_thread = thread::spawn(move || {
        app2(tx2);
    });

    let ground_station_thread = thread::spawn(move || {
        let socket = UdpSocket::bind("0.0.0.0:8888").expect("Couldn't bind to address");
        let ground_station_addr = "127.0.0.1:9999"; 
        
        while let Ok(telemetry) = rx.recv() {
            let message = format!("ID: {}, Data: {}", telemetry.id, telemetry.data);
            println!("Sending to ground station: {}", message);
            socket.send_to(message.as_bytes(), ground_station_addr).expect("Couldn't send data");
        }
    });

    // Wait for all threads to finish
    app1_thread.join().unwrap();
    app2_thread.join().unwrap();
    ground_station_thread.join().unwrap();
}
