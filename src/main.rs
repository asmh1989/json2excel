#![allow(dead_code)]

use json::D;
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use std::{collections::HashMap, io::Write, path::Path};
use structopt::StructOpt;
use xlsxwriter::{FormatAlignment, FormatColor, Workbook};

use crate::args::Opt;

mod args;
mod config;
mod download;
mod json;

#[derive(Debug, Serialize, Deserialize)]
struct ADMET {
    input: Vec<Value>,
    output: Vec<Vec<Value>>,
}

const TMP_DIR: &str = ".tmp_smiles_2_img";

fn to_excel(p: &ADMET, name: &str, map: &mut HashMap<String, String>) {
    let workbook = Workbook::new(&format!("{}.xlsx", name));

    let format = workbook
        .add_format()
        .set_font_color(FormatColor::Green)
        .set_align(FormatAlignment::CenterAcross)
        .set_align(FormatAlignment::VerticalCenter);

    let format2 = workbook
        .add_format()
        // .set_font_size(16.)
        // .set_font_color(FormatColor::Red)
        // .set_bold()
        .set_align(FormatAlignment::CenterAcross)
        .set_align(FormatAlignment::VerticalCenter);

    let format1 = workbook.add_format().set_align(FormatAlignment::Center);

    let _ = std::fs::create_dir(TMP_DIR);

    let mut sheet = workbook.add_worksheet(Some("Sheet")).unwrap();

    let mut first_one = true;

    let mut r = 4;

    p.output.iter().enumerate().for_each(|(i, f)| {
        let smiles = p.input.get(i).unwrap().as_str().unwrap();

        let name = if let Some(ee) = map.get(smiles) {
            ee.as_str()
        } else {
            smiles
        };

        let first = 0;
        let mut y = first;
        sheet.write_string(y, 0, "序号", Some(&format1)).unwrap();
        sheet.write_string(y, 1, "目标分类", Some(&format)).unwrap();
        sheet.write_string(y, 2, "目标名字", None).unwrap();
        sheet.write_string(y, 3, "单位", None).unwrap();
        sheet.set_column(0, 0, 6., None).unwrap();
        sheet.set_column(0, r, 15., None).unwrap();
        sheet.set_column(1, 1, 20., None).unwrap();
        sheet.set_column(2, 2, 60., None).unwrap();
        sheet.set_column(3, 3, 10., None).unwrap();

        sheet.write_string(0, r, name, Some(&format2)).unwrap();

        y += 1;

        let mut c_y = 1;
        let mut category = "";
        if first_one {
            f.iter().for_each(|v| {
                let array = v.as_array().unwrap();
                let index = array.get(0).unwrap().as_f64().unwrap() + 1.;
                let c = array.get(1).unwrap().as_str().unwrap();
                let m1 = array.get(2).unwrap().as_str().unwrap();
                let m2 = array.get(3).unwrap().as_str().unwrap();
                // let v1 = array.get(4).unwrap().to_string();
                let v2 = array.get(5).unwrap().as_str().unwrap();
                sheet.write_number(y, 0, index, Some(&format1)).unwrap();
                sheet
                    .write_string(y, 2, &format!("{} {}", m1, m2), None)
                    .unwrap();
                sheet.write_string(y, 3, &format!("{}", v2), None).unwrap();

                sheet.write_string(y, 1, c, Some(&format)).unwrap();
                if c != category {
                    if y - c_y > 1 {
                        sheet
                            .merge_range(c_y, 1, y - 1, 1, category, Some(&format))
                            .unwrap();
                    }
                    category = c;
                    c_y = y
                }

                y += 1;
            });

            if y - c_y > 1 {
                sheet
                    .merge_range(c_y, 1, y - 1, 1, category, Some(&format))
                    .unwrap();
            }

            first_one = false;
        }

        y = first + 1;
        // sheet.write_string(1, r, "值", None).unwrap();

        f.iter().for_each(|v| {
            let array = v.as_array().unwrap();
            let v1 = array.get(4).unwrap().to_string();

            sheet
                .write_string(y, r, &format!("{}", v1.replace("\"", "")), None)
                .unwrap();
            y += 1;
        });

        r += 1;
    });

    workbook.close().unwrap();

    let _ = std::fs::remove_dir_all(TMP_DIR);
}

