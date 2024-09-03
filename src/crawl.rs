pub(super) async fn main() {
    crate::prelude::init();

    let query = crate::prelude::get_opendict_last_inserted();
    let mut query = if let Some(query) = query {
        crate::data_collector::opendict::get_next_query(query)
    } else {
        crate::data_collector::opendict::get_first_query()
    };

    let mut failed_count = 0;
    loop {
        let data = crate::data_collector::opendict::search_opendict(&query).await;
        if let Ok((data, items)) = data {
            crate::prelude::insert_opendict_data::<true>(&query, data);
            for item in items {
                crate::prelude::insert_opendict_item(&item);
            }
            query = crate::data_collector::opendict::get_next_query(query);
            failed_count = 0;
        } else {
            failed_count += 1;
            if failed_count >= 10 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_secs(3));
        }
    }
}
