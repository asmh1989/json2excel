#![allow(dead_code)]

use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use structopt::StructOpt;
use xlsxwriter::{FormatAlignment, FormatColor, ImageOptions, Workbook};

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

fn to_excel(p: &ADMET, name: &str) {
    let workbook = Workbook::new(&format!("{}.xlsx", name));

    let format = workbook
        .add_format()
        .set_font_color(FormatColor::Green)
        .set_align(FormatAlignment::CenterAcross)
        .set_align(FormatAlignment::VerticalCenter);

    let format2 = workbook
        .add_format()
        .set_font_size(16.)
        // .set_bold()
        .set_align(FormatAlignment::CenterAcross)
        .set_align(FormatAlignment::VerticalCenter);

    let format1 = workbook.add_format().set_align(FormatAlignment::Center);

    let _ = std::fs::create_dir(TMP_DIR);

    p.output.iter().enumerate().for_each(|(i, f)| {
        let mut sheet = workbook.add_worksheet(None).unwrap();
        sheet
            .merge_range(0, 0, 0, 1, "SMILES", Some(&format2))
            .unwrap();
        sheet.merge_range(1, 5, 16, 9, "", None).unwrap();

        sheet.set_row(0, 30., None).unwrap();
        let smiles = p.input.get(i).unwrap().as_str().unwrap();

        let img = format!("{}/{}", TMP_DIR, smiles);

        let r = crate::download::fetch_url(smiles, img.clone());
        if r.is_err() {
            log::error!("smiles:{} 生成图片失败!", smiles);
        } else {
            sheet
                .insert_image_opt(
                    1,
                    5,
                    &img,
                    &ImageOptions {
                        x_offset: 14,
                        y_offset: 2,
                        x_scale: 1.0,
                        y_scale: 1.0,
                    },
                )
                .unwrap();
        }

        sheet
            .merge_range(0, 2, 0, 3, smiles, Some(&format2))
            .unwrap();

        let first = 1;
        let mut y = first;
        sheet.write_string(y, 0, "序号", Some(&format1)).unwrap();
        sheet.write_string(y, 1, "目标分类", Some(&format)).unwrap();
        sheet.write_string(y, 2, "目标名字", None).unwrap();
        sheet.write_string(y, 3, "值 (单位)", None).unwrap();
        sheet.set_column(0, 0, 6., None).unwrap();
        sheet.set_column(1, 1, 20., None).unwrap();
        sheet.set_column(2, 2, 60., None).unwrap();
        sheet.set_column(3, 3, 30., None).unwrap();

        y += 1;

        let mut c_y = 1;
        let mut category = "";
        f.iter().for_each(|v| {
            let array = v.as_array().unwrap();
            let index = array.get(0).unwrap().as_f64().unwrap() + 1.;
            let c = array.get(1).unwrap().as_str().unwrap();
            let m1 = array.get(2).unwrap().as_str().unwrap();
            let m2 = array.get(3).unwrap().as_str().unwrap();
            let v1 = array.get(4).unwrap().to_string();
            let v2 = array.get(5).unwrap().as_str().unwrap();
            sheet.write_number(y, 0, index, Some(&format1)).unwrap();
            sheet
                .write_string(y, 2, &format!("{} {}", m1, m2), None)
                .unwrap();
            sheet
                .write_string(y, 3, &format!("{} {}", v1.replace("\"", ""), v2), None)
                .unwrap();

            // info!(
            //     "index : {}, category : {},  c: {}, m1 = {}, c_y: {}, y: {}",
            //     index, category, c, m1, c_y, y
            // );

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
    });

    workbook.close().unwrap();

    let _ = std::fs::remove_dir_all(TMP_DIR);
}

pub fn file_exist(path: &str) -> bool {
    std::fs::metadata(path).is_ok()
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

    let fix_str = str.replace("NaN", "\"\"");
    let p: ADMET = serde_json::from_str(&fix_str)?;
    to_excel(&p, &name);

    info!("完成转换, 输出文件 = {}.xlsx", name);

    Ok(())
}
