use std::net::UdpSocket;
use reqwest;

use crate::screen::basic_operations::*;
use crate::screen::read::*;


pub fn get_my_local_ip() {
    clear_screen();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap(); // conecta a um servidor externo (Google DNS)

    let local_addr = socket.local_addr().unwrap();
    println!("Seu ip local é: {}", local_addr.ip());

    println!("\nPressione qualquer tecla para continuar...");
    let _input = read_string_data();
}


#[tokio::main] 
pub async fn get_my_public_ip () -> Result<String, reqwest::Error> {

    let response_ipv4 = reqwest::get("https://api.ipify.org").await?;
    let return_ipv4 = response_ipv4.text().await?;
 
    Ok(return_ipv4)

}
