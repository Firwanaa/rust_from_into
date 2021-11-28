enum Status {
    Broken(u8),
    Working,
}

impl From<u8> for Status {
    fn from(code: u8) -> Self {
        match code {
            0 => Status::Working,
            c => Status::Broken(code),
        }
    }
}

fn legacy_interface() -> u8 {
    5
}

fn main() {
    let status: Status = legacy_interface().into();
    let status = Status::from(legacy_interface());
}
