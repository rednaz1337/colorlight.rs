use std::io::ErrorKind::WouldBlock;
use std::net::{SocketAddr, UdpSocket};
use std::time::{Duration, Instant};

const IP_ADDRESS: &str = "192.168.178.50";

const PANEL_WIDTH: usize = 64;
const PANEL_HEIGHT: usize = 32;

const PORTS: [u16; 4] = [
    26177, // top left
    26184, // top right
    26180, // bottom left
    26192,
]; // bottom right

//const OFFSET_X: usize = 0;
//const OFFSET_Y: usize = 0;

const OFFSETS: [[usize; 2]; 4] = [[0, 0], [64, 0], [0, 32], [64, 32]];
fn main() {
    // Each panel has its own socket. Iterate over them and create a socket
    let sockets: Vec<UdpSocket> = PORTS
        .iter()
        .map(|port| {
            let socket = UdpSocket::bind(("0.0.0.0", *port)).expect("Failed to bind socket!");
            socket
                .connect((IP_ADDRESS, *port))
                .expect("Failed to connect socket!");
            socket
        })
        .collect();

    let display = scrap::Display::primary().expect("Can't open primary Display");

    let mut capturer = scrap::Capturer::new(display).expect("Can't create capturer");
    let scr_width = capturer.width();

    let mut row_data = Vec::new();

    //let now = Instant::now();
    loop {
        match capturer.frame() {
            Ok(frame) => {
                process_frame(frame, &mut row_data, &sockets, scr_width);
            }
            Err(ref e) if e.kind() == WouldBlock => {
                // Wait for the frame.
            }
            Err(_) => {
                break;
            }
        }
    }
    //println!("{} us", now.elapsed().as_micros());
}

fn process_frame(frame: scrap::Frame, row_data: &mut Vec<u8>, sockets: &Vec<UdpSocket>, scr_width: usize) {
    //let image_data: Vec<&[u8]> = frame.chunks(4).collect();
    for (socket, offset) in sockets.iter().zip(OFFSETS.iter()) {
        //update_panel(socket, offset[0], offset[1], capt.get_stored_frame().unwrap(), scr_width as usize);
        for y in 0..PANEL_HEIGHT {
            for x in 0..PANEL_WIDTH {
                let index = ((y + offset[1]) * scr_width + (x + offset[0])) * 4;
                row_data.extend_from_slice(&pixel_to_packet(
                    x as u8,
                    y as u8,
                    frame[index + 2] / 4,
                    frame[index + 1] / 4,
                    frame[index] / 4,
                ));
            }
            socket.send(&row_data).expect("Failed to send data");
            row_data.clear();
        }
    }

    //std::thread::sleep(Duration::from_millis(25));
}

fn pixel_to_packet(x: u8, y: u8, r: u8, g: u8, b: u8) -> [u8; 4] {
    let mut data = [0, 0, 0, 0];

    data[0] = y & 0b00111111;
    data[1] = (x << 2) | (r >> 4);
    data[2] = (r << 4) | (g >> 2);
    data[3] = (g << 6) | b;
    data
}
