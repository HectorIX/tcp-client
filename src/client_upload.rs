use std::str;

use user;
use file_io;
use interface;


pub fn upload() -> String {

    let mut full_request = "upload_state::Upload**".to_owned();
    let mut path_to_file = "to_upload/".to_string();

    let filename = interface::read_filename();

    path_to_file.push_str(&filename);

    let file_context = file_io::read_file(path_to_file.to_string());
    let x = file_io::read_u8(path_to_file.clone().to_string());

    let username = user::get_username();

    full_request.push_str(&username);
    full_request.push_str("--");
    full_request.push_str(&user::get_session_key());
    full_request.push_str("#!?#");
    full_request.push_str(&filename);
    full_request.push_str("<$$>");
    full_request.push_str(&file_context);


    full_request
}
