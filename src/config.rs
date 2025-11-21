use serde::Deserialize;

type Point = (u8, u8);

#[derive(Deserialize)]
pub struct Config {
    pub graphics: Graphics,
    pub data: Data,
}

#[derive(Deserialize)]
pub struct Graphics {
    pub width: u16,
    pub height: u16,
    pub cube_size: u8,
}

#[derive(Deserialize)]
pub struct Data {
    pub values: Vec<Point>,
}
