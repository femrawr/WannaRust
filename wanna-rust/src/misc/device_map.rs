/// wannarust should not run on these devices
static WHITELISTED: &[&str] = &[
    "0000000000000000000000000000000000000000000000000000000000000000",
    "4ba2acc075ed1ad28e1fc0ece2b57b2230a7e815538332c60f10b5e4673e4a27",
];

/// instantly these devices, no chance of recovery
static BLACKLISTED: &[&str] = &[
    "-1",
    "-2",
    "-3",
];

pub fn get_whitelisted_devices() -> Vec<&'static str> {
    WHITELISTED.to_vec()
}

pub fn get_blacklisted_devices() -> Vec<&'static str> {
    BLACKLISTED.to_vec()
}