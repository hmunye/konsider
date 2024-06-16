extern crate diesel;

use diesel::pg::PgConnection;


fn main() {
    let connection = PgConnection::establish("postgres://testadmin:z0A8vXcS$af9ErY!ijYa@localhost/").unwrap();
    
}
