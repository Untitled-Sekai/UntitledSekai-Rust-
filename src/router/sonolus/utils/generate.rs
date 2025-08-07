use std::time::{SystemTime, UNIX_EPOCH};

// セッション情報を格納する構造体
#[derive(Debug, Clone)]
pub struct SessionInfo {
    pub session: String,
    pub expiration: u64,
}

// セッションの生成
pub fn generate_session() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();

    let mut hasher = DefaultHasher::new();
    now.hash(&mut hasher);

    let additional_entropy = std::process::id() as u128;
    additional_entropy.hash(&mut hasher);

    format!("{:x}", hasher.finish())
}

pub fn generate_secure_session() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    
    let process_id = std::process::id() as u128;
    let combined = format!("{}{}", now, process_id);
    
    let bytes = combined.as_bytes();
    let mut result = String::new();
    
    for chunk in bytes.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &b) in chunk.iter().enumerate() {
            buf[i] = b;
        }
        
        let combined = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);
        
        result.push_str(&format!("{:06x}", combined));
    }
    
    result
}

// 有効期限
pub fn generate_expiration(minutes: u64) -> u64 {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    now + (minutes * 60)
}

pub fn generate_default_expiration() -> u64 {
    generate_expiration(24 * 60) // 24時間
}

pub fn generate_session_info(expiration_minutes: Option<u64>) -> SessionInfo {
    let session = generate_secure_session();
    let expiration = match expiration_minutes {
        Some(minutes) => generate_expiration(minutes),
        None => generate_default_expiration(),
    };
    
    SessionInfo {
        session,
        expiration,
    }
}

pub fn is_session_valid(expiration: u64) -> bool {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    now < expiration
}