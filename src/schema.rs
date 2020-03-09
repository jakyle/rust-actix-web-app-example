table! {
    todos (id) {
        id -> Uuid,
        finished -> Nullable<Bool>,
        item -> Varchar,
        description -> Nullable<Varchar>,
    }
}
