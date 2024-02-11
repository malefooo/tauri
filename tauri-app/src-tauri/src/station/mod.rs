use std::cell::RefCell;
use std::hash::{Hash, Hasher};
use once_cell::sync::{Lazy, OnceCell};
use serde::{Deserialize, Serialize};
use tauri::async_runtime::Mutex;
use crate::utils::file_name;

pub mod kml;
pub mod excel;

pub static STATION: Lazy<Mutex<Vec<Station>>> = Lazy::new(|| {
    Mutex::new(vec![])
});

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Station {
    pub name: String,
    pub longitude: f64,
    pub latitude: f64,
    pub height: f64,
}

impl PartialEq for Station {
    fn eq(&self, other: &Self) -> bool {
        if self.height == other.height && self.latitude == other.latitude && self.longitude == other.longitude && self.name == other.name {
            true
        } else {
            false
        }
    }
}

impl Eq for Station {}

impl Hash for Station {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
        self.longitude.to_string().hash(state);
        self.latitude.to_string().hash(state);
        self.height.to_string().hash(state);
    }
}

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct TreeNode {
    pub key: String,
    pub label: String,
    pub children: Option<Vec<TreeNode>>,
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
