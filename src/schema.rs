table! {
    ledger_entries (id) {
        id -> Int4,
        transaction_id -> Int4,
        bucket -> Varchar,
        currency -> Nullable<Varchar>,
        credit -> Bool,
        amount -> Float8,
        comment -> Varchar,
    }
}

table! {
    ledger_transactions (id) {
        id -> Int4,
        date_created -> Varchar,
        memo -> Varchar,
    }
}

joinable!(ledger_entries -> ledger_transactions (transaction_id));

allow_tables_to_appear_in_same_query!(ledger_entries, ledger_transactions,);
