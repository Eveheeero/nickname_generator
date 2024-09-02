use once_cell::sync::Lazy;

pub(crate) static DB: Lazy<sled::Db> = Lazy::new(|| sled::open(".nickname_generator").unwrap());
const OPENDICT_KEY: &str = "opendict_key";
const OPENDICT_DATA_KEY: &str = "opendict_data";

pub(crate) fn init() {
    // 로그 설정
    tracing_subscriber::fmt::init();
}

pub(crate) fn get_opendict_key() -> Option<String> {
    let data = DB.get(OPENDICT_KEY).unwrap();
    data.map(|data| String::from_utf8(data.to_vec()).unwrap())
}
pub(crate) fn set_opendict_key(s: impl AsRef<str>) {
    DB.insert(OPENDICT_KEY, s.as_ref()).unwrap();
    DB.flush().unwrap();
}
fn get_opendict_tree() -> sled::Tree {
    DB.open_tree(OPENDICT_DATA_KEY).unwrap()
}
pub(crate) fn get_opendict_saved_queries() -> Vec<crate::data_collector::opendict::OpendictQuery> {
    let tree = get_opendict_tree();
    let mut queries = vec![];
    for query in tree.iter() {
        let (key, _) = query.unwrap();
        let query: crate::data_collector::opendict::OpendictQuery =
            serde_json::from_slice(&key).unwrap();
        queries.push(query);
    }
    queries
}
pub(crate) fn insert_opendict_data(
    query: &crate::data_collector::opendict::OpendictQuery,
    data: crate::data_collector::opendict::v1::OpendictResult,
) {
    let data = serde_json::to_vec(&data).unwrap();
    let data = gzip_compress(&data);
    let tree = get_opendict_tree();
    tree.insert(serde_json::to_vec(query).unwrap(), data)
        .unwrap();
    tree.flush().unwrap();
}
pub(crate) fn get_opendict_data(
    query: &crate::data_collector::opendict::OpendictQuery,
) -> Option<crate::data_collector::opendict::v1::OpendictResult> {
    let tree = get_opendict_tree();
    let data = tree.get(serde_json::to_vec(query).unwrap()).unwrap();
    data.map(|data| gzip_decompress(&data))
        .map(|data| serde_json::from_slice(&data).unwrap())
}
fn gzip_compress(data: &[u8]) -> Vec<u8> {
    use flate2::write::GzEncoder;
    use std::io::Write;

    let mut encoder = GzEncoder::new(Vec::new(), flate2::Compression::best());
    encoder.write_all(data).unwrap();
    encoder.finish().unwrap()
}
fn gzip_decompress(data: &[u8]) -> Vec<u8> {
    use flate2::read::GzDecoder;
    use std::io::Read;

    let mut decoder = GzDecoder::new(data);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed).unwrap();
    decompressed
}
