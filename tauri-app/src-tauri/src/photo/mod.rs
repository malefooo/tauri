use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub enum PhotoType {
    #[default]
    Normal,
    Infrared,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Photo {
    pub longitude: f64,
    pub latitude: f64,
    pub photo_type: PhotoType
}

