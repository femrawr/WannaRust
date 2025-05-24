use serde::Deserialize;

use std::{error::Error, fmt};
use sha2::{digest::Update, Digest, Sha256};
use wmi::{COMLibrary, WMIConnection};

#[derive(Debug)]
pub enum DeviceError {
    ConnectionFailed,
    IdentifiersEmpty,
    QueryFailed,
}

impl fmt::Display for DeviceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DeviceError::ConnectionFailed => write!(f, "Failed to connect to WMI"),
            DeviceError::IdentifiersEmpty => write!(f, "Identifire is empty"),
            DeviceError::QueryFailed => write!(f, "WMI query failed"),
        }
    }
}

impl Error for DeviceError {}

type InfoResult = Result<Option<String>, DeviceError>;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct BIOS {
    serial_number: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct BaseBoard {
    serial_number: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ComputerSystemProduct {
    #[serde(rename = "UUID")]
    uuid: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct DiskDrive {
    serial_number: Option<String>,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct UserAccount {
    name: Option<String>,
    #[serde(rename = "SID")]
    sid: Option<String>,
}

pub fn get_identifier() -> Result<String, DeviceError> {
    let com_lib: COMLibrary = COMLibrary::new()
        .map_err(|_| DeviceError::ConnectionFailed)?;

    let wmi_con: WMIConnection = WMIConnection::new(com_lib.into())
        .map_err(|_| DeviceError::ConnectionFailed)?;

    let mut parts: Vec<String> = Vec::new();

    if let Some(bios) = get_bios(&wmi_con)? {
        parts.push(bios);
    }

    if let Some(board) = get_baseboard(&wmi_con)? {
        parts.push(board);
    }

    if let Some(uuid) = get_product(&wmi_con)? {
        parts.push(uuid);
    }

    if let Some(disk) = get_disk(&wmi_con)? {
        parts.push(disk);
    }

    if let Some(sid) = get_machine_sid(&wmi_con)? {
        parts.push(sid);
    }

    if parts.is_empty() {
        return Err(DeviceError::IdentifiersEmpty);
    }

    let joined: String = parts.join("");

    let mut sha = Sha256::default();
    Update::update(&mut sha, joined.as_bytes());

    let hash = sha.finalize();
    Ok(hex::encode(hash))
}

fn get_bios(wmi: &WMIConnection) -> InfoResult {
    let res: Vec<BIOS> = wmi.raw_query("SELECT SerialNumber FROM Win32_BIOS")
        .map_err(|_| DeviceError::QueryFailed)?;

    Ok(res
        .get(0)
        .and_then(|b| b.serial_number
            .clone()
            .map(|s|
                s.trim()
                .to_owned()
            ).filter(|s| !s.is_empty())
        )
    )
}

fn get_baseboard(wmi: &WMIConnection) -> InfoResult {
    let res: Vec<BaseBoard> = wmi.raw_query("SELECT SerialNumber FROM Win32_BaseBoard")
        .map_err(|_| DeviceError::QueryFailed)?;

    Ok(res
        .get(0)
        .and_then(|b| b.serial_number
            .clone()
            .map(|s|
                s.trim()
                .to_owned()
            ).filter(|s| !s.is_empty())
        )
    )
}

fn get_product(wmi: &WMIConnection) -> InfoResult {
    let res: Vec<ComputerSystemProduct> = wmi.raw_query("SELECT UUID FROM Win32_ComputerSystemProduct")
        .map_err(|_| DeviceError::QueryFailed)?;

    Ok(res
        .get(0)
        .and_then(|s| s.uuid
            .clone()
            .map(|u|
                u.trim()
                .to_owned()
            ).filter(|u| !u.is_empty())
        )
    )
}

fn get_disk(wmi: &WMIConnection) -> InfoResult {
    let res: Vec<DiskDrive> = wmi.raw_query("SELECT SerialNumber FROM Win32_DiskDrive WHERE MediaType='Fixed hard disk media'")
        .map_err(|_| DeviceError::QueryFailed)?;

    for disk in res.iter().take(1) {
        if let Some(serial) = &disk.serial_number {
            let trimmed = serial.trim().to_owned();
            if !trimmed.is_empty() {
                return Ok(Some(trimmed));
            }
        }
    }

    Ok(None)
}

fn get_machine_sid(wmi: &WMIConnection) -> InfoResult {
    let res: Vec<UserAccount> = wmi.raw_query("SELECT Name, SID FROM Win32_UserAccount WHERE Name='SYSTEM'")
        .map_err(|_| DeviceError::QueryFailed)?;

    for acc in res {
        if acc.name.as_deref() == Some("SYSTEM") {
            return Ok(acc.sid.map(|s|
                s.trim()
                .to_owned()
            ).filter(|s| !s.is_empty()));
        }
    }

    Ok(None)
}