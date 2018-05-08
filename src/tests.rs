use file::NBTFile;
use NBTTag;
use read::read_nbt_file;
use read::read_tag_name;
use read::tuple_vector_to_hash_map;
use std::collections::HashMap;
use write::get_tag_id;

#[test]
fn check_read_name() {
    assert_eq!(read_tag_name(vec![0x00, 0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F].as_slice()), Ok((&b""[..], "Hello")))
}

#[test]
fn check_nbt_file() {
    let input = vec![
        0x0A, 0x00, 0x01, 0x65, 0x08, 0x00, 0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F, 0x00, 0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6f, 0x00
    ];

    let mut compound_contents = HashMap::new();
    compound_contents.insert("Hello".to_owned(), NBTTag::TagString("Hello".to_owned()));

    assert_eq!(read_nbt_file(input.as_slice()), Ok((&b""[..],
                                                    Some(NBTFile {
                                                        root_name: "e".to_owned(),
                                                        root: NBTTag::TagCompound(compound_contents),
                                                    }))));
}

#[test]
fn check_tag_matcher() {
    let input = NBTTag::TagString("Hello World!".to_owned());
    let result = get_tag_id(&input);
    let expected = Some(8);

    assert_eq!(result, expected);
}
