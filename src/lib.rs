use regex::Regex;

const URL: &str = "https://dolarhoje.com/";

/// Gets the dolar value for today
///
/// Example:
///
/// ```
/// if let Ok(value) = dolarhoje::get() {
///     println!("Dolar today is {value}");
/// }
/// ```
pub fn get() -> Result<f32, &'static str> {
    let response = reqwest::blocking::get(URL).expect("Unreachable network");
    let content = response.text().expect("Unable to parse response");
    let re = Regex::new(r"US\$ 1 \(um dólar\)<\/td>      <td>R\$ ([0-9],[0-9]{2})<\/td>").unwrap();
    if let Some(mat) = re.captures(content.as_str()) {
        let dolar: f32 = mat
            .get(1)
            .expect("Unknown response")
            .as_str()
            .replace(",", ".")
            .parse()
            .expect("Failed to parse response");

        return Ok(dolar);
    }

    Err("Unknown response")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_get() {
        let result = get();
        assert!(result.is_ok());
        assert!(result.ok().unwrap() > 0.0);
    }
}
