pub fn dump_spv(path: &str, spv: &[u32]) {
    let path = std::path::Path::new(path);
    let mut buf = Vec::<u8>::with_capacity(spv.len() * 4);
    for w in spv {
        buf.push(((w >> 0) & 0xFF) as u8);
        buf.push(((w >> 8) & 0xFF) as u8);
        buf.push(((w >> 16) & 0xFF) as u8);
        buf.push(((w >> 24) & 0xFF) as u8);
    }
    std::fs::write(path, buf).unwrap();
}
