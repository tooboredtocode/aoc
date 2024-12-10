use std::fmt;
use std::time::Duration;

#[derive(Debug, Clone, Copy)]
pub struct DisplayDuration(pub Duration);

impl fmt::Display for DisplayDuration {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let secs = self.0.as_secs();
        if secs > 1 {
            return write!(f, "{}.{:03}s", secs, self.0.subsec_millis() / 10);
        }

        let millis = self.0.subsec_millis();
        if millis > 100 {
            return write!(f, "{}ms", millis);
        }
        if millis > 0 {
            let micros = self.0.subsec_micros() - millis * 1000;
            return write!(f, "{}.{:03}ms", millis, micros / 10);
        }

        let micros = self.0.subsec_micros();
        if micros > 100 {
            return write!(f, "{}µs", micros);
        }
        if micros > 0 {
            let nanos = self.0.subsec_nanos() - micros * 1000;
            return write!(f, "{}.{:03}µs", micros, nanos / 10);
        }
        
        let nanos = self.0.subsec_nanos();
        write!(f, "{}ns", nanos)
    }
}
