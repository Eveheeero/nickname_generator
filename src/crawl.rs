pub(super) async fn main() {
    let query = crate::prelude::get_opendict_last_inserted();
    let mut query = if let Some(query) = query {
        crate::data_collector::opendict::get_next_query(query)
    } else {
        crate::data_collector::opendict::get_first_query()
    };

    loop {
        let data = crate::data_collector::opendict::search_opendict(&query).await;
        if let Ok(data) = data {
            crate::prelude::insert_opendict_data::<true>(&query, data);
            query = crate::data_collector::opendict::get_next_query(query);
        } else {
            break;
        }
    }
}
