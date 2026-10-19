use ndarray::{array, Array1, Array2};
use ndarray_npy::{NpzReader, NpzWriter};
use std::io::Cursor;
use zip::{CompressionMethod, ZipArchive};

fn assert_round_trip(mut writer: NpzWriter<Cursor<Vec<u8>>>, compression: CompressionMethod) {
    let matrix = array![[1i32, 2, 3], [4, 5, 6]];
    let vector = array![7i32, 8, 9];
    let matrix_name = String::from("数组.npy");
    let vector_name = "vector.npy";

    writer.add_array(matrix_name.clone(), &matrix).unwrap();
    writer.add_array(vector_name, &vector).unwrap();
    let archive = writer.finish().unwrap();

    let mut zip = ZipArchive::new(archive.clone()).unwrap();
    for index in 0..zip.len() {
        assert_eq!(zip.by_index(index).unwrap().compression(), compression);
    }

    let mut reader = NpzReader::new(archive).unwrap();
    assert!(!reader.is_empty());
    assert_eq!(reader.len(), 2);
    assert_eq!(
        reader.names().unwrap(),
        vec![matrix_name.clone(), vector_name.into()]
    );
    let read_matrix: Array2<i32> = reader.by_name(&matrix_name).unwrap();
    let read_vector: Array1<i32> = reader.by_index(1).unwrap();
    assert_eq!(read_matrix, matrix);
    assert_eq!(read_vector, vector);
}

#[test]
fn round_trip_stored() {
    assert_round_trip(
        NpzWriter::new(Cursor::new(Vec::new())),
        CompressionMethod::Stored,
    );
}

#[cfg(feature = "compressed_npz")]
#[test]
fn round_trip_compressed() {
    assert_round_trip(
        NpzWriter::new_compressed(Cursor::new(Vec::new())),
        CompressionMethod::Deflated,
    );
}
