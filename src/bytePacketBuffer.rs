const PACKET_TRANSPORT_LIMIT: u8 = 512;

pub struct BytePacketBuffer {
    pub buffer:   [u8; PACKET_TRANSPORT_LIMIT],
    pub position: usize
};


impl BytePacketBuffer {
    pub fn new() -> BytePacketBuffer {
        BytePacketBuffer {
            buffer:   [0; PACKET_TRANSPORT_LIMIT],
            position:  0
        }
    }


    fn position(&self) -> usize {
        self.position
    }


    fn step(&mut self, steps: usize) -> Result<()> {
        self.position = steps;
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

        Ok(self.buffer[self.position])
    }


    fn get_range(&mut self, start: usize, length: usize) -> Result<&[u8]> {
        if start + length >= PACKET_TRANSPORT_LIMIT {
            return Err("End of buffer".into());
        }

        let result = &self.buffer[start..start + length as usize];

        Ok(result)
    }


    fn read_u16(&mut self) -> Result<u16> {
        let result = ((self.read()? as u16) << 8) | (self.read()? as u16);
        Ok(result)
    }


    fn read_u32(&mut self) -> Result<u32> {
        let result = ((self.read()? as u32) << 24)
            | ((self.read()? as u32) << 16)
            | ((self.read()? as u32) << 8)
            | ((self.read()? as u32) << 0);
        Ok(result)
    }


    fn read_qname(&mut self, output_str: &mut String) -> Result<()> {
        let mut position = self.position();

        let mut jumped = false;
        let mut jumps_performed = 0;
        let max_jumps = 5;

        // Delimitador que será anexado em cada label.
        let mut delim = "";

        loop {
            if jumps_performed > max_jumps {
                return Err(format!("Limit of {} jumps exceeded", max_jumps).into());
            }

            let length = self.get(position);

            if (length & 0xC0) == 0xC0 {
                unimplemented!();
            }
        }
    }
}
