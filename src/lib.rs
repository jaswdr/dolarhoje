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
    let re = Regex::new(r"US\$ 1 \(um dólar\)<\/td>      <td>R\$ ([0-9],[0-9]{2})<\/td>").unwrap();

    match reqwest::blocking::get(URL) {
        Err(_) => Err("Network unreachable"),
        Ok(response) => match response.text() {
            Err(_) => Err("Response using wrong format"),
            Ok(content) => match re.captures(content.as_str()) {
                None => Err("Invalid response"),
                Some(mat) => match mat.get(1) {
                    None => Err("Value not found"),
                    Some(str) => match str.as_str().replace(",", ".").parse() {
                        Ok(value) => Ok(value),
                        Err(_) => Err("Invalid float value"),
                    },
                },
            },
        },
    }
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
