use std::str::FromStr;
pub(crate) mod v1;
use v1::*;

/// 오픈사전 검색 키
pub(crate) struct OpendictQuery {
    /// 검색 키워드
    pub(crate) keyword: String,
    /// 페이지 번호, 1~1000
    pub(crate) page: u16,
    /// 한 페이지에 보여줄 결과 수, 최대 100
    pub(crate) amount: u8,
    /// 품사, 없으면 전체
    pub(crate) pos: Vec<Pos>,
    /// 방언 지역, 없으면 전체
    pub(crate) region: Vec<Region>,
    /// 전문 분야, 없으면 전체
    pub(crate) category: Vec<Category>,
}

#[must_use]
pub(crate) async fn search_opendict(query: OpendictQuery) -> Result<OpendictResult, ()> {
    // https://opendict.korean.go.kr/service/openApiInfo

    let OpendictQuery {
        keyword,
        page,
        amount,
        pos,
        region,
        category,
    } = query;
    let pos = if pos.is_empty() {
        "0".to_owned()
    } else {
        pos.into_iter()
            .map(|x| format!("{}", x as u8))
            .collect::<Vec<_>>()
            .join(",")
    };
    let region = if region.is_empty() {
        "0".to_owned()
    } else {
        region
            .into_iter()
            .map(|x| format!("{}", x as u8))
            .collect::<Vec<_>>()
            .join(",")
    };
    let category = if category.is_empty() {
        "0".to_owned()
    } else {
        category
            .into_iter()
            .map(|x| format!("{}", x as u8))
            .collect::<Vec<_>>()
            .join(",")
    };

    let url = format!(
        "https://opendict.korean.go.kr/api/search?key={}&q={}&req_type=json&start={}&num={}&method=include&advanced=y&pos={}&region={}&cat={}",
        crate::prelude::get_opendict_key().expect("Opendict 키가 설정되지 않았습니다."),
        keyword,
        page,
        amount,
        pos,
        region ,
        category
    );

    let client = reqwest::Client::new();
    let response = client
        .get(&url)
        .header("User-Agent", "reqwest")
        .send()
        .await
        .ok()
        .ok_or(())?;
    if !response.status().is_success() {
        tracing::error!(
            "Failed to get response from opendict: {}",
            response.status()
        );
        return Err(());
    }
    let response = response.text().await.ok().ok_or(())?;
    if cfg!(debug_assertions) {
        std::fs::write("opendict_response.json", &response).ok();
    }
    tracing::trace!("Response from opendict: {}", response);
    string_to_result(response)
}

fn string_to_result(s: impl AsRef<str>) -> Result<OpendictResult, ()> {
    let json = serde_json::Value::from_str(s.as_ref()).ok().ok_or(())?;
    let json = &json["channel"];
    let datetime = json["lastbuilddate"].as_str().ok_or(())?;
    let datetime = parse_datetime(datetime)?;
    let mut result = OpendictResult {
        total: json["total"].as_u64().ok_or(())? as u32,
        size: json["num"].as_u64().ok_or(())? as u32,
        page: json["start"].as_u64().ok_or(())? as u32,
        data: Vec::new(),
        datetime,
    };
    for item in json["item"].as_array().ok_or(())? {
        let word = item["word"].as_str().ok_or(())?.to_owned();
        let sense = &item["sense"][0];
        if item["sense"].as_array().ok_or(())?.len() != 1 {
            assert!(false);
        }
        let definition = sense["definition"].as_str().ok_or(())?.to_owned();
        let code = sense["target_code"]
            .as_str()
            .ok_or(())?
            .parse()
            .ok()
            .ok_or(())?;
        let r#type = sense["type"].as_str().ok_or(())?.to_owned();
        let pos = sense["pos"].as_str().ok_or(())?.to_owned();
        result.data.push(OpendictData {
            word,
            definition,
            code,
            r#type,
            pos,
        });
    }
    Ok(result)
}
fn parse_datetime(s: impl AsRef<str>) -> Result<time::PrimitiveDateTime, ()> {
    let format = time::macros::format_description!("[year][month][day][hour][minute][second]");
    let result = time::PrimitiveDateTime::parse(s.as_ref(), format);
    result.ok().ok_or(())
}

/// 품사
#[repr(u8)]
#[derive(Copy, Clone)]
#[allow(unused)]
pub(crate) enum Pos {
    명사 = 1,
    대명사,
    수사,
    조사,
    동사,
    형용사,
    관형사,
    부사,
    감탄사,
    접사,
    의존명사,
    보조동사,
    보조형용사,
    어미,
    관형사명사,
    수사관형사,
    명사부사,
    감탄사명사,
    대명사부사,
    대명사감탄사,
    동사형용사,
    관형사감탄사,
    부사감탄사,
    의존명사조사,
    수사관형사명사,
    대명사관형사,
    품사없음,
}
/// 방언 지역
#[repr(u8)]
#[derive(Copy, Clone)]
#[allow(unused)]
pub(crate) enum Region {
    강원 = 1,
    경기,
    경남,
    경북,
    경상,
    전남,
    전라,
    전북,
    제주,
    충남,
    충북,
    충청,
    평남,
    평북,
    평안,
    함경,
    함남,
    함북,
    황해,
    중국길림성,
    중국요령성,
    중국흑룡강성,
    중앙아시아,
}
/// 전문 분야
#[repr(u8)]
#[derive(Copy, Clone)]
#[allow(unused)]
pub(crate) enum Category {
    가톨릭 = 1,
    건설,
    경영,
    경제,
    고유명일반,
    공업,
    공예,
    공학일반,
    광업,
    교육,
    교통,
    군사,
    기계,
    기독교,
    농업,
    동물,
    매체,
    무용,
    문학,
    물리,
    미술,
    민속,
    법률,
    보건일반,
    복식,
    복지,
    불교,
    사회일반,
    산업일반,
    생명,
    서비스업,
    수산업,
    수의,
    수학,
    식물,
    식품,
    심리,
    약학,
    언어,
    역사,
    연기,
    영상,
    예체능일반,
    음악,
    의학,
    인명,
    인문일반,
    임업,
    자연일반,
    재료,
    전기전자,
    정보통신,
    정치,
    종교일반,
    지구,
    지리,
    지명,
    책명,
    천문,
    천연자원,
    철학,
    체육,
    한의,
    해양,
    행정,
    화학,
    환경,
}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_search() -> Result<(), Box<dyn std::error::Error>> {
        crate::prelude::init();

        let amount = 100;
        let key = super::OpendictQuery {
            keyword: "가".to_owned(),
            page: 1,
            amount,
            pos: vec![],
            region: vec![],
            category: vec![],
        };

        let result = super::search_opendict(key).await;

        assert!(result.is_ok());

        let result = result.unwrap();

        assert_eq!(result.size as usize, amount as usize);

        Ok(())
    }
}
