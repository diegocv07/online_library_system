// @generated automatically by Diesel CLI.

diesel::table! {
    books (id) {
        id -> Int4,
        title -> Varchar,
        author -> Varchar,
        #[max_length = 20]
        isbn -> Varchar,
        book_rating -> Int4,
        available -> Bool,
        count -> Int4,
        tags -> Array<Nullable<Text>>,
        language -> Varchar,
        description -> Nullable<Text>,
        genres -> Array<Nullable<Text>>,
    }
}
