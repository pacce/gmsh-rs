use {
    nom::{
        error::ErrorKind,
        IResult,
    },
    super::{
        decode::{
            mesh_format,
            self,
        },
        MeshFormat,
    }
};

#[test]
fn format() {
    let content = "\
$MeshFormat
4.1 0 8
$EndMeshFormat
";
    let expected = MeshFormat::new("4.1".to_string(), 0, 8);
    match decode::mesh_format::<(&str, ErrorKind)>(content) {
        Ok((_, actual)) => assert_eq!(expected, actual),
        Err(_) => assert!(false),
    }
}
