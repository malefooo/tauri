use std::mem;
use anyhow::anyhow;
use calamine::{DataType, open_workbook_auto, RangeDeserializerBuilder, Reader, Xlsx};
use tauri::InvokeError;
use crate::utils::{is_excel_file, file_name, new_invoke_err, to_invoke_err};
use crate::station::{Station, STATION, TreeNode};

#[tauri::command]
pub async fn excel_to_json(excel_file: &str) -> Result<String, InvokeError> {

    if !is_excel_file(excel_file) {
        return Err(new_invoke_err("not excel file"));
    }

    let mut station_data = vec![];

    let mut workbook = open_workbook_auto(excel_file).map_err(|e|anyhow!(e)).map_err(to_invoke_err)?;

    let sheet_names = workbook.sheet_names();
    let first_sheet_name = sheet_names.first().ok_or(anyhow::Error::msg("sheet1 not exist")).map_err(to_invoke_err)?;
    let range = workbook.worksheet_range(first_sheet_name.as_str()).map_err(|e|anyhow!(e)).map_err(to_invoke_err)?;

    for (idx,x) in range.rows().enumerate() {
        if idx == 0 {
            continue
        }

        let name = &x[0].as_string().ok_or(new_invoke_err("name is null"))?;
        let longitude = &x[1].as_string().ok_or(new_invoke_err("longitude is null"))?;
        let latitude = &x[2].as_string().ok_or(new_invoke_err("latitude is null"))?;
        let height = &x[3].as_string().ok_or(new_invoke_err("height is null"))?;

        let station = Station {
            name: name.clone(),
            longitude: longitude.parse().map_err(|e: std::num::ParseFloatError| new_invoke_err(e.to_string().as_str()))?,
            latitude: latitude.parse().map_err(|e: std::num::ParseFloatError| new_invoke_err(e.to_string().as_str()))?,
            height: height.parse().map_err(|e: std::num::ParseFloatError| new_invoke_err(e.to_string().as_str()))?,
        };

        station_data.push(station);

    }

    *STATION.lock().await = station_data.clone();

    let station_node: Vec<TreeNode> = station_data.into_iter().map(|v|v.into()).collect();

    let line_name = file_name(excel_file).map_err(to_invoke_err)?;
    let line_node = TreeNode{
        key: line_name.clone(),
        label: line_name.clone(),
        children: Some(station_node),
    };

    let json = serde_json::to_string(&vec![line_node]).map_err(|e|anyhow!(e)).map_err(to_invoke_err)?;

    Ok(json)
}