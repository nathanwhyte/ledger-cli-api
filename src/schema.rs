table! {
    ledger_entries (id) {
        id -> Int4,
        transaction_id -> Int4,
        bucket -> Text,
        currency -> Nullable<Text>,
        credit -> Bool,
        amount -> Float8,
        comment -> Nullable<Text>,
    }
}

table! {
    ledger_transactions (id) {
        id -> Int4,
        date_created -> Text,
        memo -> Text,
    }
}

joinable!(ledger_entries -> ledger_transactions (transaction_id));

allow_tables_to_appear_in_same_query!(ledger_entries, ledger_transactions,);
