use hackgu_ccs::*;
fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = CcsFile::parse(&std::fs::read(&a[1]).unwrap()).unwrap();
    for (i, o) in c.objects.iter().enumerate() { print!("{}:{} ", i, o.name); }
    println!(); for f in &c.files { print!("[{}] ", f.name); } println!();
}
