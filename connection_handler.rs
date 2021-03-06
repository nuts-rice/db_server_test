use std::env;

use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Outcome, Request, State};
use std::ops::Deref;

use diesel::PgConnection;
use r2d2_diesel::ConnectionManager;

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(database_url());
    Pool::new(manager).expect("db_pool")
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("Expected DATABASE_URL")
}

//Defining connection request guard here
pub struct DbConn(pub r2d2::PooledConnection<ConnectionManager<PgConnection>>);

//Implement DbConn here
//We have to be extra careful with ownership mechanics here because this is a
//network resource. Thus explicitly stating lifetime as well as refrence patterns
impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();
    fn from_request(request: &'a Request<'r>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}
//Derefrence/Deconstructor here
impl Deref for DbConn {
    type Target = PgConnection;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
