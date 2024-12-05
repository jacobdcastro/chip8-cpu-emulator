use std::fs::File;
use std::io::{self, Read};
use std::path::Path;

pub struct RomLoader;

impl RomLoader {
    // load a ROM file (either binary or text assembly)
    pub fn load(path: &Path) -> io::Result<Vec<u8>> {
        let mut file = File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;

        // check if file is text-based assembly
        if contents.contains(';') || contents.trim().starts_with("00E0") {
            Ok(Self::parse_assembly(&contents))
        } else {
            // handle binary ROM
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer)?;
            Ok(buffer)
        }
    }

    // parse text-based assembly into binary
    fn parse_assembly(contents: &str) -> Vec<u8> {
        let mut binary = Vec::new();

        for line in contents.lines() {
            // skip empty lines and comments
            let line = line.trim();
            if line.is_empty() || line.starts_with(';') {
                continue;
            }

            // extract the opcode part (before any comment or label)
            let opcode_str = line
                .split(';')
                .next()
                .unwrap()
                .split(':')
                .last()
                .unwrap()
                .trim();

            // skip if no opcode found
            if opcode_str.is_empty() {
                continue;
            }

            // parse the entire opcode as a single 16-bit value
            if let Some((high, low)) = Self::opcode_to_bytes(opcode_str) {
                binary.push(high);
                binary.push(low);
            }
        }

        binary
    }

    // helper function to convert a single opcode to bytes
    fn opcode_to_bytes(opcode_str: &str) -> Option<(u8, u8)> {
        // Remove any whitespace and get just the hex digits
        let opcode = opcode_str
            .chars()
            .filter(|c| !c.is_whitespace())
            .collect::<String>();

        // Ensure we have exactly 4 hex digits
        if opcode.len() != 4 {
            return None;
        }

        // Parse as a single 16-bit value then split into high and low bytes
        if let Ok(value) = u16::from_str_radix(&opcode, 16) {
            let high = ((value & 0xFF00) >> 8) as u8;
            let low = (value & 0x00FF) as u8;
            Some((high, low))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_assembly() {
        let input = "
            00E0                ; Clear the screen
            A200                ; Load sprite address
            6100                ; Set V1 = 0
            
            ; Sprite data
            F0808080F0          ; Sprite for 'C'
        ";

        let binary = RomLoader::parse_assembly(input);
        assert_eq!(
            binary,
            vec![0x00, 0xE0, 0xA2, 0x00, 0x61, 0x00, 0xF0, 0x80, 0x80, 0x80, 0xF0]
        );
    }

    #[test]
    fn test_opcode_to_bytes() {
        assert_eq!(RomLoader::opcode_to_bytes("00E0"), Some((0x00, 0xE0)));
        assert_eq!(RomLoader::opcode_to_bytes("A200"), Some((0xA2, 0x00)));
        assert_eq!(RomLoader::opcode_to_bytes("invalid"), None);
    }
}
