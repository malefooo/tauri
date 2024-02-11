use std::collections::HashMap;
use std::fs;
use std::path::Path;
use anyhow::anyhow;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::InvokeError;
use tokio::fs::File;
use tokio::sync::Mutex;
use crate::photo::{Photo, photo_list, PhotoType};
use crate::station::{STATION, Station, TreeNode};
use crate::station::kml::kml_to_json;
use crate::utils::{ensure_dir_exists, new_invoke_err, to_invoke_err};

pub static BELONG_MAP: Lazy<Mutex<HashMap<Station, HashMap<Photo, bool>>>> = Lazy::new(|| {
    Mutex::new(HashMap::new())
});

/// 1米 = 0.00001141经度
pub static ONE_METERS_TO_LONGITUDE: f64 = 0.00001141;

/// 1米 = 0.00000899纬度
pub static ONE_METERS_TO_LATITUDE: f64 = 0.00000899;

#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct CalcPhotoResult {
    pub normal: u64,
    pub infrared: u64,
}

impl CalcPhotoResult {
    pub fn to_tree_node(&self) -> Vec<TreeNode> {
        vec![
            TreeNode{
                key: "normal".to_string(),
                label: format!("普通: {}",self.normal),
                children: None,
            },
            TreeNode{
                key: "infrared".to_string(),
                label: format!("红外: {}",self.infrared),
                children: None,
            }
        ]
    }
}

pub async fn judge_photo_belong(radius: &str, photo_path: &str) -> anyhow::Result<()> {
    
    let radius: f64 = radius.parse().map_err(|e: std::num::ParseFloatError|anyhow!(e))?;
    let circle = radius * radius;
    
    let photos = photo_list(photo_path).await?;

    let stations = STATION.lock().await.clone();

    let mut belong_map: HashMap<Station, HashMap<Photo, bool>> = HashMap::new();

    for station in stations.into_iter() {
        let station_y = station.latitude / ONE_METERS_TO_LATITUDE;
        let station_x = station.longitude / ONE_METERS_TO_LONGITUDE;

        for (photo, b_) in photos.iter() {
            let photo_y = photo.latitude / ONE_METERS_TO_LATITUDE;
            let photo_x = photo.longitude / ONE_METERS_TO_LONGITUDE;

            let line = ((photo_x - station_x)*(photo_x - station_x) + (photo_y - station_y)*(photo_y - station_y)).abs();

            // println!("station: {}, line: {}, circle: {}", station.name, line, circle);
            if line > circle {
                continue
            } else {
                let mut photo_map_exist = false;
                if let Some(map) = belong_map.get_mut(&station) {
                    map.insert(photo.clone(), true);
                    photo_map_exist = true;
                }

                if !photo_map_exist {
                    let mut photo_map = HashMap::new();
                    photo_map.insert(photo.clone(), true);
                    belong_map.insert(station.clone(), photo_map);
                }
            }
        }
    }

    *BELONG_MAP.lock().await = belong_map;
    
    Ok(())
}

#[tauri::command]
pub async fn calc_photo(radius: &str, photo_path: &str) -> Result<String, InvokeError> {

    let _ = judge_photo_belong(radius, photo_path).await.map_err(to_invoke_err)?;


    let map = BELONG_MAP.lock().await.clone();
    let mut total_result = CalcPhotoResult::default();
    let mut tree_node_list = vec![];

    let mut station_tree_node_list = vec![];

    for (station, photo_map) in map.iter() {
        let mut cr = CalcPhotoResult::default();
        for (photo, _) in photo_map.into_iter() {
            match photo.photo_type {
                PhotoType::Normal => {
                    total_result.normal+=1;
                    cr.normal += 1;
                }
                PhotoType::Infrared => {
                    total_result.infrared+=1;
                    cr.infrared += 1;
                }
            }
        }

        if cr.normal != 0 || cr.infrared != 0 {
            station_tree_node_list.push(TreeNode{
                key: station.name.clone(),
                label: station.name.clone(),
                children: Some(cr.to_tree_node()),
            });
        }
    }

    tree_node_list.push(TreeNode{
        key: "total".to_string(),
        label: "总数".to_string(),
        children: Some(total_result.to_tree_node()),
    });
    tree_node_list.extend_from_slice(station_tree_node_list.as_slice());

    let json = serde_json::to_string(&tree_node_list).map_err(|e|new_invoke_err(e.to_string().as_str()))?;

    Ok(json)

}

#[tauri::command]
pub async fn move_to_output(output: &str) -> Result<(), InvokeError> {
    // let _ = ensure_dir_exists(output).map_err(to_invoke_err)?;
    let output = Path::new(output);
    let map = BELONG_MAP.lock().await.clone();

    for (station, photo_map) in map.iter() {

        let station_path = output.join(station.name.clone());
        let station_path_str = station_path.to_str().ok_or(new_invoke_err("station path is null"))?.to_string();

        ensure_dir_exists(station_path_str.as_str()).map_err(to_invoke_err)?;
        for (photo, _) in photo_map.iter() {
            let dst_file = station_path.join(photo.file_name.as_str()).to_str().ok_or(new_invoke_err("dst file path is null"))?.to_string();;
            fs::copy(photo.path.clone(), dst_file.as_str()).map_err(|e|new_invoke_err(e.to_string().as_str()))?;
        }
    }
    
    Ok(())

}


#[test]
fn test1() {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let station_path = "C:\\Users\\yunyc\\Downloads\\福丰I线.kml";
        kml_to_json(station_path).await.unwrap();

        let radius = "100.0";
        let photo_input = "C:\\Users\\yunyc\\Downloads\\photo";
        let photo_output = "C:\\Users\\yunyc\\Downloads\\photo\\outpuuuu";

        judge_photo_belong(radius, photo_input).await.unwrap();
        move_to_output(photo_output).await.unwrap();
    });
}

#[test]
fn test2() {
    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let station_path = "C:\\Users\\yunyc\\Downloads\\福丰I线.kml";
        kml_to_json(station_path).await.unwrap();

        let radius = "100.0";
        let photo_input = "C:\\Users\\yunyc\\Downloads\\photo";

        let str = calc_photo(radius, photo_input).await.unwrap();
        println!("{}",str);
    });
}