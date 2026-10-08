// Setup the new Log4rs logger
// Moved out of misc-test.

pub mod logger {

    pub enum LogStatus {
        TRACE,
        DEBUG,
        INFO,
        WARN,
        ERROR,
    }

    use log::{debug, error, info, trace, warn};

    /// Setup the logger
    ///
    /// `log_config` The logging_config.yaml or log config for Log4rs.
    pub fn init(log_config: &str) {
        // log4rs::init_file("logging_config.yaml", Default::default()).unwrap();
        log4rs::init_file(log_config, Default::default()).unwrap();
    }

    /// Run tests for the logger.
    ///
    /// So far, just runs all the log functions.
    pub fn test() {
        trace!("detailed tracing info");
        debug!("debug info");
        info!("relevant general info");
        warn!("warning this program doesn't do much");
        error!("error message here");
    }

    /// Write a message to the log file.
    ///
    /// TODO Why does this give warnings?
    ///
    /// I probably won't use this much since I can already easily do it with info!() and the other functions.
    ///
    ///
    // pub fn write(status: LogStatus, message: &str) {
    //     match status {
    //         LogStatus::TRACE => trace!("{}", message),
    //         // LogStatus::TRACE => {
    //         //     trace!("{}", message);
    //         // }
    //
    //         LogStatus::DEBUG => debug!("{}", message),
    //         LogStatus::INFO => info!("{}", message),
    //         LogStatus::WARN => warn!("{}", message),
    //         LogStatus::ERROR => error!("{}", message),
    //
    //         _ => println!("Error, invalid log value"),
    //     }
    // }



}