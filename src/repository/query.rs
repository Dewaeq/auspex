use sqlx::{Pool, Postgres};

pub struct Query {
    pub pool: Pool<Postgres>,
}

impl Query {
    pub fn new(pool: Pool<Postgres>) -> Self {
        Query { pool }
    }
}
