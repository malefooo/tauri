use std::collections::HashMap;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::BufReader;
use std::path::Path;
use exif::{Exif, Field, In, Reader, Tag};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use tauri::InvokeError;
use tokio::fs;
use tokio::sync::Mutex;
use crate::utils::{file_name, to_invoke_err};

pub static PHOTOS: Lazy<Mutex<HashMap<Photo, bool>>> = Lazy::new(|| { Mutex::new(HashMap::new()) });

pub static PHOTOS_PATH: Lazy<Mutex<String>> = Lazy::new(|| {Mutex::new(String::new())});

#[derive(Default, Debug, Serialize, Deserialize, Clone,PartialEq,Eq,Hash)]
pub enum PhotoType {
    #[default]
    Normal,
    Infrared,
}


#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Photo {
    pub longitude: f64,
    pub latitude: f64,
    pub photo_type: PhotoType,
    pub path: String,
    pub file_name: String,
}

impl Eq for Photo {}

impl PartialEq for Photo{
    fn eq(&self, other: &Self) -> bool {
        if self.photo_type == other.photo_type && self.longitude == other.longitude && self.latitude == other.latitude {
            true
        } else {
            false
        }
    }
}

impl Hash for Photo {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.photo_type.hash(state);
        self.latitude.to_string().hash(state);
        self.longitude.to_string().hash(state);
    }
}

#[tauri::command]
pub async fn input_photos(path: &str) -> Result<(), InvokeError> {
    let _ = photo_list(path).await.map_err(to_invoke_err)?;

    Ok(())
}

pub async fn photo_list(path: &str) -> anyhow::Result<HashMap<Photo, bool>> {
    let mut entries = fs::read_dir(path).await?;

    let mut map = HashMap::new();

    while let Some(entry) = entries.next_entry().await? {
        let path = entry.path();
        if path.is_file() && is_photo(&path) {
            let photo = get_photo(path.to_str().unwrap())?;
            map.insert(photo, true);
        }
    }

    *PHOTOS.lock().await = map.clone();
    *PHOTOS_PATH.lock().await = path.to_string();

    Ok(map)
}



fn is_photo(path: &Path) -> bool {
    let extension = path.extension().and_then(std::ffi::OsStr::to_str).unwrap().to_lowercase();
    match extension.as_str() {
        "jpg" => true,
        _ => false,
    }
}

fn get_photo(path: &str) -> anyhow::Result<Photo> {
    println!("path: {}", path);
    let mut file = File::open(path)?;
    let mut reader = BufReader::new(&mut file);
    let exif_data = Reader::new().read_from_container(&mut reader)?;

    let longitude = get_gps_info(&exif_data, Tag::GPSLongitude)?;
    let latitude = get_gps_info(&exif_data, Tag::GPSLatitude)?;
    let photo_type = get_photo_type(path)?;
    let photo_name = file_name(path)? + ".JPG";

    Ok(Photo{
        longitude,
        latitude,
        photo_type,
        path: path.to_string(),
        file_name: photo_name,
    })

}

fn get_gps_info(exif_data: &Exif, tag: Tag) -> anyhow::Result<f64> {
    if let Some(field) = exif_data.get_field(tag, In::PRIMARY) {
        convert_gps_field(field, tag)
    } else {
        Err(anyhow::Error::msg(format!("get field [{}] is null", tag)))
    }
}

fn convert_gps_field(field: &Field, tag: Tag) -> anyhow::Result<f64> {

    let value = field.value.display_as(tag).to_string();
    let split: Vec<&str> = value.split(" ").collect();

    if split.len() == 6 {
        let degrees = split[0].parse::<f64>()?;
        let minutes = split[2].parse::<f64>()?;
        let seconds = split[4].parse::<f64>()?;

        let decimal = degrees + (minutes / 60.0) + (seconds / 3600.0);
        Ok(decimal)
    } else {
        Err(anyhow::Error::msg("split len not eq 6"))
    }
}

fn get_photo_type(path: &str) -> anyhow::Result<PhotoType> {
    let parts: Vec<_> = path.split(".").collect();
    let parts = parts[0].split("_");
    let last_str = parts.last().ok_or(anyhow::Error::msg("last str is null"))?;

    Ok(if last_str.to_uppercase() == "V" {
        PhotoType::Normal
    } else {
        PhotoType::Infrared
    })
}

#[test]
fn test() {


    let rt = tokio::runtime::Builder::new_current_thread().enable_all().build().unwrap();

    rt.block_on(async {
        let path = "C:\\Users\\yunyc\\Downloads\\photo";
        let photos = photo_list(path).await.unwrap();
        println!("{:?}", photos);
    });
}