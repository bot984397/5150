pub trait PortMappedDevice {
    fn write_8(&mut self, port:u16, val:u8);
    fn read_8(&mut self, port:u16) -> u8;

    fn ports(&self) -> Vec<u16>;
    fn debug_info(&self) -> String;
}

pub trait Device {
    fn cycle(&mut self);
}
