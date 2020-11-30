use std::io::ErrorKind::WouldBlock;

mod panel;

///divisor is the FPS. I haven't found a proper way to give me the exact FPS yet, this will be ≈ 30 FPS
const TARGET_TIME: u64 = 1000 / 40;

fn main() {
    let mut config_json = // Default config if nothing is provided
    r#"{
        "offset": {
            "x": 0,
            "y": 0
        },
        "size": {
            "x": 64,
            "y": 32
        },
        "panels": [
            {
                "socket": "192.168.178.50:26177",
                "position": {
                    "x": 0,
                    "y": 0
                }
            },
            {
                "socket": "192.168.178.50:26184",
                "position": {
                    "x": 64,
                    "y": 0
                }
            },
            {
                "socket": "192.168.178.50:26180",
                "position": {
                    "x": 0,
                    "y": 32
                }
            },
            {
                "socket": "192.168.178.50:26192",
                "position": {
                    "x": 64,
                    "y": 32
                }
            }
        ]
    }"#.to_string();

    if std::env::args().len() >= 2 { // use second paramteter as JSON config
        config_json = std::env::args().nth(1).unwrap();
    }

    let panels = panel::Panels::from_json(&config_json).expect("Failed to parse JSON");
    #[cfg(debug_assertions)] // execuete the next line only in debug builds
    println!("Deserialized JSON: {:#?}", panels);
    let mut panel_writer = panel::PanelWriter::new(panels);

    let display = scrap::Display::primary().expect("Can't open primary Display");
    let mut capturer = scrap::Capturer::new(display).expect("Can't create capturer");
    let (scr_width, scr_height) = (capturer.width(), capturer.height());

    // setup for the fps counter
    let mut frame_duration = std::time::Instant::now();
    let mut frame_counter = 0;
    let mut time_acc = 0.0;

    // run this forever
    loop {
        // try to capture a frame
        match capturer.frame() {
            Ok(frame) => {
                //show the frame
                panel_writer.display_image(&frame[..], scr_width, scr_height).expect("Failed to display frame");
                std::thread::sleep(std::time::Duration::from_millis(TARGET_TIME));
                
                // calculate FPS
                time_acc += frame_duration.elapsed().as_secs_f64();
                frame_counter += 1;
                if frame_counter == 50 {
                    let avg_time = time_acc / (frame_counter as f64);
                    println!("FPS: {}", 1.0 / avg_time);
                    frame_counter = 0;
                    time_acc = 0.0;
                }
                frame_duration = std::time::Instant::now();
            }
            Err(ref e) if e.kind() == WouldBlock => {
                // Wait for the frame.
            }
            Err(_) => {
                break;
            }
        }
    };
}


