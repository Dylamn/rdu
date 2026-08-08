// humanize.rs
//
// One job: turn a raw byte count into something readable like
// "4.2 MB" or "1.1 GB". Small, self-contained, and easy to unit test —
// a good candidate for writing your first #[test] functions in this
// project.

// fn format(bytes: u64) -> String
//
// Hints:
//   - Divide repeatedly by 1024 while tracking a unit index
//     (B, KB, MB, GB, TB, ...) until the value is < 1024 or you run
//     out of units.
//   - Keep one decimal place for anything above B (e.g. "3.5 MB"),
//     but whole numbers for plain bytes (e.g. "512 B").
//   - Decide once: 1024-based (KiB/MiB, more "technically correct")
//     or 1000-based (KB/MB, matches what `du -h` often shows)? Either
//     is fine, just be consistent and maybe note your choice in a
//     comment so future-you remembers.
//
// pub fn format(bytes: u64) -> String {
//     todo!()
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn formats_bytes() {
//         // assert_eq!(format(500), "500 B");
//     }
//
//     #[test]
//     fn formats_megabytes() {
//         // assert_eq!(format(5_000_000), "4.8 MB");  // depends on 1024 vs 1000 choice
//     }
// }