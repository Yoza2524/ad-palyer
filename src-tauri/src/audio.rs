use rodio::{Decoder, OutputStream, Sink}; // https://docs.rs/rodio/latest/rodio/#feature-playback
use std::fs;

// 音频对象结构体
// pub struct AudioState {
//     pub _stream: OutputStream,
//     pub sink: Sink,
// }

// 全局可访问的音频状态
// pub static AUDIO_STATE: Lazy<Arc<Mutex<Option<AudioState>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

// TODO: 
// 1. 文件可用性校验
// 2. 播放/暂停/进度条/音量调节

// 获取系统默认声音设备的流(stream_handle) 和 音轨(sink)
fn init_handle() -> (OutputStream, Sink) {
    let stream_handle = rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    let sink = rodio::Sink::connect_new(&stream_handle.mixer());

    (stream_handle, sink)
}

// Tauri command: 播放指定路径的音频
#[tauri::command]
pub fn play_audio(_path: String) -> Result<(), String> {
    let (stream_handle, _sink) = init_handle();
    // 获取文件源
    let file = fs::File::open("../public/testAduio00.mp3").unwrap();
    let source = Decoder::try_from(file).unwrap();

    stream_handle.mixer().add(source);

    // 阻塞函数防止被销毁
    std::thread::sleep(std::time::Duration::from_secs(5));

    Ok(())
}