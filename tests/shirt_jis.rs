extern crate dbase;
extern crate shapefile;

use std::collections::HashMap;
use std::convert::TryFrom;
use std::io::Cursor;

use dbase::FieldName;
use dbase::FieldValue;

use dbase::Record;
use dbase::TableWriterBuilder;
use shapefile::reader::Reader as ShapeFileReader;
use shapefile::writer::ShapeWriter;
use shapefile::writer::Writer as ShapeFileWriter;
use shapefile::Point;
use shapefile::ShapeReader;

fn character_field_value(value: Option<&FieldValue>) -> Option<String> {
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
fn shift_jis_read_from_path_with_label() {
    let mut reader =
        ShapeFileReader::from_path_with_label("tests/data/shift_jis.shp", "shift_jis").unwrap();
    let mut records = HashMap::new();
    for (index, result) in reader.iter_shapes_and_records().enumerate() {
        let (_, record) = result.unwrap();
        let book = character_field_value(record.get("書籍名"));
        let author = character_field_value(record.get("著者"));
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

// #[test]
// fn shift_jis_write_from_path_with_info_and_label() {
//     let reader =
//         ShapeFileReader::from_path_with_label("tests/data/shift_jis.shp", "shift_jis").unwrap();
//     let table_info = reader.into_table_info();
//     let writer = ShapeFileWriter::from_path_with_info_and_label(
//         "tests/data/temp.shp",
//         table_info,
//         "shift_jis",
//     );
//     assert!(writer.is_ok());
// }

#[test]
fn shift_jis_write_writer() {
    let label = "shift_jis";
    let expected_field_name = "書籍名";
    let expected_value = "伊豆の踊り子";

    // create ShapeWriter
    let mut shp = Cursor::new(Vec::<u8>::new());
    let mut shx = Cursor::new(Vec::<u8>::new());
    let shape_writer = ShapeWriter::with_shx(&mut shp, &mut shx);

    // create dbase TableWriter
    let field_name = FieldName::try_from(expected_field_name).unwrap();
    let mut dbase = Cursor::new(Vec::<u8>::new());
    let dbase_writer_builder = TableWriterBuilder::new_with_label(label)
        .unwrap()
        .add_character_field(field_name, 40);
    let dbase_writer = dbase_writer_builder.build_with_dest(&mut dbase);

    // create data
    let point = Point::new(0.0, 0.0);
    let mut record = Record::default();
    let field_value = FieldValue::Character(Some(expected_value.to_string()));
    record.insert(expected_field_name.to_string(), field_value);

    // create shapefile::Writer
    {
        let mut writer = ShapeFileWriter::new(shape_writer, dbase_writer);
        writer.write_shape_and_record(&point, &record).unwrap();
    }

    // set file position to zero
    shp.set_position(0);
    shx.set_position(0);
    dbase.set_position(0);

    // read shape file
    let shape_reader = ShapeReader::with_shx(shp, shx).unwrap();
    let dbase_reader = dbase::Reader::new_with_label(dbase, label).unwrap();
    let mut reader = ShapeFileReader::new(shape_reader, dbase_reader);
    let features = reader.read_as::<Point, Record>().unwrap();

    // check a result of read
    assert_eq!(features.len(), 1);
    let feature = features.get(0).unwrap();
    let (_, record) = feature;
    let value = character_field_value(record.get(expected_field_name));
    assert_eq!(value.as_deref().unwrap(), expected_value);
}
