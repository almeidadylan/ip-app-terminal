use std::net::UdpSocket;

use crate::screen::basic_operations::*;
use crate::screen::ip_functions::controllers_ip::get_my_public_ip;
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


pub async fn show_my_public_ip () {
    clear_screen();

    let my_ip = get_my_public_ip().await;
 
    match my_ip {
        Ok(ipv4) => println!("Seu ip IPv4 público é: {:?}", ipv4),
        Err(e) => println!("Erro na requisição IPv4: {}", e),
    }

    println!("\nPressione qualquer tecla para continuar...");
    let _input = read_string_data();
}
