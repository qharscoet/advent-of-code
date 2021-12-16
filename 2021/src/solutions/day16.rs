use crate::solution::Solution;

pub struct Day16;

#[derive(Debug, PartialEq)]
struct LiteralData{
    value : u64
}

#[derive(Debug,PartialEq)]
struct OperatorData{
    length:usize,
    subpackets : Vec<Packet>
}

#[derive(Debug, PartialEq)]
enum PacketData{
    Literal(LiteralData),
    Operator(OperatorData)
}

#[derive(Debug)]
pub struct Packet {
    version:u8,
    type_id: u8,
    length:usize,
    raw_data:String,
    data:PacketData
}


impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        self.version == other.version &&
        self.type_id == other.type_id &&
        self.length == other.length &&
        self.raw_data == other.raw_data &&
        self.data == other.data
    }
}

fn hex_to_bits(hex:char) -> &'static str {
    match hex {
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
        _ => ""
    }
}

impl std::str::FromStr for LiteralData {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chunks= s.as_bytes().chunks(5);

        let value = u64::from_str_radix(&chunks.fold("".to_string(), |acc, bits| {
            acc + &bits[1..5].iter().map(|&v| v as char).collect::<String>()
        }), 2).unwrap_or_default();

        Ok(LiteralData{value:value })
    }
}

impl std::str::FromStr for OperatorData {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut chars = s.chars();
        let mut subpackets : Vec<Packet> = vec![];
        let mut length :usize = 0;
        match chars.next().unwrap() {
            '0' => {
                let total_length = u16::from_str_radix(&chars.by_ref().take(15).collect::<String>(), 2).unwrap_or_default();
                length = 15 + total_length as usize;
                let mut curr_length = 0;
                let mut subpackets_str : String = chars.collect();
                while curr_length != total_length {
                    let packet = subpackets_str.parse::<Packet>().unwrap();
                    subpackets_str = subpackets_str[packet.length..subpackets_str.len()].to_string();
                    curr_length += packet.length as u16;
                    subpackets.push(packet);
                }
            },
            '1' => {
                let nb_subpackets = u16::from_str_radix(&chars.by_ref().take(11).collect::<String>(), 2).unwrap_or_default();
                let mut subpackets_str : String = chars.collect();
                length += 11;
                for _ in 0..nb_subpackets {
                    let packet = subpackets_str.parse::<Packet>().unwrap();
                    subpackets_str = subpackets_str[packet.length..subpackets_str.len()].to_string();
                    length += packet.length;
                    subpackets.push(packet);
                }
            },
            _ => {}
        }

        length += 1; //adding the bit for the length type
        Ok(OperatorData{length: length, subpackets:subpackets})
    }
}


impl std::str::FromStr for Packet {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        let mut bits = s.chars();

        // let version = u8::from_str_radix(&bits.by_ref().take(3).collect::<String>(), 2).expect("invalid binary");
        let version = match u8::from_str_radix(&bits.by_ref().take(3).collect::<String>(), 2) {
            Err(_e) => return Err("invalid binary"),
            Ok(data) => data
        };

        let type_id = match u8::from_str_radix(&bits.by_ref().take(3).collect::<String>(), 2) {
            Err(_e) => return Err("invalid binary"),
            Ok(data) => data
        };

        let mut raw_data : String = bits.collect();
        let mut length = 0;

        let data : PacketData = match type_id {
            4 => {
                let nb_chunks = raw_data.chars().step_by(5).take_while(|&c| c == '1' ).count() + 1;
                let value_str = raw_data.chars().take(nb_chunks * 5).collect::<String>();
                length = nb_chunks*5;
                PacketData::Literal(value_str.parse::<LiteralData>().unwrap())},
            _ => {
                let operator_data = raw_data.parse::<OperatorData>().unwrap();
                length = operator_data.length;
                PacketData::Operator(operator_data)
            }
        };
        
        raw_data = raw_data[0..length].to_string();
        length += 6; // Adding header length

        Ok(Packet {version:version, type_id:type_id, length:length, raw_data: raw_data, data: data})
    }
}


impl Solution for Day16 {
    type Input = Packet;
    type ReturnType = u64;

    fn parse_input(&self, mut lines: impl Iterator<Item = std::string::String>) -> Self::Input {
        let hex= lines.next().unwrap_or_default();
        let binary =  hex.chars().fold("".to_string(), |acc, hex_digit| acc + hex_to_bits(hex_digit));

        binary.parse().unwrap()
    }

    fn first_part(&self, input: &Self::Input) -> u64 {
        let packet = input.clone();

        fn get_packet_version_sum(packet:&Packet) -> u64 {
            match &packet.data {
                PacketData::Literal(_) => {packet.version as u64},
                PacketData::Operator(data) => {
                    packet.version as u64 + data.subpackets.iter().fold(0, |acc, subpacket| acc + get_packet_version_sum(subpacket))
                }
            }
        }
        
        get_packet_version_sum(packet)
    }

