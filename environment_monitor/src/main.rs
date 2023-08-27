use std::{fmt, net::SocketAddr, sync::Mutex};

use prometheus_exporter_base::{
    prelude::{Authorization, ServerOptions},
    render_prometheus, MetricType, PrometheusInstance, PrometheusMetric,
};
use simple_dht11::dht11::{Dht11, Dht11Reading};

struct Options {
    pub dht11_pin: u8,
    pub dht11: Dht11,
}

impl Options {
    fn get_reading(&mut self) -> Dht11Reading {
        self.dht11.get_reading()
    }
}

impl Default for Options {
    fn default() -> Self {
        let pin_env = std::env::var(ENV_DHT11_BCM_PIN_VAR);

        let pin = match pin_env {
            Ok(pin) => pin
                .parse::<u8>()
                .expect(format!("{} must be a valid pin number", ENV_DHT11_BCM_PIN_VAR).as_str()),
            Err(_) => panic!("Environment variable {} must be set", ENV_DHT11_BCM_PIN_VAR),
        };

        Options {
            dht11_pin: pin,
            dht11: Dht11::new(pin),
        }
    }
}

impl fmt::Debug for Options {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Options")
            .field("dht11_pin", &self.dht11_pin)
            .finish()
    }
}

const ENV_MONITOR_PORT_VAR: &str = "DRYBOX_PORT";
const ENV_DHT11_BCM_PIN_VAR: &str = "DRYBOX_DHT11_BCM_PIN";

#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv::dotenv().ok();

    let port_env = std::env::var(ENV_MONITOR_PORT_VAR);

    let port = match port_env {
        Ok(port) => port
            .parse::<u16>()
            .expect(format!("{} must be a valid port number", ENV_MONITOR_PORT_VAR).as_str()),
        Err(_) => panic!("Environment variable {} must be set", ENV_MONITOR_PORT_VAR),
    };

    let addr: SocketAddr = ([0, 0, 0, 0], port).into();

    let server_options = ServerOptions {
        addr,
        authorization: Authorization::None,
    };

    render_prometheus(
        server_options,
        Mutex::new(Options::default()),
        |_, options| async move {
            let mut options = options.lock().unwrap();

            let reading = loop {
                let reading = options.get_reading();
                let temp_f = reading.temperature * 9.0 / 5.0 + 32.0;

                // This is due to a bug in my library that I still need to fix...
                if temp_f < 120.0 && reading.humidity < 100.0 {
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
                        .with_value((reading.temperature * 9.0 / 5.0 + 32.0) as f64),
                )
                .render())
        },
    )
    .await;
}
