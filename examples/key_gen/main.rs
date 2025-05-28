use clap::Parser;
use license_rs::crypto::KeyPair;
use std::error::Error;

/// 密钥生成工具
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// 输出文件路径
    #[arg(short, long)]
    output: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();
    
    // 生成密钥对
    let key_pair = KeyPair::generate()?;
    
    // 格式化输出
    let output_content = format!(
        "Public key:\n{:?}\nSecret key:\n{:?}",
        key_pair.public_key(),
        key_pair.private_key()
    );
    
    // 写入输出文件
    std::fs::write(&args.output, output_content)?;
    
    println!("密钥对已生成并保存到: {}", args.output);
    Ok(())
}