use std::borrow::Cow;

pub trait ProgressTracker {
    fn set_length(&self, len: u64);
    fn progress_delta(&self, delta: u64);
    fn set_progress(&self, len: u64);
    fn set_message(&self, msg: String);
    fn finish(&self);
}

#[cfg(feature = "cli")]
impl ProgressTracker for indicatif::ProgressBar {
    fn set_length(&self, len: u64) {
        self.set_length(len);
    }
    fn progress_delta(&self, delta: u64) {
        self.inc(delta);
    }
    fn set_progress(&self, len: u64) {
        self.set_position(len);
    }
    fn set_message(&self, msg: String) {
        self.set_message(msg);
    }
    fn finish(&self) {
        self.finish()
    }

}