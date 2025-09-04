use std::io;

fn read_string_data () -> String {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Falha ao ler dados");
    data.trim().to_string()
}

pub fn read_int_data () -> u32 {
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Falha ao ler dados");
    data.trim().parse().expect("Erro ao converter dados para inteiro")
}