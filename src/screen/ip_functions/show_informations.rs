use crate::screen::basic_operations::*;
use crate::screen::ip_functions::controllers_ip::{get_my_local_ipv4, get_my_public_ipv4};
use crate::screen::read::*;

pub async fn show_general_ip_info(){
    clear_screen();
    println!("\n IPv4 local: ..................... {}  ", get_my_local_ipv4().unwrap());
    println!("\n IPv4 publico: ................... {}  ", get_my_public_ipv4().await.unwrap());
    println!("\n IPv6 local: .....................  ");
    println!("\n IPv6 local: .....................  ");

    println!("\nPressione qualquer tecla para continuar...");
    let _input = read_string_data();
}


pub fn show_my_local_ipv4() {
    clear_screen();

    let my_ipv4 = get_my_local_ipv4();

    match my_ipv4 {
        Ok(ipv4) => println!("Seu ip IPv4 local é: {:?}", ipv4),
        Err(e) => println!("Erro ao obter o IP local: {}", e),
    }

    println!("\nPressione qualquer tecla para continuar...");
    let _input = read_string_data();
}


pub async fn show_my_public_ipv4 () {
    clear_screen();

    let my_ip = get_my_public_ipv4().await;
 
    match my_ip {
        Ok(ipv4) => println!("Seu ip IPv4 público é: {:?}", ipv4),
        Err(e) => println!("Erro na requisição IPv4: {}", e),
    }

    println!("\nPressione qualquer tecla para continuar...");
    let _input = read_string_data();
}
