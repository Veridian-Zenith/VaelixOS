mod kernel;
mod graphics;
mod ui;
mod package;
mod networking;
mod boot;

use boot::vaeboot;

fn main() {
    println!("Welcome to VaelixOS!");
    vaeboot::boot().expect("Failed to boot");
}
