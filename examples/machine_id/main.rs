use clap::Parser;
use license_rs::machine_info;
use std::error::Error;

/// 机器ID生成工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 输出格式: json/text (默认: text)
    #[arg(short, long, default_value = "text")]
    format: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    // 获取机器信息
    let provider = machine_info::create_provider()?;
    let cpu_id = provider.get_cpu_id()?;
    let machine_id = provider.get_machine_id()?;
    
    // 根据格式输出
    match args.format.as_str() {
        "json" => {
            let output = serde_json::json!({
                "machine_id": machine_id,
                "cpu_id": cpu_id
            });
            println!("{}", serde_json::to_string_pretty(&output)?);
        }
        "text" => {
            println!("Machine ID: {}", machine_id);
            println!("CPU ID: {}", cpu_id);
        }
        _ => return Err("无效的输出格式，支持: json/text".into()),
    }
    
    Ok(())
}