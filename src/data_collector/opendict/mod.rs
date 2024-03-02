#[must_use]
async fn search_opendict(
    api_key: impl AsRef<str>,
    keyword: impl AsRef<str>,
    page: u16,          // 1~1000
    amount: u8,         // 1~100
    pos: Pos,           // 품사
    region: Region,     // 방언 지역
    category: Category, // 전문 분야
) -> Result<String, ()> {
    // https://opendict.korean.go.kr/service/openApiInfo

    let url = format!(
        "https://opendict.korean.go.kr/api/search?key={}&q={}&req_type=json&start={}&num={}&method=include&advanced=y&pos={}&region={}&cat={}",
        api_key.as_ref(),
        keyword.as_ref(),
        page,
        amount,
        pos as u8,
        region as u8,
        category as u8
    );

    let response = reqwest::Client::new()
        .get(url)
        .header("User-Agent", "reqwest")
        .send()
        .await
        .ok()
        .ok_or(())?
        .text()
        .await
        .ok()
        .ok_or(())?;
    Ok(response)
}

#[repr(u8)]
#[derive(Default)]
enum Pos {
    #[default]
    All,
}
#[repr(u8)]
#[derive(Default)]
enum Region {
    #[default]
    All,
}
#[repr(u8)]
#[derive(Default)]
enum Category {
    #[default]
    All,
}

#[tokio::test]
async fn test_search() -> Result<(), Box<dyn std::error::Error>> {
    use std::str::FromStr;

    let key = std::fs::read_to_string("api_key.txt")?;
    let amount = 100;

    let result = search_opendict(key, "가", 1, amount, Pos::All, Region::All, Category::All).await;

    assert!(result.is_ok());

    let result = serde_json::Value::from_str(&result.unwrap())?;

    assert_eq!(
        result["channel"]["item"].as_array().unwrap().len(),
        amount as usize
    );

    Ok(())
}
