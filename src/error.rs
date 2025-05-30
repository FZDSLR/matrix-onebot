/// https://github.com/onebot-walle/walle-q/blob/master/src/error.rs
use walle_core::resp::{resp_error, RespError};
use walle_core::{error_type, WalleError};

error_type!(bad_param, 10003, "参数错误");
error_type!(empty_message, 10003, "消息为空");
error_type!(unsupported_param, 10004, "不支持的参数");
error_type!(client_not_initialized, 20001, "客户端未初始化");
error_type!(prepare_file_first, 20003, "请先 Prepare 再上传后续文件");
error_type!(file_total_size_not_match, 20004, "文件大小不匹配");
error_type!(file_sha256_not_match, 20005, "文件sha256不匹配");
error_type!(file_open_error, 32001, "文件打开失败");
error_type!(file_read_error, 32002, "文件读取失败");
error_type!(file_create_error, 32003, "文件创建失败");
error_type!(file_write_error, 32004, "文件写入失败");
error_type!(file_not_found, 32005, "文件不存在");
error_type!(file_type_not_match, 32006, "文件类型不匹配");
error_type!(net_download_fail, 33001, "网络下载失败");
error_type!(matrix_client_error, 34001, "matrix client错误");
error_type!(login_failed, 34002, "login failed");
error_type!(risk_controlled, 34003, "可能被风控");
error_type!(message_not_exist, 35001, "消息不存在");
error_type!(friend_not_exist, 35002, "好友不存在");
error_type!(group_not_exist, 35003, "群不存在");
error_type!(group_member_not_exist, 35004, "群成员不存在");
error_type!(permission_denied, 35005, "权限不足");
error_type!(image_info_decode_error, 61001, "图片解码错误");
error_type!(bad_image_url, 61002, "图片URL错误");
error_type!(bad_image_path, 61003, "图片路径错误");
error_type!(bad_image_data, 61004, "图片内容错误");
error_type!(audio_encode_failed, 61005, "音频编码失败");
error_type!(silk_encode_failed, 61005, "silk编码失败");

pub fn map_action_parse_error(error: WalleError) -> RespError {
    match error {
        WalleError::DeclareNotMatch(_, get) => resp_error::unsupported_action(get),
        WalleError::MapMissedKey(expect) => {
            resp_error::bad_param(format!("missing key {}", expect))
        }
        e => {
            tracing::warn!(target: "matrix-onebot", "{}", e);
            resp_error::bad_handler(e.to_string())
        }
    }
}