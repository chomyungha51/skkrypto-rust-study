use std::io;
use std::io::Read;
use std::io::Write;
use std::fs::File;

fn main() {
    let menu = vec!["1. read", "2. write"];
    for i in &menu {
        println!("{}", i);
    }
    let mut currentMenu = String::new();
    io::stdin().read_line(&mut currentMenu);

    let currentMenu: i32 = currentMenu.trim().parse().expect("type number");
    match currentMenu {
        1 => {
            let mut mnemonic = read_mnemonic().expect("You don't have mnemonic"); 
            println!("Current Mnemonic Words: {}", mnemonic);
        },
        2 => {
            let mut mnemonic = write_mnemonic(); 
            let mnemonic = read_mnemonic().unwrap();
            println!("Current Mnemonic Words: {}",mnemonic);
        },
        _ => {
            println!("not suppported");
        }
    }
}

fn read_mnemonic() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("mnemonic.txt")?.read_to_string(&mut s)?;

    Ok(s)
}

fn write_mnemonic(){
    const new:&str = "New mnemonic";
    let mut f = File::create("mnemonic.txt").expect("already exists");
    writeln!(&mut f, "New mnemonic").expect("file write error");
    f.write(b"Bytes\n").expect("file write error");
}