use deserr::{deserialize, Deserr, JsonError};
use insta::{assert_debug_snapshot, assert_display_snapshot};
use serde_json::json;

#[test]
fn error_attribute() {
    #[allow(unused)]
    #[derive(Debug, Deserr)]
    #[deserr(error = JsonError)]
    struct Struct {
        doggo: String,
        #[deserr(error = JsonError)]
        catto: String,
    }

    // now deserr know the error type to use
    let data = deserialize::<Struct, _, _>(json!({ "doggo": "bork", "catto": "jorts" })).unwrap();
    assert_debug_snapshot!(data, @r###"
    Struct {
        doggo: "bork",
        catto: "jorts",
    }
    "###);

    let data = deserialize::<Struct, _, _>(json!({ "catto": "jorts" })).unwrap_err();
    assert_display_snapshot!(data, @"Missing field `doggo`");

    let data = deserialize::<Struct, _, _>(json!({ "doggo": "bork" })).unwrap_err();
    assert_display_snapshot!(data, @"Missing field `catto`");
}
