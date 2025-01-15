pub trait PortMappedDevice {
    fn read_port(&mut self, port:u16) -> u8;
    fn write_port(&mut self, port:u16, val:u8);
    fn tick(&mut self, cycles:u32);
}

pub trait MemoryMappedDevice {
    
}
