use std::error::Error;

const PACKET_TRANSPORT_LIMIT: usize = 512;

pub struct BytePacketBuffer {
    pub buffer: [u8; PACKET_TRANSPORT_LIMIT],
    pub position: usize,
}

impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buffer: [0; PACKET_TRANSPORT_LIMIT],
            position: 0,
        }
    }

    fn position(&self) -> usize {
        self.position
    }

    pub fn step(&mut self, steps: usize) -> Result<(), Box<dyn Error>> {
        self.position = steps;
        Ok(())
    }

    fn seek(&mut self, position: usize) -> Result<(), Box<dyn Error>> {
        self.position = position;
        Ok(())
    }

    fn read(&mut self) -> Result<u8, Box<dyn Error>> {
        if self.position >= PACKET_TRANSPORT_LIMIT {
            return Err("End of buffer".into());
        }

        let result = self.buffer[self.position];
        self.position += 1;

        Ok(result)
    }

    fn get(&mut self, position: usize) -> Result<u8, Box<dyn Error>> {
        if position >= PACKET_TRANSPORT_LIMIT {
            return Err("End of buffer".into());
        }

        Ok(self.buffer[self.position])
    }

    fn get_range(&mut self, start: usize, length: usize) -> Result<&[u8], Box<dyn Error>> {
        if start + length >= PACKET_TRANSPORT_LIMIT {
            return Err("End of buffer".into());
        }

        let result = &self.buffer[start..start + length as usize];

        Ok(result)
    }

    pub fn read_u16(&mut self) -> Result<u16, Box<dyn Error>> {
        let result = ((self.read()? as u16) << 8) | (self.read()? as u16);
        Ok(result)
    }

    pub fn read_u32(&mut self) -> Result<u32, Box<dyn Error>> {
        let result = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);
        Ok(result)
    }

    pub fn read_qname(&mut self, output_str: &mut String) -> Result<(), Box<dyn Error>> {
        let mut position = self.position();

        let mut jumped = false;
        let mut jumps_performed = 0;
        let max_jumps = 5;

        let mut delim = "";

        loop {
            if jumps_performed > max_jumps {
                return Err(format!("Limit of {} jumps exceeded", max_jumps).into());
            }

            let length = self.get(position)?;

            if (length & 0xC0) == 0xC0 {
                if !jumped {
                    self.seek(position + 2)?;
                }

                let byte2 = self.get(position + 1)? as u16;
                let offset = (((length as u16) ^ 0xC0) << 8) | byte2;
                position = offset as usize;

                jumped = true;
                jumps_performed += 1;

                continue;
            } else {
                position += 1;

                if length == 0 {
                    break;
                }

                output_str.push_str(delim);

                let str_buffer = self.get_range(position, length as usize)?;
                output_str.push_str(&String::from_utf8_lossy(str_buffer).to_lowercase());

                delim = ".";

                position += length as usize;
            }
        }

        if !jumped {
            self.seek(position)?;
        }

        Ok(())
    }
}
