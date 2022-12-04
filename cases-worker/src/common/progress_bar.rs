use indicatif::{ProgressBar, ProgressStyle};

pub fn init_progress_bar(total_count: usize) -> ProgressBar {
    let pb = ProgressBar::new(total_count as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {pos}/{len}")
            .progress_chars("#>"),
    );
    pb
}
