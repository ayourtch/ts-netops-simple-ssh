fn main() {
    use ssh2::Session;
    use std::io::prelude::*;
    use std::net::TcpStream;

    // Connect to the local SSH server
    let tcp = TcpStream::connect("127.0.0.1:22").unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    let ts_netops_user = std::env::var("TS_NETOPS_USER").unwrap();
    let ts_netops_pass = std::env::var("TS_NETOPS_PASS").unwrap();
    sess.userauth_password(&ts_netops_user, &ts_netops_pass).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec("ls").unwrap();
    let mut s = String::new();
    channel.read_to_string(&mut s).unwrap();
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}
