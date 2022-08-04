use std::error::Error;

use log::error;
use lsp_client::init::Connection;

fn main() -> Result<(), Box<dyn Error + Sync + Send>> {
    // init logger
    flexi_logger::Logger::try_with_str("debug")
        .unwrap()
        .start()?;
    let (mut connection, io_threads) = Connection::connect("127.0.0.1:12345").unwrap();

    connection.initialize_start();
    connection.initialize_finish()?;
    main_loop(&connection)?;
    io_threads.join()?;

    // Shut down gracefully.
    eprintln!("shutting down server");
    Ok(())
}

fn main_loop(connection: &Connection) -> Result<(), Box<dyn Error + Sync + Send>> {
    // read request from stdin and send it to the server
    let stdin = std::io::stdin();
    // stdin parse
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line)?;
        match line.trim() {
            "gotodef" => {
                // send request to server
                todo!("send request to server");
            }
            _ => {
                error!("invalid input");
            }
        }
        // send done, wait for response
        let response = connection.receiver.recv().unwrap();
    }
}
