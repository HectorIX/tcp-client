extern crate lazy_static;


use self::lazy_static::*;
use std::default::Default;
use std::sync::Mutex;




#[derive(Default)]
pub struct User {

    pub username : String,
    pub session_key : String,
    pub active : bool,
}


lazy_static! {

    static ref USER: Mutex<User> = {
        let m = Mutex::new(User::default());
        m
    };
}



pub fn set_username( username:String ) {

    USER.lock().unwrap().username = username;
}


pub fn set_session_key( session_key: String ) {

    USER.lock().unwrap().session_key = session_key;
}


pub fn set_user_status ( active: bool ) {

    USER.lock().unwrap().active = active;
}



pub fn get_username() -> String {

    let username = USER.lock().unwrap().username.clone();
    username
}

pub fn get_session_key() -> String {

    let session_key = USER.lock().unwrap().session_key.clone();
    session_key
}

pub fn get_user_status() -> bool {

    let status = USER.lock().unwrap().active.clone();
    status
}
