pub struct Slot {
    slot: String,
    count: usize,
}

pub fn generate_dictionary<'a>(data: String, slen: usize) -> Vec<String> {
    const MLEN: usize = 896;
    let mut dict: Vec<Slot> = Vec::new();
    for i in 0..data.len() - slen {
        for j in 2..slen {
            let slot: String = data[i..i + j].to_string();
            if dict.iter().any(|r| r.slot == slot) {
                // do nothing
            } else {
                dict.push(Slot {
                    slot: slot.to_string(),
                    count: data.matches(&slot).count(),
                });
            }
        }
    }
    dict.sort_by(|a, b| b.count.cmp(&a.count));
    let mut v: Vec<String> = dict[0..MLEN].iter().map(|r| r.slot.to_string()).collect();
    v.sort_by(|a, b| b.len().cmp(&a.len()));
    v.to_vec()
}

pub fn tiny_string_deflate (data: String, dict: Vec<String>) -> String {
    let mut compressed: String = data;
    let mut i: u32 = 0;
    for slot in dict {
        let r: String = std::char::from_u32(128 + i).unwrap().to_string();
        compressed = compressed.replace(&slot, &r);
        i += 1;
    }
    compressed
}

pub fn tiny_string_inflate (data: String, dict: Vec<String>) -> String {
    let mut compressed: String = data;
    let mut i: u32 = 0;
    for slot in dict {
        let r: String = std::char::from_u32(128 + i).unwrap().to_string();
        compressed = compressed.replace(&r, &slot);
        i += 1;
    }
    compressed
}
