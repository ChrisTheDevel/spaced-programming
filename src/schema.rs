table! {
    inbox (id) {
        id -> Integer,
        url -> Text,
    }
}

table! {
    items (id) {
        id -> Integer,
        intervall -> Integer,
        difficulty -> Float,
        memory_strength -> Float,
        adjusting_factor -> Float,
        times_reviewed -> Integer,
        times_recalled -> Integer,
        url -> Text,
        item_data -> Text,
    }
}

table! {
    schedule (id) {
        id -> Integer,
        due -> Integer,
        item_id -> Integer,
    }
}

joinable!(schedule -> items (item_id));

allow_tables_to_appear_in_same_query!(
    inbox,
    items,
    schedule,
);
