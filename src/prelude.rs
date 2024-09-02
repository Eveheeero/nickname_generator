use once_cell::sync::Lazy;

pub(crate) static DB: Lazy<sled::Db> = Lazy::new(|| sled::open(".nickname_generator").unwrap());

pub(crate) fn init() {
    // 로그 설정
    tracing_subscriber::fmt::init();
}

const OPENDICT_KEY: &str = "opendict_key";
pub(crate) fn get_opendict_key() -> Option<String> {
    let data = DB.get(OPENDICT_KEY).unwrap();
    data.map(|data| String::from_utf8(data.to_vec()).unwrap())
}
pub(crate) fn set_opendict_key(s: impl AsRef<str>) {
    DB.insert(OPENDICT_KEY, s.as_ref()).unwrap();
    DB.flush().unwrap();
}
