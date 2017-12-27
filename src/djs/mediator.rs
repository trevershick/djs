pub trait Mediator {
    fn print(&self, out: String);
    fn start_progress(&mut self, name: &str, total_value: Option<u64>);
    fn incr_progress(&mut self, name: &str, incr_by: u64);
    fn finish_progress(&mut self, name: &str);

    fn start_step(&mut self, step: &str);
    fn finish_step(&mut self);

    fn human_bytes(&self, bytes: u64) -> String;
}
