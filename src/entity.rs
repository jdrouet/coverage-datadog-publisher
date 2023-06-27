use datadog_client::metrics::{Serie, Type, Point};

#[derive(Debug, serde::Deserialize)]
pub struct FileContent {
    pub data: Vec<Entry>,
    #[serde(rename = "type")]
    pub kind: String,
    pub version: String,
}

impl FileContent {
    pub fn metrics(&self, ts: u64, base: String) -> Vec<Serie> {
        let mut metrics = Vec::with_capacity(20 * self.data.len());
        for entry in self.data.iter() {
            entry.totals.add_metrics(ts, format!("{base}.totals"), &mut metrics);
        }
        metrics
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Entry {
    pub totals: EntryTotals,
}

#[derive(Debug, serde::Deserialize)]
pub struct EntryTotals {
    pub branches: Report,
    pub functions: Report,
    pub instantiations: Report,
    pub lines: Report,
    pub regions: Report,
}

impl EntryTotals {
    pub fn add_metrics(&self, ts: u64, base: String, result: &mut Vec<Serie>) {
        self.branches.add_metrics(ts, format!("{base}.branches"), result);
        self.functions.add_metrics(ts, format!("{base}.functions"), result);
        self.instantiations.add_metrics(ts, format!("{base}.instantiations"), result);
        self.lines.add_metrics(ts, format!("{base}.lines"), result);
        self.regions.add_metrics(ts, format!("{base}.regions"), result);
    }
}

#[derive(Debug, serde::Deserialize)]
pub struct Report {
    pub count: usize,
    pub covered: usize,
    pub notcovered: Option<usize>,
    pub percent: f64,
}

impl Report {
    pub fn add_metrics(&self, ts: u64, base: String, result: &mut Vec<Serie>) {
        result.push(Serie::new(format!("{base}.count"), Type::Gauge).add_point(Point::new(ts, self.count as f64)));
        result.push(Serie::new(format!("{base}.covered"), Type::Gauge).add_point(Point::new(ts, self.covered as f64)));
        if let Some(notcovered) = self.notcovered {
            result.push(Serie::new(format!("{base}.notcovered"), Type::Gauge).add_point(Point::new(ts, notcovered as f64)));
        }
        result.push(Serie::new(format!("{base}.percent"), Type::Gauge).add_point(Point::new(ts, self.percent)));
    }
}