/// Module for converting between IPv4 addresses and their decimal representations
///
/// This module provides functions to convert IPv4 addresses to decimal integers
/// and vice versa.
///
/// Reference: https://www.geeksforgeeks.org/convert-ip-address-to-integer-and-vice-versa/
use std::num::ParseIntError;

/// Errors that can occur during IPv4 address conversion
#[derive(Debug, PartialEq)]
pub enum Ipv4Error {
    /// The IPv4 address does not have exactly 4 octets
    InvalidFormat,
    /// An octet value is greater than 255
    InvalidOctet(u32),
    /// The decimal value is out of valid range
    InvalidDecimal,
    /// Failed to parse an octet as a number
    ParseError,
}

impl std::fmt::Display for Ipv4Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Ipv4Error::InvalidFormat => write!(f, "Invalid IPv4 address format"),
            Ipv4Error::InvalidOctet(octet) => write!(f, "Invalid IPv4 octet {octet}"),
            Ipv4Error::InvalidDecimal => write!(f, "Invalid decimal IPv4 address"),
            Ipv4Error::ParseError => write!(f, "Failed to parse octet"),
        }
    }
}

impl std::error::Error for Ipv4Error {}

impl From<ParseIntError> for Ipv4Error {
    fn from(_: ParseIntError) -> Self {
        Ipv4Error::ParseError
    }
}

/// Convert an IPv4 address to its decimal representation.
///
/// The conversion is performed by treating each octet as 8 bits and combining
/// them into a 32-bit unsigned integer.
///
/// # Arguments
///
/// * `ipv4_address` - A string slice representing an IPv4 address (e.g., "192.168.0.1")
///
/// # Returns
///
/// * `Ok(u32)` - The decimal representation of the IP address
/// * `Err(Ipv4Error)` - If the input format is invalid or contains invalid octets
pub fn ipv4_to_decimal(ipv4_address: &str) -> Result<u32, Ipv4Error> {
    let octets: Vec<&str> = ipv4_address.split('.').collect();

    if octets.len() != 4 {
        return Err(Ipv4Error::InvalidFormat);
    }

    let mut decimal_ipv4: u32 = 0;

    for octet_str in octets {
        let octet: u32 = octet_str.parse()?;

        if octet > 255 {
            return Err(Ipv4Error::InvalidOctet(octet));
        }

        decimal_ipv4 = (decimal_ipv4 << 8) + octet;
    }

    Ok(decimal_ipv4)
}

/// Alternative implementation to convert an IPv4 address to its decimal representation
/// using hexadecimal conversion.
///
/// This function converts each octet to a two-digit hexadecimal string, concatenates
/// them, and then parses the result as a hexadecimal number.
///
/// # Arguments
///
/// * `ipv4_address` - A string slice representing an IPv4 address
///
/// # Returns
///
/// * `Ok(u32)` - The decimal representation of the IP address
/// * `Err(Ipv4Error)` - If the input is invalid
pub fn alt_ipv4_to_decimal(ipv4_address: &str) -> Result<u32, Ipv4Error> {
    let octets: Vec<&str> = ipv4_address.split('.').collect();

    if octets.len() != 4 {
        return Err(Ipv4Error::InvalidFormat);
    }

    let hex_string: String = octets
        .iter()
        .map(|octet| {
            let num: u32 = octet.parse().map_err(|_| Ipv4Error::ParseError)?;
            if num > 255 {
                return Err(Ipv4Error::InvalidOctet(num));
            }
            Ok(format!("{num:02x}"))
        })
        .collect::<Result<Vec<String>, Ipv4Error>>()?
        .join("");

    u32::from_str_radix(&hex_string, 16).map_err(|_| Ipv4Error::ParseError)
}

