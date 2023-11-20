pub fn generate_band(city: &str, pet_name: &str) -> Result<String, String> {
    if city.is_empty() || pet_name.is_empty() {
        return Err("both city and pet name needs to be provided".to_owned());
    }
    Ok(format!("{city} {pet_name}"))
}

#[cfg(test)]
mod tests {
    use crate::generate_band;

    #[test]
    fn test_generate_band_good() {
        let city = "Lodz";
        let pet = "Tarzan";

        let result = generate_band(city, pet);
        assert_eq!(result, Ok("Lodz Tarzan".to_owned()));
    }

    #[test]
    fn test_generate_band_bad() {
        let error = Err("both city and pet name needs to be provided".to_owned());

        assert_eq!(generate_band("", "Pet"), error);
        assert_eq!(generate_band("City", ""), error);
        assert_eq!(generate_band("", ""), error);
    }
}
