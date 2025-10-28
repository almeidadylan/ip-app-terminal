use std::net::UdpSocket;
use reqwest;

use crate::screen::basic_operations::*;
use crate::screen::read::*;

pub fn show_general_ip_info(){
    clear_screen();
    println!("\n IPv4 local: ................... \n ");
    println!("\n IPv4 publico: ................... \n ");
    println!("\n IPv6 local: ................... \n ");
    println!("\n IPv6 local: ................... \n ");

}


pub fn show_my_local_ip() {
    clear_screen();

    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap(); // conecta a um servidor externo (Google DNS)

    let local_addr = socket.local_addr().unwrap();
    println!("Seu ip local é: {}", local_addr.ip());

    println!("\nPressione qualquer tecla para continuar...");
    let _input = read_string_data();
}


#[tokio::main] 
pub async fn show_my_public_ip () {
    clear_screen();

    // Faz a requisição ao serviço ipify
    let response_ipv4 = reqwest::get("https://api.ipify.org").await;
    //let response_ipv6 = reqwest::get("https://api6.ipify.org").await;
 
    match response_ipv4 {
        Ok(ipv4) => println!("Seu ip público IPv4 é: {}", ipv4.text().await.unwrap()),
        Err(e) => println!("Erro na requisição IPv4: {}", e),
    }

    /*  
    match response_ipv6 {
        Ok(ipv6) => println!("Seu ip público IPv6 é: {:?}", ipv6),
        Err(_) => println!("IPv6 público: não disponível nesta conexão."),
    }
    */

    println!("\nPressione qualquer tecla para continuar...");
    let _input = read_string_data();
}
