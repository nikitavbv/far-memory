use {
    std::{collections::HashMap, io::Write},
    tracing::info,
    rand::{RngCore, Rng, rngs::OsRng},
    aes_gcm::{aead::{KeyInit, Aead, AeadCore}, Aes256Gcm, Key},
};

const PICTURE_SIZE: usize = 8 * 1024 * 1024;

struct DemoWebService {
    users: HashMap<UserId, PictureId>,
    pictures: Vec<Picture>,

    cipher: Aes256Gcm,
}

impl DemoWebService {
    pub fn new(users: HashMap<UserId, PictureId>, pictures: Vec<Picture>) -> Self {
        Self {
            users,
            pictures,

            cipher: Aes256Gcm::new(&Aes256Gcm::generate_key(OsRng)), // AIFM uses AES-CBC, but it doesn't matter for evaluation performance.
        }
    }

    pub fn handle_request(&self, request: WebServiceRequest) -> WebServiceResponse {
        let picture_to_get: u64 = request.user_ids.iter()
            .map(|id| self.users.get(id).unwrap().picture_id)
            .sum();
        let picture_to_get = picture_to_get % self.pictures.len() as u64; // TODO: zipf distribution

        let picture = &self.pictures.get(picture_to_get as usize).unwrap().picture_data;
        let encrypted_picture = self.encrypt_picture(picture);

        // yes, encrpyted data cannot be compressed, but it still a good way to simulate CPU load. AIFM does the same for their evaluation.
        let compressed_picture = self.compress_picture(&encrypted_picture);

        WebServiceResponse::new(compressed_picture)
    }

    fn encrypt_picture(&self, picture: &[u8]) -> Vec<u8> {
        let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
        assert_eq!(12, nonce.len());

        let encrypted = self.cipher.encrypt(&nonce, picture).unwrap();
        {
            let mut encrypted = encrypted;
            let mut result = nonce.to_vec();
            result.append(&mut encrypted);
            result
        }
    }

    fn compress_picture(&self, picture: &[u8]) -> Vec<u8> {
        let mut output = Vec::new();
        snap::write::FrameEncoder::new(&mut output).write_all(picture).unwrap();
        output
    }
}

#[derive(Eq, PartialEq, Hash, Debug)]
struct UserId {
    id: u64,
}

impl UserId {
    pub fn new(id: u64) -> Self {
        Self {
            id,
        }
    }
}

#[derive(Debug)]
struct PictureId {
    picture_id: u64,
}

impl PictureId {
    pub fn new(picture_id: u64) -> Self {
        Self {
            picture_id,
        }
    }
}

#[derive(Debug)]
struct Picture {
    picture_data: Vec<u8>,
}

impl Picture {
    pub fn new(picture_data: Vec<u8>) -> Self {
        Self {
            picture_data,
        }
    }
}

struct WebServiceRequest {
    user_ids: Vec<UserId>,
}

impl WebServiceRequest {
    pub fn new(user_ids: Vec<UserId>) -> Self {
        Self {
            user_ids,
        }
    }
}

#[derive(Debug)]
struct WebServiceResponse {
    data: Vec<u8>,
}

impl WebServiceResponse {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            data,
        }
    }
}

/**
Inspired by "Web Service Frontend" demo from AIFM.

This example is not intended to compare performance against AIFM, it is just their demo app is a good
example software to integrate to.
*/
pub fn run_web_service_demo() {
    info!("web service demo");

    let pictures = generate_pictures(1000);
    info!("finished generating pictures");

    let total_users = 100;
    let users = generate_users(total_users, pictures.len());
    info!("finished generating users");

    let web_service = DemoWebService::new(users, pictures);

    let request = random_request(total_users);
    let response = web_service.handle_request(request);

}

fn random_request(total_users: usize) -> WebServiceRequest {
    WebServiceRequest::new((0..32).map(|_| generate_user_id(total_users)).collect())
}

fn generate_user_id(total_users: usize) -> UserId {
    UserId::new(rand::thread_rng().gen_range(0..total_users as u64))
}

fn generate_users(total_users: usize, total_pictures: usize) -> HashMap<UserId, PictureId> {
    (0..total_users)
        .into_iter()
        .map(|user_id| (UserId::new(user_id as u64), PictureId::new(pick_picture_for_user(total_pictures))))
        .collect()
}

fn pick_picture_for_user(total_pictures: usize) -> u64 {
    rand::thread_rng().gen_range((0..total_pictures as u64))
}

fn generate_pictures(total_pictures: usize) -> Vec<Picture> {
    (0..total_pictures)
        .into_iter()
        .map(|_| generate_picture())
        .collect()
}

fn generate_picture() -> Picture {
    let mut data = vec![0; PICTURE_SIZE];
    rand::thread_rng().fill_bytes(&mut data);

    Picture::new(data)
}
