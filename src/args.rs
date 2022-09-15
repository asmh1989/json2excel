use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pub_chems_data")]
pub struct Opt {
    #[structopt(short = "v", long, help = "显示版本")]
    pub version: bool,

    #[structopt(short = "i", help = "输入json文件路径")]
    pub input: Option<String>,

    #[structopt(long = "actue_herg_file", help = "输入actue_herg预测值路径")]
    pub actue_herg_file: Option<String>,

    #[structopt(long = "id", help = "id文件列表")]
    pub id: Option<String>,

    #[structopt(long = "csv", help = "输出csv文件格式")]
    pub csv: bool,
}
