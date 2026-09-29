// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        isbn -> Varchar,
        book_rating -> Int4,
        genre -> Nullable<Varchar>,
        tags -> Nullable<Varchar>,
        available -> Bool,
        count -> Int4,
    }
}
