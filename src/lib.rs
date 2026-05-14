use wasm_bindgen::prelude::*;
use std::io::Cursor;
use lepton_jpeg::{
    decode_lepton, 
    EnabledFeatures, 
    SingleThreadPool,
};

#[cfg(feature = "full")]
use lepton_jpeg::encode_lepton;

/// 将 .lep 文件解码为 .jpg 文件
/// 
/// 这个函数在 viewer 和 full 特性下均可用。
#[cfg(any(feature = "viewer", feature = "full"))]
#[wasm_bindgen]
pub fn lep_to_jpg(lep_data: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut input = Cursor::new(lep_data);
    // 预分配一定的空间以减少扩容开销
    let mut output = Vec::with_capacity(lep_data.len() * 2);

    // 使用兼容性最好的 read 配置
    // 如果编译时开启了 WASM SIMD，底层的 wide crate 会自动加速
    let features = EnabledFeatures::compat_lepton_vector_read();
    
    // Web 环境默认使用单线程池（避免复杂的 Worker 配置）
    let thread_pool = SingleThreadPool {};

    match decode_lepton(&mut input, &mut output, &features, &thread_pool) {
        Ok(_) => Ok(output),
        Err(e) => Err(JsValue::from_str(&format!("Lepton 解码失败: {:?}", e))),
    }
}

/// 将 .jpg 文件编码为 .lep 文件
/// 
/// 仅在开启 full 特性时导出。
#[cfg(feature = "full")]
#[wasm_bindgen]
pub fn jpg_to_lep(jpg_data: &[u8]) -> Result<Vec<u8>, JsValue> {
    let mut input = Cursor::new(jpg_data);
    let mut output_buf = Vec::with_capacity(jpg_data.len());
    let mut output = Cursor::new(&mut output_buf);

    let features = EnabledFeatures::compat_lepton_vector_write();
    let thread_pool = SingleThreadPool {};

    match encode_lepton(&mut input, &mut output, &features, &thread_pool) {
        Ok(_) => Ok(output_buf),
        Err(e) => Err(JsValue::from_str(&format!("Lepton 编码失败: {:?}", e))),
    }
}

/// 获取版本信息
#[wasm_bindgen]
pub fn get_version() -> String {
    lepton_jpeg::get_version_string()
}

/// 自动初始化错误钩子，方便在控制台看到崩溃信息
#[wasm_bindgen(start)]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}
