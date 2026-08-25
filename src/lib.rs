// GST Engine - GST calculation, validation, aur duplicate detection

// ===== GST CALCULATION =====

pub fn calculate_gst(amount: f64, gst_percent: f64) -> f64 {
    amount * gst_percent / 100.0
}

pub fn calculate_total(amount: f64, gst_percent: f64) -> f64 {
    let gst = calculate_gst(amount, gst_percent);
    amount + gst
}

pub struct GstSplit {
    pub cgst: f64,
    pub sgst: f64,
    pub igst: f64,
}

pub fn calculate_gst_split(amount: f64, gst_percent: f64, same_state: bool) -> GstSplit {
    let total_gst = calculate_gst(amount, gst_percent);

    if same_state {
        GstSplit {
            cgst: total_gst / 2.0,
            sgst: total_gst / 2.0,
            igst: 0.0,
        }
    } else {
        GstSplit {
            cgst: 0.0,
            sgst: 0.0,
            igst: total_gst,
        }
    }
}

// ===== DUPLICATE DETECTION =====

// Ek invoice item ka structure
pub struct InvoiceItem {
    pub name: String,
    pub quantity: f64,
}

// List of items check karega ki koi item naam repeat toh nahi ho raha
// (case-insensitive: "MCB Switch" aur "mcb switch" dono same maane jayenge)
pub fn find_duplicate_items(items: &[InvoiceItem]) -> Vec<String> {
    let mut seen: Vec<String> = Vec::new();
    let mut duplicates: Vec<String> = Vec::new();

    for item in items {
        // Naam ko lowercase aur trim karke normalize karte hain,
        // taaki "MCB Switch " aur "mcb switch" dono match ho jayein
        let normalized = item.name.trim().to_lowercase();

        if seen.contains(&normalized) {
            // Agar pehle se dekha hua naam hai, aur abhi tak duplicates list me nahi hai,
            // tabhi add karo (taaki ek hi naam baar-baar list me na aaye)
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

// GSTIN format check: 15 characters hone chahiye (India ka standard rule)
pub fn is_valid_gstin(gstin: &str) -> bool {
    let trimmed = gstin.trim();
    trimmed.len() == 15 && trimmed.chars().all(|c| c.is_alphanumeric())
}

// Amount negative nahi hona chahiye
pub fn is_valid_amount(amount: f64) -> bool {
    amount >= 0.0
}

// Buyer ka naam khali nahi hona chahiye
pub fn is_valid_buyer_name(name: &str) -> bool {
    !name.trim().is_empty()
}

// Poora invoice ek saath validate karta hai, saari galtiyon ki list deta hai
pub fn validate_invoice(gstin: &str, amount: f64, buyer_name: &str) -> Vec<String> {
    let mut errors: Vec<String> = Vec::new();

    if !is_valid_gstin(gstin) {
        errors.push("GSTIN sahi format me nahi hai (15 characters chahiye)".to_string());
    }

    if !is_valid_amount(amount) {
        errors.push("Amount negative nahi ho sakta".to_string());
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
        assert_eq!(calculate_gst(1000.0, 18.0), 180.0);
    }

    #[test]
    fn test_total_calculation() {
        assert_eq!(calculate_total(1000.0, 18.0), 1180.0);
    }

    #[test]
    fn test_gst_split_same_state() {
        let split = calculate_gst_split(1000.0, 18.0, true);
        assert_eq!(split.cgst, 90.0);
        assert_eq!(split.sgst, 90.0);
        assert_eq!(split.igst, 0.0);
    }

    #[test]
    fn test_gst_split_different_state() {
        let split = calculate_gst_split(1000.0, 18.0, false);
        assert_eq!(split.cgst, 0.0);
        assert_eq!(split.sgst, 0.0);
        assert_eq!(split.igst, 180.0);
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
    fn test_invalid_amount() {
        assert!(!is_valid_amount(-500.0));
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