pub fn file_exist(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
}

fn to_csv(p: &ADMET, name: &str, map: &mut HashMap<String, String>) {
    let len = p.output.len();

    let file_name = format!("{}.csv", name);

    if file_exist(&file_name) {
        let _ = std::fs::remove_file(&file_name);
    }

    let mut file = std::fs::File::create(&file_name).unwrap();

    let mut vec: Vec<String> = Vec::with_capacity(len + 1);

    let mut cc = "ID".to_string();
    if map.is_empty() {
        cc = "Smiles".to_string();
    }
    p.output.get(0).unwrap().iter().for_each(|f| {
        let array = f.as_array().unwrap();
        let c = array.get(1).unwrap().as_str().unwrap();
        let m1 = array.get(2).unwrap().as_str().unwrap();
        let m2 = array.get(3).unwrap().as_str().unwrap();
        let v2 = array.get(5).unwrap().as_str().unwrap();

        cc = format!("{},{}-{} {}-{}", cc, c, m1, m2, v2);
    });

    vec.push(cc);

    p.output.iter().enumerate().for_each(|(i, f)| {
        let smiles = p.input.get(i).unwrap().as_str().unwrap();

        let name = if let Some(ee) = map.get(smiles) {
            ee.as_str()
        } else {
            smiles
        };

        let mut cc = name.to_string();

        f.iter().for_each(|v| {
            let array = v.as_array().unwrap();
            let v1 = array.get(4).unwrap().to_string();
            let vv = &format!("{}", v1.replace("\"", ""));

            cc = format!("{},{}", cc, vv);
        });
        vec.push(cc);
    });

    vec.into_iter().for_each(|f| {
        file.write_all(format!("{}\n", &f).as_bytes())
            .expect("write output file error !");
    });
}

fn read_csv<P: AsRef<Path>>(path: P, v: &mut HashMap<String, String>) {
    // let file = File::open(path)?;
    let mut rdr = csv::ReaderBuilder::new()
        .delimiter(b',')
        .has_headers(true)
        .from_path(path)
        .unwrap();
    for result in rdr.deserialize() {
        let ele: D = result.unwrap();

        if !ele.smiles.is_empty() {
            v.insert(ele.smiles.clone(), ele.id.clone());
        }
    }
}

fn main() -> Result<()> {
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    let opt: Opt = Opt::from_args();

    // 打印版本
    if opt.version {
        println!("{}", VERSION);
        return Ok(());
    }
    crate::config::init_config();

    let path = opt.input.map_or_else(|| "".to_string(), |f| f);

    let id = opt.id.map_or_else(|| "".to_string(), |f| f);

    let mut map: HashMap<String, String> = HashMap::new();

    if !id.is_empty() {
        read_csv(id, &mut map);
    }

    if !file_exist(&path) {
        info!("{} 输入文件不存在!", path);
        return Ok(());
    }
    info!("开始转换...");

    let name = std::path::PathBuf::from(&path)
        .file_stem()
        .unwrap()
        .to_os_string()
        .into_string()
        .unwrap();

    let str = std::fs::read_to_string(path).unwrap();

    let fix_str = str.replace("NaN", "\"NaN\"");
    let fix_str = fix_str.replace("Infinity", "999999999.99");
    let p: ADMET = serde_json::from_str(&fix_str)?;

    if opt.csv {
        to_csv(&p, &name, &mut map);
        info!("完成转换, 输出文件 = {}.csv", name);
    } else {
        to_excel(&p, &name, &mut map);

        info!("完成转换, 输出文件 = {}.xlsx", name);
    }

    Ok(())
}
