use std::result::Result;
use nickel::{Request, Response, Middleware, Continue, MiddlewareResult};
use r2d2_postgres::PostgresConnectionManager;
use r2d2::{Pool, PooledConnection, GetTimeout};
use typemap::Key;
use plugin::Extensible;

pub struct PostgresMiddleware {
    pub pool: Pool<PostgresConnectionManager>,
}

impl PostgresMiddleware {
    pub fn new(pool: Pool<PostgresConnectionManager>) -> PostgresMiddleware {
        PostgresMiddleware { pool: pool }
    }
}

impl Key for PostgresMiddleware { type Value = Pool<PostgresConnectionManager>; }

impl<D> Middleware<D> for PostgresMiddleware {
    fn invoke<'mw, 'conn>(&self, req: &mut Request<'mw, 'conn, D>, res: Response<'mw, D>) -> MiddlewareResult<'mw, D> {
        req.extensions_mut().insert::<PostgresMiddleware>(self.pool.clone());

        Ok(Continue(res))
    }
}

pub trait PostgresRequestExtensions {
    fn db_conn(&self) -> Result<PooledConnection<PostgresConnectionManager>, GetTimeout>;
}

impl<'a, 'b, D> PostgresRequestExtensions for Request<'a, 'b, D> {
    fn db_conn(&self) -> Result<PooledConnection<PostgresConnectionManager>, GetTimeout> {
        self.extensions().get::<PostgresMiddleware>().unwrap().get()
    }
}
