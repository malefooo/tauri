
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};

pub mod kml;
mod excel;

pub static STATION: Lazy<Station> = Lazy::new(|| {
    Station::default()
});

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Station {
    pub name: String,
    pub longitude: f64,
    pub latitude: f64,
    pub height: f64,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct TreeNode {
    key: String,
    label: String,
    children: Option<Vec<TreeNode>>,
}

impl From<Station> for TreeNode {
    fn from(station: Station) -> Self {
        TreeNode {
            key: station.name.clone(),
            label: station.name.clone(),
            children: Some(vec![
                TreeNode { key: "longitude".to_string(), label: format!("经度: {}",station.longitude), children: None },
                TreeNode { key: "latitude".to_string(), label: format!("纬度: {}", station.latitude), children: None },
                TreeNode { key: "height".to_string(), label: format!("高度: {}", station.height), children: None },
            ]),
        }
    }
}
