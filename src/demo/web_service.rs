use {
    std::collections::HashMap,
    tracing::info,
    rand::{RngCore, Rng},
};

const PICTURE_SIZE: usize = 8 * 1024 * 1024;

struct DemoWebService {
    users: HashMap<UserId, PictureId>,
    pictures: Vec<Picture>,
}

impl DemoWebService {
    pub fn new(users: HashMap<UserId, PictureId>, pictures: Vec<Picture>) -> Self {
        Self {
            users,
            pictures,
        }
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

/** Inspired by "Web Service Frontend" demo from AIFM. */
pub fn run_web_service_demo() {
    info!("web service demo");

    let pictures = generate_pictures(1000);
    let users = generate_users(100, pictures.len());

    let web_service = DemoWebService::new(users, pictures);
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
