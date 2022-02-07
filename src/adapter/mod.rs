
pub struct MP4 {
    name: String,
}

pub struct AVI {
    name: String,
}

pub struct MKV {
    name: String,
}

pub struct Webm {
    name: String,
}

pub struct WebVideoPlayer;

impl WebVideoPlayer {
    pub fn play(&self, video: &Webm) {
        println!("Playing {}", video.name);
    }
}

pub trait Adapter {
    fn convert(&self) -> Webm;
}

pub struct MP4Adapter {
    file: MP4,
}

impl Adapter for MP4Adapter {
    fn convert(&self) -> Webm {
        Webm {
            name: format!("{}.webm", self.file.name),
        }
    }
}

pub struct MKVAdapter {
    file: MKV,
}

impl Adapter for MKVAdapter {
    fn convert(&self) -> Webm {
        Webm {
            name: format!("{}.webm", self.file.name),
        }
    }
}

pub struct AVIAdapter {
    file: AVI,
}

impl Adapter for AVIAdapter {
    fn convert(&self) -> Webm {
        Webm {
            name: format!("{}.webm", self.file.name),
        }
    }
}

pub fn example() {
    let mp4_file = MP4 {
        name: "test.MP4".to_string(),
    };

    let avi_file = AVI {
        name: "test.AVI".to_string(),
    };
    
    let mkv_file = MKV {
        name: "test.MKV".to_string(),
    };

    let webm_file = Webm {
        name: "test.WEBM".to_string(),
    };

    let player = WebVideoPlayer{};

    player.play(&webm_file);
    player.play(&MP4Adapter { file: mp4_file }.convert());
    player.play(&MKVAdapter { file: mkv_file }.convert());
    player.play(&AVIAdapter { file: avi_file }.convert());
}