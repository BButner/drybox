use std::net::SocketAddr;

use prometheus_exporter_base::{
    prelude::{Authorization, ServerOptions},
    render_prometheus, MetricType, PrometheusInstance, PrometheusMetric,
};
use simple_dht11::dht11::Dht11;

#[derive(Debug, Clone, Default)]
struct Options {}

const ENV_MONITOR_PORT_VAR: &str = "DRYBOX_PORT";
const ENV_DHT11_BCM_PIN_VAR: &str = "DRYBOX_DHT11_BCM_PIN";

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let port_env = std::env::var(ENV_MONITOR_PORT_VAR);
    let pin_env = std::env::var(ENV_DHT11_BCM_PIN_VAR);

    let port = match port_env {
        Ok(port) => port
            .parse::<u16>()
            .expect(format!("{} must be a valid port number", ENV_MONITOR_PORT_VAR).as_str()),
        Err(_) => panic!("Environment variable {} must be set", ENV_MONITOR_PORT_VAR),
    };

    let pin = match pin_env {
        Ok(pin) => pin
            .parse::<u8>()
            .expect(format!("{} must be a valid pin number", ENV_DHT11_BCM_PIN_VAR).as_str()),
        Err(_) => panic!("Environment variable {} must be set", ENV_DHT11_BCM_PIN_VAR),
    };

    let addr: SocketAddr = ([0, 0, 0, 0], port).into();

    let server_options = ServerOptions {
        addr,
        authorization: Authorization::None,
    };

    let mut dht11 = Dht11::new(pin);

    render_prometheus(server_options, Options::default(), |_, _| async move {
        let reading = loop {
            let reading = dht11.read();
            let temp_f = reading.temperature * 9 / 5 + 32;

            // This is due to a bug in my library that I still need to fix...
            if reading.temperature < 120 {
                break reading;
            }
        };

        Ok(PrometheusMetric::build()
            .with_name("drybox_reading")
            .with_metric_type(MetricType::Gauge)
            .with_help("Temperature and Humidity in the drybox")
            .build()
            .render_and_append_instance(
                &PrometheusInstance::new()
                    .with_label("metric", "humidity")
                    .with_value(reading.humidity as f64),
            )
            .render_and_append_instance(
                &PrometheusInstance::new()
                    .with_label("metric", "temperature")
                    .with_value((reading.temperature * 9 / 5 + 32) as f64),
            )
            .render())
    })
    .await;
}
