//! DropOff related enums and implementations

#[derive(Debug, Serialize, FromPrimitive, ToPrimitive)]
pub enum DropOff {
    RegularlyScheduled = 0,
    NotAvailable = 1,
    MustArrangeWithAgency = 2,
    MustCoordinateWithDriver = 3,
}

impl DropOff {
    pub fn from_string(input: &str) -> DropOff {
        match input {
            "RegularlyScheduled" => DropOff::RegularlyScheduled,
            "NotAvailable" => DropOff::NotAvailable,
            "MustArrangeWithAgency" => DropOff::MustArrangeWithAgency,
            "MustCoordinateWithDriver" => DropOff::MustCoordinateWithDriver,
            _ => DropOff::RegularlyScheduled,
        }
    }
}
