const MAX_LOCAL_PART_LENGTH: u32 = 64;
const MAX_DOMAIN_LENGTH: u32 = 255;
const CHARACTERS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890";
const SYMBOLS: &str = "!#$%&'*+-/=?^_`{|}~";
const HEX_CHARACTERS: &str = "0123456789abcdef";

fn clear_quotes(input: &str) -> String {
    let mut quote_count = input.starts_with('"') as u32;
    let mut new_local_part: String = String::from(input.chars().next().unwrap());
    for i in 1..input.len() {
        if input.chars().nth(i).unwrap() == '"' && input.chars().nth(i - 1).unwrap() != '\\' {
            quote_count += 1;

            if !new_local_part.starts_with('"') && quote_count != 1 {
                new_local_part.push('"');
            }
        }

        if quote_count % 2 == 0 {
            new_local_part.push(input.chars().nth(i).unwrap());
        }
    }

    new_local_part
}

fn has_bad_dots(part: &str) -> bool {
    part.starts_with('.') || part.ends_with('.') || part.contains("..")
}

fn contains_legal_local_characters(local_part: &str) -> bool {
    for item in local_part.chars() {
        if !CHARACTERS.contains(item) && !SYMBOLS.contains(item) && item != '.' && item != '"' {
            return false;
        }
    }
    true
}

fn contains_legal_domain_characters(domain: &str) -> bool {
    for item in domain.chars() {
        if !CHARACTERS.contains(item) && item != '-' && item != '.' {
            return false;
        }
    }
    true
}

fn is_valid_quotes(part: &str) -> bool {
    for i in 1..part.len() {
        if part.chars().nth(i - 1).unwrap() == '"' && part.chars().nth(i).unwrap() == '"' {
            let proceeding_quote = part.chars().nth(i + 1);
            if proceeding_quote.is_some() && proceeding_quote.unwrap() != '.' {
                return false;
            }
        }
    }
    true
}

fn is_ipv4(ip: &str) -> bool {
    let parts = ip
        .split('.')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();
    if parts.len() != 4 {
        return false;
    }

    for value in parts {
        let value = value.parse::<u32>();
        if value.is_err() || value.unwrap() > 255 {
            return false;
        }
    }

    true
}

fn is_ipv6(ip: &str) -> bool {
    let parts = ip
        .split(':')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    // we check for the length 9 here as the IPv6 counts as one of the parts
    if parts.len() != 9 || parts.first().unwrap() != "IPv6" {
        return false;
    }

    for value in &parts[1..parts.len()] {
        if value.is_empty() || value.len() > 4 {
            return false;
        }

        for chr in value.chars() {
            if !HEX_CHARACTERS.contains(chr.to_ascii_lowercase()) {
                return false;
            }
        }
    }

    true
}

pub fn is_valid_ip(domain: &str) -> bool {
    is_ipv4(domain) || is_ipv6(domain)
}

fn is_valid_local_part(local_part: &str) -> bool {
    if local_part.len() > MAX_LOCAL_PART_LENGTH as usize {
        return false;
    }

    if !is_valid_quotes(local_part) {
        return false;
    }

    if !contains_legal_local_characters(local_part) {
        return false;
    }

    if has_bad_dots(local_part) {
        return false;
    }

    true
}

fn is_valid_domain(domain: &str) -> bool {
    if domain.starts_with('[') && domain.ends_with(']') {
        return is_valid_ip(&domain[1..domain.len() - 1]);
    }

    if domain.len() > MAX_DOMAIN_LENGTH as usize {
        return false;
    }

    if !contains_legal_domain_characters(domain) {
        return false;
    }

    if has_bad_dots(domain) {
        return false;
    }

    if domain.starts_with('[') || domain.ends_with(']') {
        return false;
    }

    if domain.starts_with('-') || domain.ends_with('-') {
        return false;
    }

    true
}

