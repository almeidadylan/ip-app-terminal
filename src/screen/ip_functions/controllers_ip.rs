use std::net::IpAddr;
use std::net::UdpSocket;
use reqwest;


pub fn get_my_local_ip() -> Result<IpAddr, reqwest::Error>{
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    socket.connect("8.8.8.8:80").unwrap(); // conecta a um servidor externo (Google DNS)

    let local_addr = socket.local_addr().unwrap();
    Ok(local_addr.ip())
}


pub async fn get_my_public_ip () -> Result<String, reqwest::Error> {

    let response_ipv4 = reqwest::get("https://api.ipify.org").await?;
    let return_ipv4 = response_ipv4.text().await?;
 
    Ok(return_ipv4)

}
