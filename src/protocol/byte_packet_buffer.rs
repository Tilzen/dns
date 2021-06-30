use super::types::Result;

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

    pub fn position(&self) -> usize {
        self.position
    }

    pub fn step(&mut self, steps: usize) -> Result<()> {
        self.position += steps;
        Ok(())
    }

    fn seek(&mut self, position: usize) -> Result<()> {
        self.position = position;
        Ok(())
    }

    fn read(&mut self) -> Result<u8> {
        if self.position >= PACKET_TRANSPORT_LIMIT {
            return Err("End of buffer".into());
        }

        let result = self.buffer[self.position];
        self.position += 1;

        Ok(result)
    }

    fn get(&mut self, position: usize) -> Result<u8> {
        if position >= PACKET_TRANSPORT_LIMIT {
            return Err("End of buffer".into());
        }

        Ok(self.buffer[position])
    }

    fn get_range(&mut self, start: usize, length: usize) -> Result<&[u8]> {
        if start + length >= PACKET_TRANSPORT_LIMIT {
            return Err("End of buffer".into());
        }

        let result = &self.buffer[start..start + length as usize];

        Ok(result)
    }

    pub fn read_u16(&mut self) -> Result<u16> {
        let result = ((self.read()? as u16) << 8) | (self.read()? as u16);
        Ok(result)
    }

    pub fn read_u32(&mut self) -> Result<u32> {
        let result = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);
        Ok(result)
    }

    pub fn read_qname(&mut self, output_str: &mut String) -> Result<()> {
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

    fn write(&mut self, value: u8) -> Result<()> {
        if self.position >= PACKET_TRANSPORT_LIMIT {
            return Err("End of buffer".into());
        }

        self.buffer[self.position] = value;
        self.position += 1;
        Ok(())
    }

    pub fn write_u8(&mut self, value: u8) -> Result<()> {
        self.write(value)?;
        Ok(())
    }

    pub fn write_u16(&mut self, value: u16) -> Result<()> {
        self.write((value >> 8) as u8)?;
        self.write((value & 0xFF) as u8)?;
        Ok(())
    }

    pub fn write_u32(&mut self, value: u32) -> Result<()> {
        self.write(((value >> 24) & 0xFF) as u8)?;
        self.write(((value >> 16) & 0xFF) as u8)?;
        self.write(((value >> 8) & 0xFF) as u8)?;
        self.write(((value >> 0) & 0xFF) as u8)?;
        Ok(())
    }

    pub fn write_qname(&mut self, qname: &str) -> Result<()> {
        for label in qname.split('.') {
            let len = label.len();
            if len > 0x3f {
                return Err("Single label exceeds 63 characters of length".into());
            }

            self.write_u8(len as u8)?;
            for b in label.as_bytes() {
                self.write_u8(*b)?;
            }
        }

        self.write_u8(0)?;
        Ok(())
    }
}
