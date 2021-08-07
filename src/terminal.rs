
pub struct Size {
    pub width: u16,
    pub height: u16,
}

pub struct Terminal {
    size: Size
}



impl Terminal {
    
    pub fn default() -> Result<Self, std::io::Error> {
        let size = termion::terminal_size()?;
        
        // size is a tuple
        // size.0 is width, size.1 is height
        Ok(Self {
            size: Size {
                width: size.0,
                height: size.1,
            }
        })
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}
