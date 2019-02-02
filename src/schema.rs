table! {
    transactions (id) {
        id -> Int4,
        title -> Varchar,
        debit -> Int4,
        credit -> Int4,
        payment -> Int4,
        time_created -> Timestamptz,
    }
}
