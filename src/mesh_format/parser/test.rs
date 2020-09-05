use {
    nom::{
        error::ErrorKind,
        IResult,
    },
    super::{
        decode::{
            format,
            self,
        },
        Format,
    }
};

#[test]
fn mesh_format() {
    let content = "\
$MeshFormat
4.1 0 8
$EndMeshFormat
";
    let expected = Format::new("4.1".to_string(), 0, 8);
    match decode::format::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}
