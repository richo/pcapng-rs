/// The padding you'll need to read to catch the reader up to alignment.
/// Returns a usize, because you'll probably never pass it to anything other than take!
pub fn pad_to_32bits(length: u32) -> usize {
    let padding = length % 4;
    if padding == 0 {
        0
    } else {
        let delta = 4 - padding;
        delta as usize
    }
}
