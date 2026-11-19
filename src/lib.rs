#![forbid(unsafe_code)]
/// Power BI dataset model and DAX measure registry for SigmaOS BI integration.
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DaxMeasure {
    pub name: String,
    pub expression: String,
    pub format_string: Option<String>,
    pub table: String,
}

impl DaxMeasure {
    pub fn new(table: &str, name: &str, expression: &str) -> Self {
        Self {
            name: name.into(),
            expression: expression.into(),
            format_string: None,
            table: table.into(),
        }
    }

    pub fn with_format(mut self, f: &str) -> Self {
        self.format_string = Some(f.into());
        self
    }

    pub fn full_name(&self) -> String {
        format!("[{}]", self.name)
    }

    pub fn dax_definition(&self) -> String {
        format!("{} = {}", self.full_name(), self.expression)
    }
}

#[derive(Debug, Default)]
pub struct PbiDataset {
    pub name: String,
    pub measures: HashMap<String, DaxMeasure>,
}

impl PbiDataset {
    pub fn add_measure(&mut self, m: DaxMeasure) {
        self.measures.insert(m.name.clone(), m);
    }
    pub fn get_measure(&self, name: &str) -> Option<&DaxMeasure> {
        self.measures.get(name)
    }
    pub fn measure_count(&self) -> usize {
        self.measures.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dax_definition_format_aaa() {
        let m = DaxMeasure::new("Sales", "Total Revenue", "SUM(Sales[Revenue])");
        assert_eq!(m.dax_definition(), "[Total Revenue] = SUM(Sales[Revenue])");
    }

    #[test]
    fn test_full_name_format_aaa() {
        let m = DaxMeasure::new("Orders", "Order Count", "COUNTROWS(Orders)");
        assert_eq!(m.full_name(), "[Order Count]");
    }

    #[test]
    fn test_dataset_measure_count_aaa() {
        let mut ds = PbiDataset {
            name: "Sales Model".into(),
            measures: HashMap::new(),
        };
        ds.add_measure(DaxMeasure::new("S", "Rev", "SUM(S[R])"));
        assert_eq!(ds.measure_count(), 1);
    }

    #[test]
    fn test_dataset_get_measure_aaa() {
        let mut ds = PbiDataset {
            name: "DS".into(),
            measures: HashMap::new(),
        };
        ds.add_measure(DaxMeasure::new("T", "KPI", "1+1"));
        assert!(ds.get_measure("KPI").is_some());
    }
}
