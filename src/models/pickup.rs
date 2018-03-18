#[derive(Debug, Serialize, FromPrimitive, ToPrimitive)]
pub enum PickUp {
    RegularlyScheduled = 0,
    NoPickupAvailable = 1,
    MustArrangeWithAgency = 2,
    MustCoordinateWithDriver = 3
}
