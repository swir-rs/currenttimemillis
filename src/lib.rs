use std::time::{SystemTime};

pub fn current_time_milliseconds()->u128{
    match SystemTime::from(SystemTime::UNIX_EPOCH).elapsed(){
        Ok(time) => time.as_millis(),
        Err(_) => panic!("SystemTime before UNIX EPOCH!")
    }
}
