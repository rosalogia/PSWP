table! {
    problems (id) {
        id -> Integer,
        author_id -> Integer,
        name -> Text,
        description -> Text,
        test_case_file -> Text,
    }
}

table! {
    user_accounts (id) {
        id -> Integer,
        username -> Text,
        password_hash -> Text,
    }
}

allow_tables_to_appear_in_same_query!(problems, user_accounts,);
