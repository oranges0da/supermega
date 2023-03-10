use::colored::Colorize;
use std::time::Duration;
use std::thread::sleep;

// print art
pub fn draw() {
    let logo1 = r#"    
    .▄▄ · ▄• ▄▌ ▄▄▄·▄▄▄ .▄▄▄  
    ▐█ ▀. █▪██▌▐█ ▄█▀▄.▀·▀▄ █·
    ▄▀▀▀█▄█▌▐█▌ ██▀·▐▀▀▪▄▐▀▀▄ 
    ▐█▄▪▐█▐█▄█▌▐█▪·•▐█▄▄▌▐█•█▌
     ▀▀▀▀  ▀▀▀ .▀    ▀▀▀ .▀  ▀ "#;
    let logo2 = r#"    
    • ▌ ▄ ·. ▄▄▄ . ▄▄ •  ▄▄▄· 
    ·██ ▐███▪▀▄.▀·▐█ ▀ ▪▐█ ▀█ 
    ▐█ ▌▐▌▐█·▐▀▀▪▄▄█ ▀█▄▄█▀▀█ 
    ██ ██▌▐█▌▐█▄▄▌▐█▄▪▐█▐█▪ ▐▌
    ▀▀  █▪▀▀▀ ▀▀▀ ·▀▀▀▀  ▀  ▀ 
    "#;
    for line in logo1.lines() {
        println!("{}", line.bright_red());
        sleep(Duration::from_millis(15));
    }
    for line in logo2.lines() {
        println!("{}", line.blue());
        sleep(Duration::from_millis(15));
    }
}