/// Convert a decimal representation of an IP address to its IPv4 format.
///
/// The conversion extracts each octet by masking the lower 8 bits and right-shifting.
///
/// # Arguments
///
/// * `decimal_ipv4` - An unsigned 32-bit integer representing the decimal IP address
///
/// # Returns
///
/// * `Ok(String)` - The IPv4 representation of the decimal IP address
pub fn decimal_to_ipv4(decimal_ipv4: u32) -> Result<String, Ipv4Error> {
    let mut ip_parts = Vec::new();
    let mut num = decimal_ipv4;

    for _ in 0..4 {
        ip_parts.push((num & 255).to_string());
        num >>= 8;
    }

    ip_parts.reverse();
    Ok(ip_parts.join("."))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ipv4_to_decimal_valid() {
        assert_eq!(ipv4_to_decimal("192.168.0.1").unwrap(), 3232235521);
        assert_eq!(ipv4_to_decimal("10.0.0.255").unwrap(), 167772415);
        assert_eq!(ipv4_to_decimal("0.0.0.0").unwrap(), 0);
        assert_eq!(ipv4_to_decimal("255.255.255.255").unwrap(), 4294967295);
        assert_eq!(ipv4_to_decimal("8.8.8.8").unwrap(), 134744072);
    }

    #[test]
    fn test_ipv4_to_decimal_invalid_format() {
        assert_eq!(ipv4_to_decimal("10.0.255"), Err(Ipv4Error::InvalidFormat));
        assert_eq!(ipv4_to_decimal("10.0.0.0.1"), Err(Ipv4Error::InvalidFormat));
        assert_eq!(ipv4_to_decimal(""), Err(Ipv4Error::InvalidFormat));
        assert_eq!(ipv4_to_decimal("192.168.0"), Err(Ipv4Error::InvalidFormat));
    }

    #[test]
    fn test_ipv4_to_decimal_invalid_octet() {
        assert_eq!(
            ipv4_to_decimal("10.0.0.256"),
            Err(Ipv4Error::InvalidOctet(256))
        );
        assert_eq!(
            ipv4_to_decimal("300.168.0.1"),
            Err(Ipv4Error::InvalidOctet(300))
        );
        assert_eq!(
            ipv4_to_decimal("192.168.256.1"),
            Err(Ipv4Error::InvalidOctet(256))
        );
    }

    #[test]
    fn test_ipv4_to_decimal_parse_error() {
        assert_eq!(ipv4_to_decimal("192.168.0.abc"), Err(Ipv4Error::ParseError));
        assert_eq!(ipv4_to_decimal("a.b.c.d"), Err(Ipv4Error::ParseError));
    }

    #[test]
    fn test_alt_ipv4_to_decimal_valid() {
        assert_eq!(alt_ipv4_to_decimal("192.168.0.1").unwrap(), 3232235521);
        assert_eq!(alt_ipv4_to_decimal("10.0.0.255").unwrap(), 167772415);
        assert_eq!(alt_ipv4_to_decimal("0.0.0.0").unwrap(), 0);
        assert_eq!(alt_ipv4_to_decimal("255.255.255.255").unwrap(), 4294967295);
    }

    #[test]
    fn test_alt_ipv4_to_decimal_invalid() {
        assert_eq!(
            alt_ipv4_to_decimal("10.0.255"),
            Err(Ipv4Error::InvalidFormat)
        );
        assert_eq!(
            alt_ipv4_to_decimal("10.0.0.256"),
            Err(Ipv4Error::InvalidOctet(256))
        );
    }

    #[test]
    fn test_decimal_to_ipv4_valid() {
        assert_eq!(decimal_to_ipv4(3232235521).unwrap(), "192.168.0.1");
        assert_eq!(decimal_to_ipv4(167772415).unwrap(), "10.0.0.255");
        assert_eq!(decimal_to_ipv4(0).unwrap(), "0.0.0.0");
        assert_eq!(decimal_to_ipv4(4294967295).unwrap(), "255.255.255.255");
        assert_eq!(decimal_to_ipv4(134744072).unwrap(), "8.8.8.8");
        assert_eq!(decimal_to_ipv4(2886794752).unwrap(), "172.16.254.0");
    }

    #[test]
    fn test_round_trip_conversion() {
        let test_addresses = vec![
            "192.168.0.1",
            "10.0.0.255",
            "172.16.254.1",
            "8.8.8.8",
            "255.255.255.255",
            "0.0.0.0",
            "127.0.0.1",
            "1.2.3.4",
        ];

        for addr in test_addresses {
            let decimal = ipv4_to_decimal(addr).unwrap();
            let result = decimal_to_ipv4(decimal).unwrap();
            assert_eq!(addr, result, "Round trip failed for {addr}");
        }
    }

    #[test]
    fn test_both_methods_agree() {
        let test_addresses = vec![
            "192.168.0.1",
            "10.0.0.255",
            "172.16.254.1",
            "8.8.8.8",
            "255.255.255.255",
            "0.0.0.0",
        ];

        for addr in test_addresses {
            let result1 = ipv4_to_decimal(addr).unwrap();
            let result2 = alt_ipv4_to_decimal(addr).unwrap();
            assert_eq!(result1, result2, "Methods disagree for address: {addr}");
        }
    }

    #[test]
    fn test_edge_cases() {
        // All zeros
        assert_eq!(ipv4_to_decimal("0.0.0.0").unwrap(), 0);
        assert_eq!(decimal_to_ipv4(0).unwrap(), "0.0.0.0");

        // All 255s (max value)
        assert_eq!(ipv4_to_decimal("255.255.255.255").unwrap(), 4294967295);
        assert_eq!(decimal_to_ipv4(4294967295).unwrap(), "255.255.255.255");

        // Common private ranges
        assert_eq!(ipv4_to_decimal("10.0.0.0").unwrap(), 167772160);
        assert_eq!(ipv4_to_decimal("172.16.0.0").unwrap(), 2886729728);
        assert_eq!(ipv4_to_decimal("192.168.0.0").unwrap(), 3232235520);
    }
}
