/// Which convention to use when formatting byte counts.
///
/// - `Decimal` uses base 1000 and SI labels (`KB`, `MB`, `GB`, ...) —
///   matches how storage vendors advertise capacity, and what tools
///   like `du -h --si` show.
/// - `Binary` uses base 1024 and IEC labels (`KiB`, `MiB`, `GiB`, ...) —
///   matches how most operating systems and `du -h` (without `--si`)
///   actually report sizes, since memory/disk blocks are powers of two.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SizeUnit {
    Decimal,
    Binary,
}

impl SizeUnit {
    /// The multiplier between consecutive units (1000 or 1024).
    fn base(self) -> f64 {
        match self {
            SizeUnit::Decimal => 1000.0,
            SizeUnit::Binary => 1024.0,
        }
    }

    /// The ordered unit labels, smallest to largest, for this convention.
    /// Index 0 is always the plain-bytes label.
    fn labels(self) -> [&'static str; 6] {
        match self {
            SizeUnit::Decimal => ["B", "KB", "MB", "GB", "TB", "PB"],
            SizeUnit::Binary => ["B", "KiB", "MiB", "GiB", "TiB", "PiB"],
        }
    }
}

/// Formats a raw byte count as a short, human-readable string, e.g.,
/// `"512 B"`, `"4.8 MB"`, or `"1.1 GiB"` depending on `unit`.
///
/// The value is repeatedly divided by the convention's base (1000 for
/// [`SizeUnit::Decimal`], 1024 for [`SizeUnit::Binary`]) until it drops below that base or the
/// largest available label (`PB`/`PiB`) is reached — whichever comes
/// first. Plain bytes are printed as a whole number; anything larger
/// is printed with one decimal place.
///
/// # Examples
///
/// ```
/// assert_eq!(format(500, SizeUnit::Binary), "500 B");
/// assert_eq!(format(5_000_000, SizeUnit::Decimal), "5.0 MB");
/// assert_eq!(format(5_000_000, SizeUnit::Binary), "4.8 MiB");
/// ```
///
/// # Panics
///
/// Never panics — even [`u64::MAX`] is handled by capping at the largest
/// available unit rather than indexing past the label array.
pub fn format(bytes: u64, unit: SizeUnit) -> String {
    let base = unit.base();
    let labels = unit.labels();

    let mut value = bytes as f64;
    let mut unit_index = 0;

    while value >= base && unit_index < labels.len() - 1 {
        value /= base;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, labels[unit_index])
    } else {
        format!("{:.1} {}", value, labels[unit_index])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn formats_bytes() {
        assert_eq!(format(500, SizeUnit::Binary), "500 B");
        assert_eq!(format(500, SizeUnit::Decimal), "500 B");
    }

    #[test]
    fn formats_megabytes_binary() {
        assert_eq!(format(5_000_000, SizeUnit::Binary), "4.8 MiB");
    }

    #[test]
    fn formats_megabytes_decimal() {
        assert_eq!(format(5_000_000, SizeUnit::Decimal), "5.0 MB");
    }

    #[test]
    fn formats_exact_boundary() {
        assert_eq!(format(1024, SizeUnit::Binary), "1.0 KiB");
        assert_eq!(format(1000, SizeUnit::Decimal), "1.0 KB");
    }

    #[test]
    fn caps_at_largest_unit() {
        // Doesn't matter how big this is, it should never run past `PiB`/`PB`
        // or fall through to an unlabeled number.
        let huge = u64::MAX;
        assert!(format(huge, SizeUnit::Binary).ends_with("PiB"));
        assert!(format(huge, SizeUnit::Decimal).ends_with("PB"));
    }
}
