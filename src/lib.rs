// GST Engine - Basic calculation

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gst_calculation() {
        let gst = calculate_gst(1000.0, 18.0);
        assert_eq!(gst, 180.0);
    }

    #[test]
    fn test_total_calculation() {
        let total = calculate_total(1000.0, 18.0);
        assert_eq!(total, 1180.0);
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
}