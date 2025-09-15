use crate::screen::{
    read::*,
    basic_operations::*,
    ip::*
};
pub fn menu () {
    loop {
        clear_screen();
        println!("\
        ========== Menu ========== \n\
        Escolha uma das opções abaixo:\n\n\
            1 - Descobrir o meu IP local e publico\n\
            2 - Descobrir meu ip\n\
            3 - Lista os IPs da rede 2\n\
            4 - Opção 4\n\
            5 - Opção 5\n\
            0 - Sair do Programa\n\
        ");

        let input = read_int_data();

        match input {
            0 => return,
            1 => { get_my_local_ip(); get_my_public_ip(); wait(3); }, 
            2 => println!("Você escolheu a opção 2"),
            3 => println!("Você escolheu a opção 3"),
            4 => println!("Você escolheu a opção 4"),
            _ => println!("Você escolheu a opção _"),
        }
    }
}