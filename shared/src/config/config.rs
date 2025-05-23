/// file extention
pub const FILE_EXTENTION: &str = "WR";

/// the max number of files that can be decrypted normally
pub const MAX_DECRYPTABLE_FILES: usize = 6;

/// the files must have these extentions to be decrypted normally
pub const WHITELISTED_EXTENTIONS: [&str; 8] = ["png", "jpg", "jpeg", "txt", "pdf", "docx", "xlsx", "csv"];

/// the folders that will be ignored
pub const BLACKLISTED_FOLDERS: [&str; 1] = ["node_modules"];

/// the max file size allowed to be decrypted normally
pub const MAX_FILE_SIZE_MB: u64 = 30;

/// the min file size so we dont get extreamly small files
pub const MIN_FILE_SIZE_MB: u64 = 10;