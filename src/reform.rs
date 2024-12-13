#[allow(dead_code)]
pub(super) async fn reform() {
    let mut items = Vec::new();

    /* 모든 데이터에 대해 조회 */
    let codes = crate::prelude::get_opendict_item_codes();
    for code in codes {
        let item = crate::prelude::get_opendict_item(code).unwrap();
        items.push(item);
    }

    /* 수정 */

    /* 저장 */
    // for item in items {
    //     crate::prelude::insert_opendict_item(&item);
    // }
}
