use unicode_segmentation::UnicodeSegmentation;

pub fn truncate_name(filename: &str) -> String {
    const MAX_BYTES: usize = 32;
    const ELLIPSIS: &str = "...";

    if filename.len() <= MAX_BYTES {
        return filename.to_owned();
    }

    let mut out = String::new();
    let mut bytes = 0;

    for g in filename.graphemes(true) {
        let g_bytes = g.len();
        if bytes + g_bytes + ELLIPSIS.len() > MAX_BYTES {
            break;
        }
        out.push_str(g);
        bytes += g_bytes;
    }

    out.push_str(ELLIPSIS);
    out
}
