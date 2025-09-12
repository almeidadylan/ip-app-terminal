use std::net::UdpSocket;

use crate::screen::basic_operations::*;
use crate::screen::read::read_string_data;

pub fn get_my_local_ip() {
    clear_screen();
    let my_local_ip = local_ip_address::local_ip().unwrap();
    println!("Seu ip local é: {}", my_local_ip);

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap(); // conecta a um servidor externo (Google DNS)

    let local_addr = socket.local_addr().unwrap();
    println!("Seu ip local é: {}", local_addr.ip());



    let _input = read_string_data();


}

fn _get_my_local_ip2() {
    //clear_screen();
    // Criar um socket UDP sem realmente enviar nada
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap(); // conecta a um servidor externo (Google DNS)

    let local_addr = socket.local_addr().unwrap();
    println!("Seu ip local é: {}", local_addr.ip());
}