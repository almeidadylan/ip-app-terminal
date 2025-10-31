use crate::screen::{
    read::*,
    basic_operations::*,
    ip_functions::{
        show_informations::*,
    }
};

#[tokio::main] 
pub async fn menu () {
    loop {
        clear_screen();
        println!("\
        ========== Menu ========== \n\
        Escolha uma das opções abaixo:\n\n\
            1 - Mostrar informações sobre meu ip \n\
            2 - Descobrir o meu IP Local\n\
            3 - Descobrir meu IP Público\n\
            4 - Lista os IPs da rede 2\n\
            5 - Opção 4\n\
            6 - Opção 5\n\
            0 - Sair do Programa\n\
        ");

        let input = read_int_data();

        match input {
            0 => return,
            1 => show_general_ip_info().await,
            2 => show_my_local_ipv4(), 
            3 => show_my_public_ipv4().await,
            4 => println!("Você escolheu a opção 3"),
            5 => println!("Você escolheu a opção 4"),
            6 => println!("Você escolheu a opção 5"),
            _ => println!("Você escolheu a opção _"),
        }
    }
}