table! {
    currency (name) {
        name -> Varchar,
    }
}

table! {
    wallet (id) {
        id -> Varchar,
        deposit -> BigInt,
        currency -> Nullable<Varchar>,
    }
}

joinable!(wallet -> currency (currency));

allow_tables_to_appear_in_same_query!(currency, wallet,);
