fn main() {
    use ssh2::Session;
    use std::io::prelude::*;
    use std::net::TcpStream;

    let command_to_run = std::env::args().nth(1).unwrap();
    // Connect to the local SSH server
    let ts_netops_host = std::env::var("TS_NETOPS_HOST").unwrap();
    let tcp_target = format!("{}:22", ts_netops_host);
    let tcp = TcpStream::connect(tcp_target).unwrap();
    let mut sess = Session::new().unwrap();
    sess.set_tcp_stream(tcp);
    sess.handshake().unwrap();
    let ts_netops_user = std::env::var("TS_NETOPS_USER").unwrap();
    let ts_netops_pass = std::env::var("TS_NETOPS_PASS").unwrap();
    sess.userauth_password(&ts_netops_user, &ts_netops_pass).unwrap();

    let mut channel = sess.channel_session().unwrap();
    channel.exec(&command_to_run).unwrap();
    let mut s = String::new();
    let mut result = channel.read_to_string(&mut s);
    let mut retry_count = 10;
    while result.is_err() && retry_count > 0 {
        retry_count = retry_count - 1;
        eprintln!("Retry. Remaining: {}", retry_count);
        result = channel.read_to_string(&mut s);
    }
    println!("{}", s);
    channel.wait_close();
    println!("{}", channel.exit_status().unwrap());
}
