table! {
    posts (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        published -> Bool,
        start_time -> Timestamptz,
    }
}

table! {
    plans (id) {
        id -> Int4,
        site_id -> Varchar,
        stage_id -> Varchar,
        oper_id -> Varchar,
        resource_id -> Varchar,
        product_id -> Varchar,
        plan_qty -> Numeric,
        start_time -> Timestamptz,
        end_time -> Timestamptz,
    }
}

table! {
    products (id) {
        id -> Int4,
        created_at -> Nullable<Timestamptz>,
        updated_at -> Nullable<Timestamptz>,
        deleted_at -> Nullable<Timestamptz>,
        code -> Nullable<Text>,
        price -> Nullable<Int4>,
        start_time -> Nullable<Timestamptz>,
    }
}

table! {
    users (email) {
        email -> Varchar,
        password -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    posts,
    plans,
    products,
    users,
);
