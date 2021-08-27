#![allow(dead_code)]
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::{Result, Value};
use structopt::StructOpt;
use xlsxwriter::{FormatAlignment, FormatColor, Workbook};

use crate::args::Opt;

mod args;
mod config;
mod json;

#[derive(Debug, Serialize, Deserialize)]
struct ADMET {
    input: Vec<Value>,
    output: Vec<Vec<Value>>,
}

fn to_excel(p: &ADMET, name: &str) {
    let workbook = Workbook::new(&format!("{}.xlsx", name));

    let format = workbook
        .add_format()
        .set_font_color(FormatColor::Green)
        .set_align(FormatAlignment::CenterAcross)
        .set_align(FormatAlignment::VerticalCenter);

    let format1 = workbook.add_format().set_align(FormatAlignment::Center);

    p.output.iter().for_each(|f| {
        let mut sheet = workbook.add_worksheet(None).unwrap();
        let mut y = 0;
        sheet.write_string(0, 0, "序号", Some(&format1)).unwrap();
        sheet.write_string(0, 1, "目标分类", Some(&format)).unwrap();
        sheet.write_string(0, 2, "目标名字", None).unwrap();
        sheet.write_string(0, 3, "值 (单位)", None).unwrap();
        sheet.set_column(0, 0, 6., None).unwrap();
        sheet.set_column(1, 1, 20., None).unwrap();
        sheet.set_column(2, 2, 60., None).unwrap();
        sheet.set_column(3, 3, 30., None).unwrap();

        y += 1;

        let mut c_y = 1;
        let mut category = "";
        let mut num = 0;
        let mut first = true;
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
                .write_string(y, 3, &format!("{} {}", v1.as_str(), v2), None)
                .unwrap();

            // info!(
            //     "index : {}, category : {}, c: {}, num: {}, c_y: {}, y: {}",
            //     index, category, c, num, c_y, y
            // );

            if c != category {
                if !category.is_empty() {
                    // 写入合并列
                    if num > 0 {
                        // info!("unit ... {:?}, {}", (c_y, y), category);
                        sheet
                            .merge_range(c_y, 1, y, 1, category, Some(&format))
                            .unwrap();
                    } else {
                        // info!("wrirte 1 ... {:?}, {}", c_y, category);
                        sheet.write_string(c_y, 1, category, Some(&format)).unwrap();
                    }
                    if first {
                        c_y = y;
                        first = false;
                    } else {
                        c_y = y + 1;
                    }
                }

                num = 0;
                category = c;
            } else {
                num += 1;
            }

            y += 1;
        });
    });

    workbook.close().unwrap();
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

    let path = opt.input;

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

    info!("完成转换, 输出文件 = {}.json", name);

    Ok(())
}
