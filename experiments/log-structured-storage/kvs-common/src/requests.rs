use serde_derive::{Deserialize, Serialize};

#[allow(unused)]
#[derive(Serialize, Deserialize, Debug)]
enum Request {
    Put,
    Get,
    Update,
    Delete,
}
