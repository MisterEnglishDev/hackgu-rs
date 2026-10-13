//! blobdump file.ccs out.bin : write the payload of the first BinaryBlob (0x2400) section.
use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for b in c.objects_of_type(objects::SECTION_BINARYBLOB) {
        println!("{} len {}", c.object_name(b.object_id).unwrap_or("?"), b.payload.len());
        std::fs::write(&a[2], &b.payload).unwrap();
        break;
    }
}
