use yggdrasil_shared::crypto::{DecryptedAPIObject, EncryptedAPIObject};
use yggdrasil_shared::models::*;
use rocket_contrib::JSON;

#[post("/serverStatus", data="<server_info>")]
fn get_server_info(server_info: DecryptedAPIObject<ServerInfo>) -> JSON<EncryptedAPIObject> {
  unimplemented!()
}