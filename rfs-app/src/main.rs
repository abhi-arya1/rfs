use std::net::UdpSocket;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod app1;
mod app2;
mod consts;
mod pkt;



fn app1(tx: mpsc::Sender<pkt::RfsPacket>, rx: mpsc::Receiver<pkt::RfsPacket>) {
    let tx1 = tx.clone();
    thread::spawn(move || {
        for i in 0..5 {
            let telemetry = pkt::RfsTelemetry {
                id: i,
                data: format!("Telemetry {} from {}", app2::app2_main(), "APP1"),
            };
            tx1.send(telemetry).unwrap();
            thread::sleep(Duration::from_millis(500));
        }
    });
}


fn app2(tx: mpsc::Sender<pkt::RfsPacket>, rx: mpsc::Receiver<pkt::RfsPacket>) {
    let tx2 = tx.clone();
    thread::spawn(move || {
        for i in 0..5 {
            let telemetry = pkt::RfsTelemetry {
                id: i,
                data: format!("Telemetry {} from {}", i, "APP2"),
            };
            tx2.send(telemetry).unwrap();
            thread::sleep(Duration::from_millis(300));
        }
    });
}


fn main() {
    let (tx, rx) = mpsc::channel();

    let tx1 = tx.clone();
    let app1_thread = thread::spawn(move || {
        app1(tx1, rx);
    });

    let tx2 = tx.clone();=
    let app2_thread = thread::spawn(move || {
        app2(tx2, rx);
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

    app1_thread.join().unwrap();
    app2_thread.join().unwrap();
    ground_station_thread.join().unwrap();
}
