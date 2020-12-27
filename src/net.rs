/// MAC address struct.  Can be instantiated with `MacAddress::new`.
///
/// This is an EUI-48 MAC address (previously called MAC-48).
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug, Hash, Default)]
pub struct MacAddress {
    /// Octets of the MAC address.
    octets: [u8; 6],
}

impl MacAddress {
    /// Creates a new EUI-48 MAC address from six eight-bit octets.
    ///
    /// The result will represent the EUI-48 MAC address
    /// `a`:`b`:`c`:`d`:`e`:`f`.
    ///
    /// # Examples
    ///
    /// ```
    /// use w5500::net::MacAddress;
    ///
    /// let addr = MacAddress::new(0x00, 0x00, 0x5E, 0x00, 0x00, 0x00);
    /// ```
    ///
    /// Consider using freely available private/locally administered mac
    /// addresses that match the following hex pattern:
    ///
    /// ```code
    ///  x2-xx-xx-xx-xx-xx
    ///  x6-xx-xx-xx-xx-xx
    ///  xA-xx-xx-xx-xx-xx
    ///  xE-xx-xx-xx-xx-xx
    /// ```
    ///
    /// "Universally administered and locally administered addresses are
    /// distinguished by setting the second-least-significant bit of the first
    /// octet of the address"
    /// [Wikipedia](https://en.wikipedia.org/wiki/MAC_address#Universal_vs._local)
    #[allow(clippy::many_single_char_names)]
    pub const fn new(a: u8, b: u8, c: u8, d: u8, e: u8, f: u8) -> MacAddress {
        MacAddress {
            octets: [a, b, c, d, e, f],
        }
    }

    pub fn octets(&self) -> [u8; 6] {
        self.octets
    }

    /// An EUI-48 MAC address representing an unspecified address:
    /// 00:00:00:00:00:00
    ///
    /// # Examples
    ///
    /// ```
    /// use w5500::net::MacAddress;
    ///
    /// let addr = MacAddress::UNSPECIFIED;
    /// assert_eq!(addr, MacAddress::new(0x00, 0x00, 0x00, 0x00, 0x00, 0x00));
    /// ```
    pub const UNSPECIFIED: Self = MacAddress::new(0, 0, 0, 0, 0, 0);
}

impl ::core::fmt::Display for MacAddress {
    /// String formatter for MacAddress addresses.
    fn fmt(&self, fmt: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        write!(
            fmt,
            "{:02X}:{:02X}:{:02X}:{:02X}:{:02X}:{:02X}",
            self.octets[0],
            self.octets[1],
            self.octets[2],
            self.octets[3],
            self.octets[4],
            self.octets[5],
        )
    }
}
