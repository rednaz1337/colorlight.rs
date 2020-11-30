use serde::{de::Error, Deserialize, Deserializer};
use std::net::UdpSocket;
#[derive(Deserialize, Debug)]
pub struct Point {
    x: usize,
    y: usize,
}
#[derive(Deserialize, Debug)]
pub struct Panels {
    size: Point,
    offset: Point,
    panels: Vec<Panel>,
}
#[derive(Deserialize, Debug)]
pub struct Panel {
    position: Point,
    #[serde(deserialize_with = "udp_socket_from_string")]
    socket: UdpSocket,
}

pub struct PanelWriter {
    panels: Panels,
    row_data: Vec<u8>,
}

impl Panels {
    /// Creates the panel Structs from a Json String
    /// An example string can be found in `main.rs`
    pub fn from_json(json: &str) -> Result<Self, String> {
        let deserialized: Self = serde_json::from_str(json).map_err(|e| e.to_string())?;

        Ok(deserialized)
    }
}

/// This struct is used to send image data to the LED panel
impl PanelWriter {
    pub fn new(panels: Panels) -> Self {
        let row_data = Vec::new();

        Self { panels, row_data }
    }

    /// Displays an image in the format BGRA on the LED Panels, with a color Depth of 8 bit per channel, which will be reduced to 6 bit per channel
    pub fn display_image(
        &mut self,
        image_data: &[u8],
        image_width: usize,
        _image_height: usize, ) -> Result<(), String> {

        for panel in self.panels.panels.iter() {
            for y in 0..self.panels.size.y {
                for x in 0..self.panels.size.x {
                    let index = 
                          ((y + panel.position.y + self.panels.offset.y) * image_width
                        + (x + panel.position.x + self.panels.offset.x))
                        * 4;
                    self.row_data.extend_from_slice(&pixel_to_packet(
                        x as u8,
                        y as u8,
                        image_data[index + 2] / 4,
                        image_data[index + 1] / 4,
                        image_data[index] / 4,
                    ));
                }
                panel
                    .socket
                    .send(&self.row_data)
                    .map_err(|e| e.to_string())?;
                self.row_data.clear();
            }
        }

        Ok(())
    }
}

/// takes a position and a color and creates a 4 byte UDP packet for the LED panel
fn pixel_to_packet(x: u8, y: u8, r: u8, g: u8, b: u8) -> [u8; 4] {
    let mut data = [0, 0, 0, 0];

    data[0] = y & 0b00111111;
    data[1] = (x << 2) | (r >> 4);
    data[2] = (r << 4) | (g >> 2);
    data[3] = (g << 6) | b;
    data
}

/// creates a UdpSocket from a string in Json
fn udp_socket_from_string<'de, D>(deserializer: D) -> Result<UdpSocket, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let split: Vec<&str> = s.split(":").collect();
    if split.len() != 2 {
        Err(Error::custom("Wrong format for Socket IP and Port"))
    } else {
        let port: u16 = split[1]
            .parse()
            .map_err(|_| Error::custom("Failed to parse Port"))?;
        let socket =
            UdpSocket::bind(("0.0.0.0", port)).map_err(|_| Error::custom("Failed to bind Port"))?;
        socket
            .connect((split[0], port))
            .map_err(|_| Error::custom("Failed to connect to Socket"))?;
        Ok(socket)
    }
}
