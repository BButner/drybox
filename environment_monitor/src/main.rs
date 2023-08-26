use std::net::SocketAddr;

use prometheus_exporter_base::{
    prelude::{Authorization, ServerOptions},
    render_prometheus, MetricType, PrometheusInstance, PrometheusMetric,
};

#[derive(Debug, Clone, Default)]
struct Options {}

const ENV_MONITOR_PORT_VAR: &str = "DRYBOX_PORT";

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

    render_prometheus(server_options, Options::default(), |_, _| async move {
        Ok(PrometheusMetric::build()
            .with_name("drybox_reading")
            .with_metric_type(MetricType::Gauge)
            .with_help("Temperature and Humidity in the drybox")
            .build()
            .render_and_append_instance(
                &PrometheusInstance::new()
                    .with_label("metric", "humidity")
                    .with_value(50.0),
            )
            .render_and_append_instance(
                &PrometheusInstance::new()
                    .with_label("metric", "temperature")
                    .with_value(20.0),
            )
            .render())
    })
    .await;
}
