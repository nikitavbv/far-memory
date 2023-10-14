use {
    std::net::TcpListener,
    tracing::info,
    self::protocol::StorageRequest,
};

mod protocol;

pub fn run_storage_server() {
    info!("running storage server");

    let listener = TcpListener::bind("0.0.0.0:14000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        info!("handling incoming connection");
        let res: StorageRequest = bincode::deserialize_from(&stream).unwrap();

        info!("got request: {:?}", res);
    }
}

#[cfg(test)]
mod tests {
    use {
        super::*,
    };

    #[test]
    fn simple() {
        // TODO: think how a server can be tested
    }
}