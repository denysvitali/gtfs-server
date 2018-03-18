#[derive(Debug, Serialize, FromPrimitive, ToPrimitive)]
pub enum DropOff {
    RegularlyScheduled = 0,
    NotAvailable = 1,
    MustArrangeWithAgency = 2,
    MustCoordinateWithDriver = 3
}
