mod adb;

pub trait Tool {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
}
