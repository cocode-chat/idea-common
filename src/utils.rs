// 定义全局 Snowflake 变量
use rustflake::Snowflake;
use lazy_static::lazy_static;
lazy_static! {
    static ref GLOBAL_SNOWFLAKE: std::sync::Mutex<Snowflake> = std::sync::Mutex::new(Snowflake::new(1420070400000, 1, 1));
}

/// 生成一个全局唯一的ID (基于Snowflake算法)
///
/// # 返回
/// 返回一个u64类型的唯一ID
pub fn get_next_id() -> i64 {
    GLOBAL_SNOWFLAKE.lock().unwrap().generate()
}


/// 将字节向量编码为Base64字符串
///
/// # 参数
/// * `bytes` - 需要编码的字节向量
///
/// # 返回
/// 返回Base64编码后的字符串
use base64::{Engine as _, engine::general_purpose};
pub fn base64_encode(bytes: Vec<u8>) -> String {
    general_purpose::STANDARD.encode(bytes)
}