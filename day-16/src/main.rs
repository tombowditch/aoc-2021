use std::fs;

fn input_to_binary(s: String) -> Vec<char> {
    let mut binary_data = s
        .chars()
        .map(|d| match d {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
            _ => panic!("invalid identifier"),
        })
        .collect::<Vec<_>>()
        .join("")
        .chars()
        .collect::<Vec<char>>();

    binary_data
}

#[derive(Debug, Clone)]
enum Content {
    LV(usize),
    Packets(Vec<Packet>),
}

#[derive(Debug, Clone)]
struct Packet {
    version: usize,
    packet_type: usize,
    length_type_id: usize,
    inner_data: Content,
}

impl Packet {
    fn version_sum(&self) -> usize {
        match &self.inner_data {
            Content::LV(_) => self.version,
            Content::Packets(packets) => {
                packets.iter().map(Packet::version_sum).sum::<usize>() + self.version
            }
        }
    }

    fn print_lvs(&self) {
        match &self.inner_data {
            Content::LV(lv) => println!("{}", lv),
            Content::Packets(packets) => {
                for packet in packets {
                    packet.print_lvs();
                }
            }
        }
    }

    fn eval(&self) -> usize {
        match &self.inner_data {
            Content::LV(value) => {
                println!("val = {}", value);

                *value as usize
            }
            Content::Packets(packets) => match &self.packet_type {
                0 => packets.iter().map(Packet::eval).sum::<usize>(),
                1 => packets.iter().map(Packet::eval).product::<usize>(),
                2 => packets.iter().map(Packet::eval).min().unwrap(),
                3 => packets.iter().map(Packet::eval).max().unwrap(),
                5 => {
                    if packets[0].eval() > packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                6 => {
                    if packets[0].eval() < packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                7 => {
                    if packets[0].eval() == packets[1].eval() {
                        1
                    } else {
                        0
                    }
                }
                _ => todo!(),
            },
        }
    }
}

fn main() {
    let c = fs::read_to_string("input.txt").unwrap();
    let mut binary_data = input_to_binary(c);

    let (outer_packet, _uselesszeros) = parse_packet(&binary_data);

    println!("final packet: {:#?}", outer_packet);
    println!("end data: {:?}", _uselesszeros);

    println!("version = {}", outer_packet.version_sum());
    println!("[part 2] value = {}", outer_packet.eval());

    // outer_packet.print_lvs();
}

fn parse_packet(mut bit_stream: &[char]) -> (Packet, &[char]) {
    let (version, mut bit_stream) = parse_version(bit_stream);
    println!("version = {}", version);
    let (type_id, mut bit_stream) = parse_type_id(bit_stream);
    println!("type id = {}", type_id);

    if type_id == 4 {
        // LV packet

        let mut end = false;
        let mut final_combine = vec![];

        while !end {
            let cont = &bit_stream[..1];
            bit_stream = &bit_stream[1..];

            let num = &bit_stream[..4];
            bit_stream = &bit_stream[4..];

            let s = num.iter().collect::<String>();
            final_combine.push(s);

            if cont[0] == '0' {
                end = true;
            }
        }

        return (
            Packet {
                version,
                packet_type: type_id,
                inner_data: Content::LV(to_dec(final_combine.join("").as_str()).into()),
                length_type_id: 0,
            },
            bit_stream,
        );
    } else {
        // op packet
        let (length_type_id, bit_stream) = parse_length_type_id(bit_stream);

        if length_type_id == 0 {
            let (subpacket_bit_length, mut bit_stream) = parse_subpacket_bit_length(bit_stream);

            let mut subpackets = vec![];

            let mut sub_bit_stream = &bit_stream[..subpacket_bit_length];
            bit_stream = &bit_stream[subpacket_bit_length..];

            while sub_bit_stream.len() > 0 {
                let (sp, sub_bit_stream_inner) = parse_packet(sub_bit_stream);
                sub_bit_stream = sub_bit_stream_inner;
                subpackets.push(sp);
            }

            return (
                Packet {
                    version,
                    packet_type: type_id,
                    inner_data: Content::Packets(subpackets),
                    length_type_id: length_type_id,
                },
                bit_stream,
            );
        } else if length_type_id == 1 {
            let (subpacket_count, mut bit_stream) = parse_subpacket_count(bit_stream);

            let mut subpackets: Vec<Packet> = vec![];

            for _i in 0..subpacket_count {
                let (subpacket, bit_stream_rem) = parse_packet(bit_stream);
                bit_stream = bit_stream_rem;
                subpackets.push(subpacket);
            }

            return (
                Packet {
                    version,
                    packet_type: type_id,
                    inner_data: Content::Packets(subpackets),
                    length_type_id: length_type_id,
                },
                bit_stream,
            );
        } else {
            panic!("unknown length type id");
        }
    }
}

fn parse_version(mut bit_stream: &[char]) -> (usize, &[char]) {
    let b = bit_stream[..3].iter().collect::<String>();

    (to_hex(&b).parse::<usize>().unwrap(), &bit_stream[3..])
}

fn parse_type_id(mut bit_stream: &[char]) -> (usize, &[char]) {
    let b = bit_stream[..3].iter().collect::<String>();

    (to_hex(&b).parse::<usize>().unwrap(), &bit_stream[3..])
}

fn parse_length_type_id(mut bit_stream: &[char]) -> (usize, &[char]) {
    let b = bit_stream[..1].iter().collect::<String>();

    (to_dec(&b) as usize, &bit_stream[1..])
}

fn parse_subpacket_bit_length(mut bit_stream: &[char]) -> (usize, &[char]) {
    let b = bit_stream[..15].iter().collect::<String>();

    (to_dec(&b) as usize, &bit_stream[15..])
}

fn parse_subpacket_count(mut bit_stream: &[char]) -> (usize, &[char]) {
    let b = bit_stream[..11].iter().collect::<String>();

    (to_dec(&b), &bit_stream[11..])
}

fn to_hex(val: &str) -> String {
    let n: u32 = u32::from_str_radix(val, 2).unwrap();
    format!("{:01$x}", n, 0)
}

fn to_dec(val: &str) -> usize {
    usize::from_str_radix(val, 2).unwrap()
}
