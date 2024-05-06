use std::{time::SystemTime, io};
use crate::r#enum::{Color, Type};

pub struct Cpu {
  cpuType: Type,
  cpuColor: Color,
}

impl Cpu {
  pub fn new() -> Self {
    Cpu {
      cpuType: Type::Human,
      cpuColor: Color::Black,
    }
  }

  pub fn  changeColor (&mut self, color: Color) {
    self.cpuColor = color
  }

  pub fn cpuInput(n: usize) -> usize {
    let timestamp = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .expect("Failed to obtain timestamp")
        .as_nanos();
    (timestamp as usize % n) + 1
  }
}
