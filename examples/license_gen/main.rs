use clap::Parser;
use license_rs::{crypto::KeyPair, License};
use serde::{Deserialize, Serialize};
use std::{error::Error, fs, path::PathBuf};
use chrono::DateTime;

/// 许可证配置
#[derive(Debug, Serialize, Deserialize)]
struct LicenseConfig {
    customer_id: String,
    product_id: String,
    expiry_date: String, // RFC3339格式
    features: Vec<String>,
    machine_id: Option<String>,
}

/// 许可证生成工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 输入JSON配置文件
    #[arg(short, long)]
    config: PathBuf,
    
    /// 私钥文件路径
    #[arg(short, long)]
    private_key: PathBuf,
    
    /// 输出许可证文件路径
    #[arg(short, long)]
    output: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    // 读取配置文件
    let config = fs::read_to_string(&args.config)?;
    let license_config: LicenseConfig = serde_json::from_str(&config)?;
    
    // 解析过期时间
    let expiry_date = DateTime::parse_from_rfc3339(&license_config.expiry_date)?;
    
    // 创建许可证
    let mut license = License::new(
        license_config.customer_id,
        license_config.product_id,
        expiry_date.into(),
        license_config.features,
        license_config.machine_id,
    );
    
    // 读取私钥
    let private_key = fs::read_to_string(&args.private_key)?;
    let key_pair = KeyPair::from_private_key(private_key.trim().as_bytes())?;
    
    // 签名许可证
    license.sign(&key_pair)?;
    
    // 序列化并写入输出文件
    let license_data = license.to_string()?;
    fs::write(&args.output, license_data)?;
    
    println!("许可证已生成并保存到: {}", args.output.display());
    Ok(())
}