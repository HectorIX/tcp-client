

fn request_constructor(req:String) -> String {

    match req.as_ref() {

        "help" => {
            "help".to_string()
        },
        "sign_up" => {
            "sign_up".to_string()
        },
        "sign_in" => {
            "sign_in".to_string()
        },
        "upload" => {
            "upload".to_string()
        },
        "download" => {
            "download".to_string()
        },
        "integrity" => {
            "integrity".to_string()
        },
        "encrypt" => {
            "encrypt".to_string()
        },
        "decrypt" => {
            "decrypt".to_string()
        },
        "exit" => {
            "exit".to_string()
        },
        _ => {
            "\n\t+ Typpo(!!!), please try again...\n".to_string()
        },

    }

}
