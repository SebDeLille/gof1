use serde::Deserialize;

#[derive(Deserialize)]
pub struct Config {
    pub graphics: Graphics,
}

#[derive(Deserialize)]
pub struct Graphics {
    pub width: u16,
    pub height: u16,
    pub cube_size: u8,
}
