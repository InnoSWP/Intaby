mod database;
mod game;

use database::{psql::*, DBAccessor};

fn main() -> PsqlResult<()> {
    let db = PsqlAccess::new()?;

    Ok(())
}