/// Follows email address rules as listed:
/// https://en.wikipedia.org/wiki/Email_address#Examples
pub fn is_valid_email_address(input_email_address: &str) -> bool {
    let email_address = clear_quotes(input_email_address);

    let parts: Vec<String> = email_address
        .split('@')
        .map(|x| x.to_string())
        .collect::<Vec<String>>();

    // (1)  ensure there is only one '@' symbol in the address
    if parts.len() != 2 {
        return false;
    }

    let (local_part, domain): (String, String) = (parts[0].clone(), parts[1].clone());

    if !is_valid_local_part(&local_part) {
        return false;
    }

    if !is_valid_domain(&domain) {
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use crate::string::is_valid_email_address::is_valid_email_address;

    macro_rules! test_is_valid_email_address {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (s, expected) = $inputs;
                assert_eq!(is_valid_email_address(s), expected);
            }
        )*
        }
    }

    macro_rules! test_is_ipv4 {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (s, expected) = $inputs;
                assert_eq!(crate::string::is_valid_email_address::is_ipv4(s), expected);
            }
        )*
        }
    }

    macro_rules! test_is_ipv6 {
        ($($name:ident: $inputs:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (s, expected) = $inputs;
                assert_eq!(crate::string::is_valid_email_address::is_ipv6(s), expected);
            }
        )*
        }
    }

    test_is_valid_email_address! {
        basic: ("simple@example.com", true),
        basic_2: ("very.common@example.com", true),
        cases: ("FirstName.LastName@EasierReading.org", true),
        one_letter_local: ("x@example.com", true),
        long_email_subdomains: ("long.email-address-with-hyphens@and.subdomains.example.com", true),
        tags: ("user.name+tag+sorting@example.com", true),
        slashes: ("name/surname@example.com", true),
        no_tld: ("admin@example", true),
        tld: ("example@s.example", true),
        quotes_with_space: ("\" \"@example.org", true),
        quoted_double_dot: ("\"john..doe\"@example.org", true),
        host_route: ("mailhost!username@example.org", true),
        quoted_non_letters: (r#""very.(),:;<>[]\".VERY.\"very@\\ \"very\".unusual"@strange.example.com"#, true),
        percent_symbol: ("user%example.com@example.org", true),
        local_end_symbol: ("user-@example.org", true),
        ip_address: ("postmaster@[123.123.123.123]", true),
        ip_address_2: ("postmaster@[255.255.255.255]", true),
        other_ip: ("postmaster@[IPv6:2001:0db8:85a3:0000:0000:8a2e:0370:7334]", true),
        begin_with_underscore: ("_test@[IPv6:2001:0db8:85a3:0000:0000:8a2e:0370:7334]", true),
        valid_ipv6: ("example@[IPv6:2001:db8:3333:4444:5555:6666:7777:8888]", true),
        small_ipv6: ("test@[IPv6:0:0:0:0:0:0:0:0]", true),

        no_closing_bracket: ("postmaster@[", false),
        empty_brackets: ("example@[]", false),
        another_invalid_example: ("test@[1234]", false),
        empty_parts: ("x@[IPv6:1000:1000:1000:1000:1000:1000::1000]", false),
        wrong_ip_address: ("postmaster@[1234.123.123.123]", false),
        too_long_ipv4: ("wrong.ip@[123.123.123.123.123.123.123.123]", false),
        missing_closing: ("example@[1.1.1.1", false),
        missing_closing_ipv6: ("test@[IPv6:1000:1000:1000:1000:1000:1000:1000:1000", false),
        no_ipv6_at_start: ("test@[1234:2001:0db8:85a3:0000:0000:8a2e:0370:7334]", false),
        too_long_ipv6: ("test@[IPv6:1234:2001:0db8:85a3:0000:0000:8a2e:0370:7334", false),
        invalid_ipv4: ("example@[123.123.123.123.123]", false),
        bad_ip_address: ("postmaster@[hello.255.255.255]", false),
        barely_invalid: ("example@[255.255.255.256]", false),
        no_at: ("abc.example.com", false),
        multiple_ats: ("a@b@c@example.com", false),
        bad_local_characters: ("a\"b(c)d,e:f;g<h>i[j\\k]l@example.com", false),
        bad_local_string: ("just\"not\"right@example.com", false),
        bad_backslash: ("this is\"not\\allowed@example.com", false),
        escaped_backslash: ("this\\ still\\\"not\\\\allowed@example.com", false),
        long_local_part: ("1234567890123456789012345678901234567890123456789012345678901234+x@example.com", false),
        domain_underscore: ("i.like.underscores@but_they_are_not_allowed_in_this_part", false),
    }

    test_is_ipv4! {
        standard: ("100.100.100.100", true),
        two_digit: ("10.10.10.10", true),
        one_digit: ("9.9.9.9", true),
        extreme_high: ("255.255.255.255", true),
        extreme_low: ("0.0.0.0", true),
        mixed_length: ("255.10.0.125", true),

        invalid: ("255.255.255.256", false),
        missing_part: ("123.123.123", false),
        empty_part: ("123.123.123.", false),
        empty_begin: (".2.21.25", false),
        invalid_characters: ("123.123.123.a", false),
        too_long: ("123.123.123.1234", false),
        no_dots: ("123", false),
        null_ipv4: ("", false),
    }

    test_is_ipv6! {
        regular: ("IPv6:1000:1000:1000:1000:1000:1000:1000:1000", true),
        mixed_lengths: ("IPv6:2001:db8:3333:4444:5555:6666:7777:8888", true),
        short: ("IPv6:0:0:0:0:0:0:0:0", true),

        bad_case: ("Ipv6:1000:1000:1000:1000:1000:1000:1000:1000", false),
        no_starting_ivp6: ("1234:2001:0db8:85a3:0000:0000:8a2e:0370:7334", false),
        invalid_length: ("IPv6:1:1:1:1:1:1:1:12345", false),
        empty_value: ("IPv6:100:100:100:100::100:100:100", false),
        invalid_character: ("IPv6:100:100:100:100g:100:100:100:100", false),
        no_colons: ("IPv6", false),
        null: ("", false),
    }
}
