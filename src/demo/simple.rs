use {
    tracing::info,
    crate::client::{FarMemoryClient, FarMemoryVec, backend::disk::LocalDiskBackend},
};

struct SomeApplicationData {
}

impl SomeApplicationData {
    pub fn new() -> Self {
        Self {
        }
    }

    pub fn do_something(&self) {
    }
}

pub fn run_simple_demo() {
    info!("running a simple demo");

    let client = FarMemoryClient::new(Box::new(LocalDiskBackend::new()), 1000);
    let vec = FarMemoryVec::from_vec(client, vec![10.02, 9.02, 8.02, 7.02, 6.02, 5.02, 4.02, 3.02, 2.02, 1.02]);

    let res = vec.to_local_vec();
    println!("res: {:?}", res);

    let token = "";

    // create a client when software is initilized:
    let client = FarMemoryClient::connect_to("192.168.254.30:14000", token).unwrap();

    let object = SomeApplicationData::new();
    object.do_something();

    let object = client.object(object);
    object.do_something();

    client.stop();
}
