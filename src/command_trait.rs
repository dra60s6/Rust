pub trait Command {
    fn execute(&self);
    fn usage(&self);
}