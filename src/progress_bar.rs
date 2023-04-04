pub struct ProgressBar(indicatif::ProgressBar);

impl ProgressBar {
  pub fn new(length: i32) -> Self {
    let bar = indicatif::ProgressBar::new(u64::try_from(length).expect("Unable to convert progress bar length"));
    bar.set_style(
      indicatif::ProgressStyle::with_template(
        "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}",
      )
      .expect("Failed to create progress bar")
      .progress_chars("#>-"),
    );

    Self(bar)
  }

  pub fn finish(&self) {
    self.0.finish();
  }

  pub fn increment_by(&self, length: u64) {
    self.0.inc(length);
  }
}
