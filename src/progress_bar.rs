pub struct ProgressBar(indicatif::ProgressBar);

impl ProgressBar {
  pub fn new(length: i32) -> Self {
    Self(
      indicatif::ProgressBar::new(u64::try_from(length).expect("Unable to convert progress bar length")).with_style(
        indicatif::ProgressStyle::with_template("[{elapsed_precise}] |{bar:40.yellow/white}| {pos}/{len}")
          .expect("Failed to create progress bar")
          .progress_chars(" \u{15E7}\u{00B7}"),
      ),
    )
  }

  pub fn finish(&self) {
    self.0.finish_with_message("Done.");
  }

  pub fn increment_by(&self, length: u64) {
    self.0.inc(length);
  }
}
