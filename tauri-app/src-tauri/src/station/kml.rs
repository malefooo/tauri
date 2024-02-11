use std::fs::File;
use std::io::BufReader;
use std::ops::DerefMut;
use std::{mem, path};
use std::path::Path;
use anyhow::anyhow;
use tauri::InvokeError;
use xlsxwriter::Workbook;
use xml::EventReader;
use xml::reader::XmlEvent;
use crate::station::{Station, STATION, TreeNode};
use crate::utils::{ensure_dir_exists, is_kml_file, file_name, new_invoke_err, to_invoke_err};

#[tauri::command]
pub fn kml_to_excel(kml_file: &str, output_dir: &str) -> Result<(), InvokeError> {

    ensure_dir_exists(output_dir).map_err(to_invoke_err)?;

    if is_kml_file(kml_file) {
        let data = kml_to_station_list(kml_file).map_err(to_invoke_err)?;

        let line_name = file_name(kml_file).map_err(to_invoke_err)?;
        let file_name = format!("{}.xlsx",line_name);

        // 将数据保存到Excel文件
        let output_file = Path::new(output_dir).join(file_name);
        let workbook = Workbook::new(output_file.as_os_str().to_str()
            .unwrap())
            .map_err(|e|anyhow!(e))
            .map_err(to_invoke_err)?;
        let mut sheet = workbook.add_worksheet(None).unwrap();
        let mut row = 0;

        sheet.write_string(0,0,"杆塔编号",None).unwrap();
        sheet.write_string(0,1,"经度",None).unwrap();
        sheet.write_string(0,2,"纬度",None).unwrap();
        sheet.write_string(0,3,"高度",None).unwrap();

        row += 1;

        for station in data.into_iter() {
            sheet.write_string(row,0,station.name.as_str(),None).unwrap();
            sheet.write_string(row,1,station.longitude.to_string().as_str(),None).unwrap();
            sheet.write_string(row,2,station.latitude.to_string().as_str(),None).unwrap();
            sheet.write_string(row,3,station.height.to_string().as_str(),None).unwrap();
            row += 1;
        }

        workbook.close().unwrap();
    } else {
        eprintln!("not kml file");
        return Err(new_invoke_err("not kml file"));
    }

    // let station = STATION

    Ok(())
}

#[tauri::command]
pub async fn kml_to_json(kml_file: &str) -> Result<String, InvokeError> {

    if !is_kml_file(kml_file) {
        return Err(new_invoke_err("not kml file"));
    }

    let station_data = kml_to_station_list(kml_file).map_err(to_invoke_err)?;

    *STATION.lock().await = station_data.clone();

    let station_node: Vec<TreeNode> = station_data.into_iter().map(|v|v.into()).collect();

    let line_name = file_name(kml_file).map_err(to_invoke_err)?;
    let line_node = TreeNode{
        key: line_name.clone(),
        label: line_name.clone(),
        children: Some(station_node),
    };
    let json = serde_json::to_string(&vec![line_node]).map_err(|e|anyhow!(e)).map_err(to_invoke_err)?;

    Ok(json)
}



fn kml_to_station_list(kml_file: &str) -> anyhow::Result<Vec<Station>> {
    let file = File::open(kml_file).unwrap();
    let file = BufReader::new(file);
    let mut parser = EventReader::new(file);
    let mut data: Vec<Station> = Vec::new();
    let mut station = Option::None;

    let mut in_placemark = false;
    let mut in_name = false;
    let mut in_point = false;
    let mut in_coordinates = false;

    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { ref name, attributes, .. }) => {
                match name.local_name.as_str() {
                    "Placemark" => {
                        in_placemark = true;
                        station.replace(Station::default());
                    },
                    "name" if in_placemark => in_name = true,
                    "Point" if in_placemark => in_point = true,
                    "coordinates" if in_point => in_coordinates = true,
                    _ => {}
                }
            }
            Ok(XmlEvent::Characters(content)) => {
                if in_name {
                    station.as_mut().unwrap().name = content;
                } else if in_coordinates {
                    let parts: Vec<_> = content.split(",").collect();
                    station.as_mut().unwrap().longitude = parts[0].parse().map_err(|e: std::num::ParseFloatError| anyhow!(e.clone()))?;
                    station.as_mut().unwrap().latitude = parts[1].parse().map_err(|e: std::num::ParseFloatError| anyhow!(e.clone()))?;
                    station.as_mut().unwrap().height = parts[2].parse().map_err(|e: std::num::ParseFloatError| anyhow!(e.clone())).map_err(to_invoke_err).unwrap_or_default();
                }
            }
            Ok(XmlEvent::EndElement { ref name }) => {
                match name.local_name.as_str() {
                    "Placemark" => {
                        in_placemark = false;
                        let station = station.clone().ok_or(anyhow::Error::msg("station is null"))?;
                        data.push(station);
                    }
                    "name" if in_placemark => in_name = false,
                    "Point" if in_placemark => in_point = false,
                    "coordinates" if in_point => in_coordinates = false,
                    _ => {}
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }

    Ok(data)
}