    fn second_part(&self, input: &Self::Input) -> u64 {
        let packet = input.clone();

        fn get_packet_value(packet:&Packet) -> u64 {
            match packet.type_id {
                4 => if let PacketData::Literal(data) = &packet.data {data.value} else {0}
                _ => {
                    if let PacketData::Operator(data) = &packet.data {
                        let subpacket_values: Vec<u64> = data.subpackets.iter().map(|subpacket| get_packet_value(subpacket)).collect();
                        match packet.type_id {
                            0 => subpacket_values.iter().sum(),
                            1 => subpacket_values.iter().product(),
                            2 => *subpacket_values.iter().min().unwrap(),
                            3 => *subpacket_values.iter().max().unwrap(),
                            5 => (subpacket_values[0] > subpacket_values[1]) as u64,
                            6 => (subpacket_values[0] < subpacket_values[1]) as u64,
                            7 => (subpacket_values[0] == subpacket_values[1]) as u64,
                            _ => 0,
                        }
                    } else {
                        0
                    }
                }
            }
        }
       get_packet_value(packet)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solution::Solution;

    #[test]
    fn test_parse_literal() {
        let input = vec!["D2FE28".to_string()];
        let expected = Packet {
            version: 6,
            type_id: 4,
            length: 21,
            raw_data: "101111111000101".to_string(),
            data: PacketData::Literal(LiteralData { value: 2021 }),
        };

        let result = Day16.parse_input(input.into_iter());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_operator_type0() {
        let input = vec!["38006F45291200".to_string()];
        let expected = Packet {
            version: 1,
            type_id: 6,
            length: 49,
            raw_data: "0000000000011011110100010100101001000100100".to_string(),
            data: PacketData::Operator(OperatorData { length: 43,subpackets: vec![
                Packet {
                version: 6,
                type_id: 4,
                length: 11,
                raw_data: "01010".to_string(),
                data: PacketData::Literal(LiteralData { value: 10 }),
            },
            Packet {
                version: 2,
                type_id: 4,
                length: 16,
                raw_data: "1000100100".to_string(),
                data: PacketData::Literal(LiteralData { value: 20 }),
            }] }),
        };

        let result = Day16.parse_input(input.into_iter());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_operator_type1() {
        let input = vec!["EE00D40C823060".to_string()];
        let expected = Packet {
            version: 7,
            type_id: 3,
            length: 51,
            raw_data: "100000000011010100000011001000001000110000011".to_string(),
            data: PacketData::Operator(OperatorData { length: 45,subpackets: vec![
                Packet {
                version: 2,
                type_id: 4,
                length: 11,
                raw_data: "00001".to_string(),
                data: PacketData::Literal(LiteralData { value: 1 }),
            },
            Packet {
                version: 4,
                type_id: 4,
                length: 11,
                raw_data: "00010".to_string(),
                data: PacketData::Literal(LiteralData { value: 2 }),
            },
            Packet {
                version: 1,
                type_id: 4,
                length: 11,
                raw_data: "00011".to_string(),
                data: PacketData::Literal(LiteralData { value: 3 }),
            }] }),
        };

        let result = Day16.parse_input(input.into_iter());
        assert_eq!(result, expected);
    }

    #[test]
    fn test_first_part() {
        let test_values = vec![
            vec!["D2FE28".to_string()],
            vec!["38006F45291200".to_string()],
            vec!["8A004A801A8002F478".to_string()],
            vec!["620080001611562C8802118E34".to_string()],
            vec!["C0015000016115A2E0802F182340".to_string()],
            vec!["A0016C880162017C3686B18A3D4780".to_string()]
        ];
        let test_results : Vec<<crate::solutions::day16::Day16 as Solution>::ReturnType> = vec![6, 9,16,12,23,31];

        for (value, result) in test_values.into_iter().zip(test_results.iter()) {
            let input = Day16.parse_input(value.into_iter());
            assert_eq!(Day16.first_part(&input), *result)
        }
    }

    #[test]
    fn test_second_part() {
        let test_values = vec![
            vec!["C200B40A82".to_string()],
            vec!["04005AC33890".to_string()],
            vec!["880086C3E88112".to_string()],
            vec!["CE00C43D881120".to_string()],
            vec!["D8005AC2A8F0".to_string()],
            vec!["F600BC2D8F".to_string()],
            vec!["9C005AC2F8F0".to_string()],
            vec!["9C0141080250320F1802104A08".to_string()],
        ];
        let test_results : Vec<<crate::solutions::day16::Day16 as Solution>::ReturnType> = vec![3, 54, 7,9,1,0, 0, 1];

        for (value, result) in test_values.into_iter().zip(test_results.iter()) {
            println!("{:?} VS {:?}", value, result);
            let input = Day16.parse_input(value.into_iter());
            assert_eq!(Day16.second_part(&input), *result)
        }
    }
}
