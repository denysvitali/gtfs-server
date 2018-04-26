//! PickUp related enums and implementations
#[derive(Debug, Serialize, FromPrimitive, ToPrimitive, Hash, Clone)]
pub enum PickUp {
    RegularlyScheduled = 0,
    NoPickupAvailable = 1,
    MustArrangeWithAgency = 2,
    MustCoordinateWithDriver = 3,
}

impl PickUp {
    pub fn from_string(input: &str) -> PickUp {
        match input {
            "RegularlyScheduled" => PickUp::RegularlyScheduled,
            "NoPickupAvailable" => PickUp::NoPickupAvailable,
            "MustArrangeWithAgency" => PickUp::MustArrangeWithAgency,
            "MustCoordinateWithDriver" => PickUp::MustCoordinateWithDriver,
            _ => PickUp::RegularlyScheduled,
        }
    }
}
