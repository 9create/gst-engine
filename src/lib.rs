// GST Engine - GST calculation, validation, aur duplicate detection

// ===== GST CALCULATION (Robust Version) =====

// Ab function crash nahi karega galat input pe -
// ya toh sahi number dega (Ok), ya error message dega (Err)
pub fn calculate_gst(amount: f64, gst_percent: f64) -> Result<f64, String> {
    if amount < 0.0 {
        return Err("Amount negative nahi ho sakta".to_string());
    }
    if gst_percent < 0.0 || gst_percent > 28.0 {
        return Err("GST percentage 0 se 28 ke beech hona chahiye".to_string());
    }
    Ok(amount * gst_percent / 100.0)
}

pub fn calculate_total(amount: f64, gst_percent: f64) -> Result<f64, String> {
    let gst = calculate_gst(amount, gst_percent)?; // agar upar error aaya, yahi return ho jayega
    Ok(amount + gst)
}

pub struct GstSplit {
    pub cgst: f64,
    pub sgst: f64,
    pub igst: f64,
}

pub fn calculate_gst_split(amount: f64, gst_percent: f64, same_state: bool) -> Result<GstSplit, String> {
    let total_gst = calculate_gst(amount, gst_percent)?;

    if same_state {
        Ok(GstSplit {
            cgst: total_gst / 2.0,
            sgst: total_gst / 2.0,
            igst: 0.0,
        })
    } else {
        Ok(GstSplit {
            cgst: 0.0,
            sgst: 0.0,
            igst: total_gst,
        })
    }
}

// ===== DUPLICATE DETECTION =====

pub struct InvoiceItem {
    pub name: String,
    pub quantity: f64,
}

pub fn find_duplicate_items(items: &[InvoiceItem]) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    let mut duplicates: Vec<String> = Vec::new();

    for item in items {
        let normalized = item.name.trim().to_lowercase();

        if seen.contains(&normalized) {
            if !duplicates.contains(&item.name) {
                duplicates.push(item.name.clone());
            }
        } else {
            seen.push(normalized);
        }
    }

    duplicates
}

// ===== VALIDATION =====

pub fn is_valid_gstin(gstin: &str) -> bool {
    let trimmed = gstin.trim();
    trimmed.len() == 15 && trimmed.chars().all(|c| c.is_alphanumeric())
}

pub fn is_valid_amount(amount: f64) -> bool {
    amount >= 0.0 && amount.is_finite() // is_finite() = infinity ya "NaN" jaisi ajeeb values reject karega
}

pub fn is_valid_buyer_name(name: &str) -> bool {
    !name.trim().is_empty()
}

pub fn validate_invoice(gstin: &str, amount: f64, buyer_name: &str) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();

    if !is_valid_gstin(gstin) {
        errors.push("GSTIN sahi format me nahi hai (15 characters chahiye)".to_string());
    }

    if !is_valid_amount(amount) {
        errors.push("Amount negative ya invalid nahi ho sakta".to_string());
    }

    if !is_valid_buyer_name(buyer_name) {
        errors.push("Buyer ka naam khali nahi ho sakta".to_string());
    }

    errors
}

// ===== TESTS =====

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gst_calculation() {
        assert_eq!(calculate_gst(1000.0, 18.0), Ok(180.0));
    }

    #[test]
    fn test_total_calculation() {
        assert_eq!(calculate_total(1000.0, 18.0), Ok(1180.0));
    }

    #[test]
    fn test_negative_amount_returns_error() {
        let result = calculate_gst(-500.0, 18.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_negative_gst_percent_returns_error() {
        let result = calculate_gst(1000.0, -18.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gst_percent_too_high_returns_error() {
        let result = calculate_gst(1000.0, 999.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_gst_split_same_state() {
        let split = calculate_gst_split(1000.0, 18.0, true).unwrap();
        assert_eq!(split.cgst, 90.0);
        assert_eq!(split.sgst, 90.0);
        assert_eq!(split.igst, 0.0);
    }

    #[test]
    fn test_gst_split_different_state() {
        let split = calculate_gst_split(1000.0, 18.0, false).unwrap();
        assert_eq!(split.cgst, 0.0);
        assert_eq!(split.sgst, 0.0);
        assert_eq!(split.igst, 180.0);
    }

    #[test]
    fn test_gst_split_with_invalid_input_returns_error() {
        let result = calculate_gst_split(-1000.0, 18.0, true);
        assert!(result.is_err());
    }

    #[test]
    fn test_duplicate_detection_finds_case_insensitive_match() {
        let items = vec![
            InvoiceItem { name: "MCB Switch".to_string(), quantity: 5.0 },
            InvoiceItem { name: "Wire".to_string(), quantity: 2.0 },
            InvoiceItem { name: "mcb switch".to_string(), quantity: 3.0 },
        ];
        let duplicates = find_duplicate_items(&items);
        assert_eq!(duplicates.len(), 1);
        assert_eq!(duplicates[0], "mcb switch");
    }

    #[test]
    fn test_no_duplicates_when_all_unique() {
        let items = vec![
            InvoiceItem { name: "MCB Switch".to_string(), quantity: 5.0 },
            InvoiceItem { name: "Wire".to_string(), quantity: 2.0 },
        ];
        let duplicates = find_duplicate_items(&items);
        assert_eq!(duplicates.len(), 0);
    }

    #[test]
    fn test_valid_gstin() {
        assert!(is_valid_gstin("27ABCDE1234F1Z5"));
    }

    #[test]
    fn test_invalid_gstin_wrong_length() {
        assert!(!is_valid_gstin("ABC123"));
    }

    #[test]
    fn test_invalid_amount_negative() {
        assert!(!is_valid_amount(-500.0));
    }

    #[test]
    fn test_invalid_amount_infinite() {
        assert!(!is_valid_amount(f64::INFINITY));
    }

    #[test]
    fn test_valid_amount() {
        assert!(is_valid_amount(500.0));
    }

    #[test]
    fn test_empty_buyer_name_invalid() {
        assert!(!is_valid_buyer_name("   "));
    }

    #[test]
    fn test_full_invoice_validation_with_errors() {
        let errors = validate_invoice("BADGSTIN", -100.0, "");
        assert_eq!(errors.len(), 3);
    }

    #[test]
    fn test_full_invoice_validation_no_errors() {
        let errors = validate_invoice("27ABCDE1234F1Z5", 1000.0, "Ramesh Electricals");
        assert_eq!(errors.len(), 0);
    }
}