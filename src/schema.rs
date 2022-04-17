table! {
  ledger_entry (id) {
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
  ledger_transaction (id) {
    id -> Int4,
    date_created -> Date,
    memo -> Varchar,
  }
}

joinable!(ledger_entry -> ledger_transaction (transaction_id));

allow_tables_to_appear_in_same_query!(ledger_entry, ledger_transaction,);
