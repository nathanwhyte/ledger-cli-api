pub mod db_utils {

    use diesel::{pg::PgConnection, prelude::*};
    use dotenv::dotenv;
    use std::env;

    use crate::{
        models::{LedgerTransaction, NewLedgerTransaction},
        schema::ledger_transactions,
    };

    pub fn establish_connection() -> PgConnection {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    pub fn create_ledger_transaction<'a>(
        connection: &PgConnection,
        date_created: &'a str,
        memo: &'a str,
    ) -> LedgerTransaction {
        let new_ledger_transaction = NewLedgerTransaction { date_created, memo };

        diesel::insert_into(ledger_transactions::table)
            .values(&new_ledger_transaction)
            .get_result(connection)
            .expect("error adding new ledger transaction to database")
    }
}
