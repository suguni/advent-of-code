// fn load_data(text: &str) -> (Vec<i32>, usize, usize) {
// let rows = text.lines().count();
// let cols = text.lines().next().unwrap().chars().count();
// let levels = text
//     .lines()
//     .flat_map(|line| {
//         line.chars()
//             .map(|c| c.to_digit(10).unwrap() as i32)
//             .collect::<Vec<i32>>()
//     })
//     .collect::<Vec<i32>>();

// (levels, rows, cols)
// }
fn hex2bins(c: char) -> Vec<u8> {
    let mut v = c.to_digit(16).unwrap();
    let mut bins = vec![0_u8; 4];
    let mut c = 3;

    loop {
        bins[c] = (v & 1) as u8;

        v >>= 1;

        if v == 0 {
            return bins;
        }

        c -= 1;
    }
}

pub fn hexs2bins(hs: &str) -> Vec<u8> {
    hs.chars().flat_map(|c| hex2bins(c)).collect::<Vec<u8>>()
}

fn bins2num(bins: &Vec<u8>, offset: usize, len: usize) -> u32 {
    let mut v = 0;
    for i in offset..(offset + len) {
        v = (v << 1) + bins[i] as u32;
    }
    v
}

fn packet_version(packet: &Vec<u8>, offset: usize) -> u32 {
    bins2num(packet, offset, 3)
}

fn packet_type(packet: &Vec<u8>, offset: usize) -> u32 {
    bins2num(packet, offset + 3, 3)
}

fn length_of_sub_packet(packet: &Vec<u8>, offset: usize) -> u32 {
    bins2num(packet, offset + 7, 15)
}

fn count_of_sub_packet(packet: &Vec<u8>, offset: usize) -> u32 {
    bins2num(packet, offset + 7, 11)
}

fn end_of_literal_packet(packet: &Vec<u8>, offset: usize) -> usize {
    let mut offset = offset + 6;
    loop {
        if packet[offset] == 0 {
            return offset + 5;
        }
        offset += 5;
    }
}

const LITERAL: u32 = 4;

pub fn count_versions(packet: &Vec<u8>, offset: usize) -> (u32, usize) {
    let v = packet_version(packet, offset);
    let t = packet_type(packet, offset);

    if t == LITERAL {
        return (v, end_of_literal_packet(packet, offset));
    }

    let bit_i = packet[offset + 6];

    let mut count = 0;
    let mut current_offset = offset;

    if bit_i == 0 {
        let sub_packet_len = length_of_sub_packet(packet, offset);
        let end_of_packet = offset + 22 + sub_packet_len as usize;

        current_offset += 22;

        while current_offset < end_of_packet {
            let (c, next_offset) = count_versions(packet, current_offset);
            count += c;
            current_offset = next_offset;
        }
    } else {
        let sub_packet_count = count_of_sub_packet(packet, offset);
        current_offset += 18;

        for _ in 0..sub_packet_count {
            let (c, next_offset) = count_versions(packet, current_offset);
            count += c;
            current_offset = next_offset;
        }
    }
    return (count + v, current_offset);
}

pub fn quiz1(text: &str) -> u32 {
    let packet = hexs2bins(text);
    let (count, _) = count_versions(&packet, 0);
    count
}

// pub fn quiz2(text: &str) -> i32 {}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::*;

    // const DATA1: &str = "";

    #[test]
    fn test_count_versions() {
        let packet = hexs2bins("D2FE28");
        assert_eq!(count_versions(&packet, 0), (6, 21));

        let packet = hexs2bins("38006F45291200");
        assert_eq!(count_versions(&packet, 0), (0b001 + 0b110 + 0b010, 49));

        let packet = hexs2bins("EE00D40C823060");
        assert_eq!(
            count_versions(&packet, 0),
            (0b111 + 0b010 + 0b100 + 0b001, 51)
        );

        let packet = hexs2bins("8A004A801A8002F478");
        assert_eq!(count_versions(&packet, 0).0, 16);

        let packet = hexs2bins("620080001611562C8802118E34");
        assert_eq!(count_versions(&packet, 0).0, 12);

        let packet = hexs2bins("C0015000016115A2E0802F182340");
        assert_eq!(count_versions(&packet, 0).0, 23);

        let packet = hexs2bins("A0016C880162017C3686B18A3D4780");
        assert_eq!(count_versions(&packet, 0).0, 31);
    }

    #[test]
    fn test_end_of_literal_packet() {
        let packet = hexs2bins("D2FE28");
        assert_eq!(end_of_literal_packet(&packet, 0), 21);
    }

    #[test]
    fn test_packet_component() {
        let packet = hexs2bins("D2FE28");
        assert_eq!(packet_version(&packet, 0), 6);
        assert_eq!(packet_type(&packet, 0), 4);

        let packet = hexs2bins("38006F45291200");
        assert_eq!(packet_version(&packet, 22), 6);
        assert_eq!(packet_type(&packet, 22), 4);
        assert_eq!(length_of_sub_packet(&packet, 0), 27);

        let packet = hexs2bins("EE00D40C823060");
        assert_eq!(count_of_sub_packet(&packet, 0), 3);
    }

    #[test]
    fn test_bins2num() {
        assert_eq!(bins2num(&vec![1, 1, 0], 0, 3), 6);
    }

    #[test]
    fn test_hex2bins() {
        assert_eq!(hex2bins('0'), vec![0, 0, 0, 0]);
        assert_eq!(hex2bins('F'), vec![1, 1, 1, 1]);
    }

    #[test]
    fn test_hexs2bins() {
        assert_eq!(
            hexs2bins("D2FE28"),
            "110100101111111000101000"
                .chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        );
    }

    #[test]
    fn run_2021_d16_quiz1() {
        let text = read_file("data/2021/input16.txt");
        assert_eq!(quiz1(text.as_str().trim()), 993);
    }

    // #[test]
    // fn run_2021_d16_quiz2() {
    //     assert_eq!(quiz2(DATA1.trim()), 315);

    //     let text = read_file("data/2021/input16.txt");
    //     assert_eq!(quiz2(text.as_str().trim()), 2948);
    // }
}
