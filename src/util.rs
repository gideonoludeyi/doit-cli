use nanoid::nanoid;
use rusqlite::Connection;

const VALID_ID_CHARACTERS: [char; 16] = [
    '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', 'a', 'b', 'c', 'd', 'e', 'f',
];

pub fn random_id() -> String {
    nanoid!(8, &VALID_ID_CHARACTERS)
}

pub struct Config {
    pub conn: Connection,
}
