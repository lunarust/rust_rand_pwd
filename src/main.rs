use std::process::Command;
use rand::Rng;
//use std::io::{self, Write};

fn main() {
    println!("Hi there!");
    println!("-------------------");
    for n in 1..11 {
    println!("{:2}. {}", n, generate_random_serie());
    }
    println!("-------------------");
    println!("~ Bye now ~~  (ᵔ◡ᵔ)/");
}
fn generate_random_serie() -> String {

    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                      abcdefghijklmnopqrstuvwxyz\
                      0123456789^%$#";
    const PASSWORD_LEN: usize = 15;
    let mut rng = rand::thread_rng();

    let password: String = (0..PASSWORD_LEN)
    .map(|_| {
      let idx = rng.gen_range(0..CHARSET.len());
      CHARSET[idx] as char
    })
    .collect();

    let mycmd = format!("echo {} | openssl passwd -1 -stdin", password);
    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
                .args(["/C", "echo hello"])
                .output()
                .expect("failed to execute process")
    } else {
        Command::new("sh")
                .arg("-c")
                .arg(mycmd)
                .output()
                .expect("failed to execute process")
    };
    let hashedit = String::from_utf8_lossy(&output.stdout);
    //println!("status: {:>50}", output.status);
    //println!("stdout: {:>50}", hashedit);
    //println!("stderr: {:>50}", String::from_utf8_lossy(&output.stderr));

    return format!("{} - {}", password, hashedit);
}