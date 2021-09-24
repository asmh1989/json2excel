use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(name = "pub_chems_data")]
pub struct Opt {
    #[structopt(short = "v", long, help = "显示版本")]
    pub version: bool,

    #[structopt(short = "i", help = "输入json文件路径")]
    pub input: Option<String>,
}
