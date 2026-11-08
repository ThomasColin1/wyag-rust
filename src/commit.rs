use crate::repo::WyagError;
use std::collections::HashMap;

pub type Kvlm = HashMap<Option<Vec<u8>>, Vec<Vec<u8>>>;

pub fn kvlm_parse(raw: Vec<u8>) -> Result<Kvlm, WyagError> {
    let mut dict: Kvlm = HashMap::new();
    let mut pos = 0;
    // Read through the file
    while pos < raw.len() {
        let relative_spc: Option<usize> = raw[pos..].iter().position(|&b| b == b' ');
        let relative_nl: Option<usize> = raw[pos..].iter().position(|&b| b == b'\n');
        // Case 1 : no space before the new line : the message
        let condition = match (relative_spc, relative_nl) {
            (_, None) => false,
            (None, Some(_)) => true,
            (Some(s), Some(n)) => n < s,
        };
        if condition {
            assert_eq!(relative_nl.unwrap(), 0);
            dict.insert(None, vec![raw[pos + 1..].to_vec()]);
            return Ok(dict);
        }
        let spc = pos + relative_spc.unwrap();
        let nl = pos + relative_nl.unwrap();
        let key = raw[pos..spc].to_vec();

        // Case 2 : keywords : key-value pairs
        let mut end = nl;
        // We loop until we find a linebreak without being followed a space
        // A space after the new line means continuation, not next pair
        loop {
            // Line break without space found
            if end + 1 >= raw.len() || raw[end + 1] != b' ' {
                break;
            }
            // Jump to next new line
            let loop_nl: Option<usize> = raw[end + 1..].iter().position(|&b| b == b'\n');
            if let Some(some_loop_nl) = loop_nl {
                end = end + 1 + some_loop_nl;
            } else {
                end = raw.len();
                break;
            }
        }
        // Read value
        let value = raw[spc + 1..end].to_vec();

        // Removing leading space after newlines (continuation lines)
        let string = String::from_utf8(value)?;
        let string = string.replace("\n ", "\n");
        let cleaned_value = string.into_bytes();

        // Either create new entry or append to existing one
        dict.entry(Some(key))
            .and_modify(|v| v.push(cleaned_value.clone()))
            .or_insert(vec![cleaned_value]);
        pos = end + 1;
    }

    Ok(dict)
}

#[cfg(test)]
mod tests {
    use crate::commit::kvlm_parse;
    #[test]
    fn test_kvlm_parse() {
        let s = "tree 29ff16c9c14e2652b22f8b78bb08a5a07930c147
parent 206941306e8a8af65b66eaaaea388a7ae24d49a0
author Thibault Polge <thibault@thb.lt> 1527025023 +0200
committer Thibault Polge <thibault@thb.lt> 1527025044 +0200
gpgsig -----BEGIN PGP SIGNATURE-----
 
 iQIzBAABCAAdFiEExwXquOM8bWb4Q2zVGxM2FxoLkGQFAlsEjZQACgkQGxM2FxoL
 kGQdcBAAqPP+ln4nGDd2gETXjvOpOxLzIMEw4A9gU6CzWzm+oB8mEIKyaH0UFIPh
 rNUZ1j7/ZGFNeBDtT55LPdPIQw4KKlcf6kC8MPWP3qSu3xHqx12C5zyai2duFZUU
 wqOt9iCFCscFQYqKs3xsHI+ncQb+PGjVZA8+jPw7nrPIkeSXQV2aZb1E68wa2YIL
 3eYgTUKz34cB6tAq9YwHnZpyPx8UJCZGkshpJmgtZ3mCbtQaO17LoihnqPn4UOMr
 V75R/7FjSuPLS8NaZF4wfi52btXMSxO/u7GuoJkzJscP3p4qtwe6Rl9dc1XC8P7k
 NIbGZ5Yg5cEPcfmhgXFOhQZkD0yxcJqBUcoFpnp2vu5XJl2E5I/quIyVxUXi6O6c
 /obspcvace4wy8uO0bdVhc4nJ+Rla4InVSJaUaBeiHTW8kReSFYyMmDCzLjGIu1q
 doU61OM3Zv1ptsLu3gUE6GU27iWYj2RWN3e3HE4Sbd89IFwLXNdSuM0ifDLZk7AQ
 WBhRhipCCgZhkj9g2NEk7jRVslti1NdN5zoQLaJNqSwO1MtxTmJ15Ksk3QP6kfLB
 Q52UWybBzpaP9HEd4XnR+HuQ4k2K0ns2KgNImsNvIyFwbpMUyUWLMPimaV1DWUXo
 5SBjDB/V/W2JBFR+XKHFJeFwYhj7DD/ocsGr4ZMx/lgc8rjIBkI=
 =lgTX
 -----END PGP SIGNATURE-----

Create first draft"
            .to_owned();
        let raw = s.into_bytes();
        let result = kvlm_parse(raw).unwrap();

        for (key, values) in &result {
            let key_display = match key {
                Some(k) => String::from_utf8(k.to_vec()).to_owned().unwrap(),
                None => "None".to_owned(),
            };
            let mut values_display = Vec::new();
            for v in values {
                let text = String::from_utf8(v.to_vec()).to_owned().unwrap();
                values_display.push(text);
            }
            println!("{key_display:?}: {values_display:#?}");
        }
    }
}
