use crate::screen::basic_operations::*;
use crate::screen::read::read_string_data;

pub fn get_my_local_ip() {
    clear_screen();
    let my_local_ip = local_ip_address::local_ip().unwrap();
    println!("Seu ip local é: {}", my_local_ip);
    let _input = read_string_data();
}