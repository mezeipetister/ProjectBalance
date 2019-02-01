table! {
    transactions (id) {
        id -> Integer,
        title -> Text,
        body -> Text,
        debit -> Integer,
        credit -> Integer,
        payment -> Integer,
        time_created -> Nullable<Text>,
    }
}
