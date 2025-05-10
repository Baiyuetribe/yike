use rodio::{Decoder, OutputStream, Sink, Source};
use std::fs::File;
use std::io::Cursor;

const MP3_DATA: &'static [u8] = include_bytes!("waves.mp3");

#[tauri::command]
pub fn play_audio() {
    // 使用 std::thread 在新线程中播放音频，避免阻塞主线程
    std::thread::spawn(|| {
        let _ = play_audio_internal();
    });
}

// 内部函数处理具体播放逻辑
fn play_audio_internal() -> Result<(), String> {
    // 1. 获取输出流
    let (_stream, stream_handle) =
        OutputStream::try_default().map_err(|e| format!("无法获取音频输出流: {}", e))?;

    // 2. 创建Sink
    let sink = Sink::try_new(&stream_handle).map_err(|e| format!("无法创建音频Sink: {}", e))?;

    // 3. 解码音频
    let source = Decoder::new(Cursor::new(MP3_DATA)).map_err(|e| format!("音频解码失败: {}", e))?;

    // 4. 播放
    sink.append(source);
    sink.sleep_until_end();

    Ok(())
}
