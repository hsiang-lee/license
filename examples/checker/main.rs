use clap::Parser;
use license_rs::{crypto::KeyPair, License};
use std::{error::Error, fs, path::PathBuf};

/// 许可证验证工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 许可证文件路径
    #[arg(short, long)]
    license: PathBuf,

    /// 公钥文件路径
    #[arg(short, long)]
    public_key: PathBuf,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    // 读取许可证文件
    let license_data = fs::read_to_string(&args.license)?;
    
    // 解析许可证
    let license = License::from_str(&license_data)?;
    
    // 读取私钥文件(包含公钥)
    let key_data = fs::read_to_string(&args.public_key)?;
    let key_pair = KeyPair::from_private_key(key_data.trim().as_bytes())?;

    // 验证许可证
    match license.validate(key_pair.public_key()) {
        Ok(_) => {
            println!("✅ 许可证验证通过");
            println!("许可证信息:");
            println!("- 客户: {}", license.customer_id);
            println!("- 产品: {}", license.product_id);
            println!("- 有效期至: {}", license.expiry_date);
            println!("- 功能: {}", license.features.join(", "));
            if let Some(machine_id) = license.machine_id {
                println!("- 机器ID: {}", machine_id);
            }
            Ok(())
        }
        Err(e) => {
            eprintln!("❌ 许可证验证失败: {}", e);
            Err(e.into())
        }
    }
}