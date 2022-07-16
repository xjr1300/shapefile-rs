extern crate dbase;
extern crate shapefile;

use std::collections::HashMap;

use dbase::FieldValue;

use shapefile::Reader;

fn character_value(value: Option<&FieldValue>) -> Option<String> {
    match value {
        Some(value) => match value {
            FieldValue::Character(value) => match value {
                Some(value) => Some(value.clone()),
                None => None,
            },
            _ => None,
        },
        None => None,
    }
}

#[test]
fn shift_jis_from_path_with_label() {
    let mut reader = Reader::from_path_with_label("tests/data/shift_jis.shp", "shift_jis").unwrap();
    let mut records = HashMap::new();
    for (index, result) in reader.iter_shapes_and_records().enumerate() {
        let (_, record) = result.unwrap();
        let book = character_value(record.get("書籍名"));
        let author = character_value(record.get("著者"));
        records.insert(index, (book, author));
    }
    // check
    let mut expected_data = HashMap::new();
    expected_data.insert(0, (Some("浮雲"), Some("二葉亭四迷")));
    expected_data.insert(1, (Some("十三夜"), Some("樋口一葉")));
    expected_data.insert(2, (Some("金色夜叉"), Some("尾崎紅葉")));
    expected_data.insert(3, (Some("三四郎"), Some("夏目漱石")));
    expected_data.insert(4, (Some("羅生門"), Some("芥川龍之介")));
    expected_data.insert(5, (Some("平家物語"), None));

    for index in 0..records.keys().len() {
        let record = records.get(&index).unwrap();
        let expected = expected_data.get(&index).unwrap();
        assert_eq!(record.0.as_deref(), expected.0);
        assert_eq!(record.1.as_deref(), expected.1);
    }
}
