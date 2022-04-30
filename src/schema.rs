// The table! macro creates a bunch of code based on the database schema 
// to represent all of the tables and columns. We’ll see how exactly to 
// use that in the next example.

table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
    }
}
