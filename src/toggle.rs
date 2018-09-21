#[derive(Clone, Copy)]
pub enum Toggle {
	On, Off
}

impl Toggle {
    pub fn to_bool(self) -> bool {
        match self {
            Toggle::On => true,
            Toggle::Off => false,
        }
    }
}
