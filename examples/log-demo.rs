#[macro_use]
extern crate log;
#[macro_use]
extern crate sentry;

fn main() {
    let _sentry = sentry::init(
        sentry::ClientOptions {
            dsn: "https://a94ae32be2584e0bbd7a4cbb95971fee@sentry.io/1041156"
                .parse()
                .ok(),
            release: sentry_crate_release!(),
            ..Default::default()
        }.add_integration(
            sentry::integrations::log::LogIntegration::default().with_env_logger_dest(None),
        ),
    );

    debug!("System is booting");
    info!("System is booting");
    warn!("System is warning");
    error!("Holy shit everything is on fire!");
}
