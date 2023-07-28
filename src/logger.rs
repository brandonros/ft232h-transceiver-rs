use log::LevelFilter;

use log4rs::{
  append::console::ConsoleAppender,
  config::{Appender, Config, Logger, Root},
  encode::pattern::PatternEncoder,
};

const LOG_PATTERN: &str = "[{d:>35.35}] {t} {l}: {m}\n";

pub fn init_stdout_logger() {
  let stdout = ConsoleAppender::builder().encoder(Box::new(PatternEncoder::new(LOG_PATTERN))).build();
  let config = Config::builder()
    .appender(Appender::builder().build("stdout", Box::new(stdout)))
    .logger(Logger::builder().build("reqwest", LevelFilter::Warn))
    .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
    .unwrap();
  log4rs::init_config(config).unwrap();
}
