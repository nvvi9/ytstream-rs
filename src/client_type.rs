pub enum ClientType {
    Android,
    Embedded,
}

impl ClientType {
    pub fn name(&self) -> &str {
        match self {
            ClientType::Android => "ANDROID",
            ClientType::Embedded => "WEB_EMBEDDED_PLAYER",
        }
    }

    pub fn version(&self) -> &str {
        match self {
            ClientType::Android => "17.31.35",
            ClientType::Embedded => "1.19700101",
        }
    }

    pub fn key(&self) -> &str {
        match self {
            ClientType::Android => "AIzaSyA8eiZmM1FaDVjRy-df2KTyQ_vz_yYM39w",
            ClientType::Embedded => "AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8",
        }
    }

    pub fn user_agent(&self) -> &str {
        match self {
            ClientType::Android => "com.google.android.youtube/17.31.35 (Linux; U; Android 11) gzip",
            ClientType::Embedded => "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/91.0.4472.124 Safari/537.36",
        }
    }

    pub fn android_sdk_version(&self) -> Option<u32> {
        match self {
            ClientType::Android => Some(30),
            ClientType::Embedded => None,
        }
    }
}
