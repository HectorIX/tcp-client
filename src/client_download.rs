use user;
use parser;
use file_io;
use interface;


pub fn download() -> String {

    let mut full_request = "download_state::Download**".to_owned();

    let username = user::get_username();
    let filename = interface::read_filename();

    full_request.push_str(&username);
    full_request.push_str("--");
    full_request.push_str(&filename);

    full_request
}




pub fn store_file_locally( data: String ) -> bool {


    let mut path_to_file = "download/".to_string();


    if data.len() > 0 {

        let (filename, file_context) = parser::extract_file_context(data);

        path_to_file.push_str(&filename);

        file_io::write_file(path_to_file, file_context);

        return true;
    }
    else {

        return false;
    }


    return false;
}
