/// the default port on which the dev server will run
pub const DEFAULT_PORT: u16 = 4000;
/// the max port to use when searching for an available port
pub const MAX_PORT: u16 = u16::MAX;
/// localhost IP
pub const LOCALHOST: &str = "127.0.0.1";
/// the name of the folder indicating a grafbase project
pub const GRAFBASE_FOLDER: &str = "grafbase";
/// a file expected to be in the grafbase folder
pub const GRAFBASE_SCHEMA: &str = "schema.graphql";
/// the name for the db / cache directory per project and the global cache directory for the user
pub const DOT_GRAFBASE_FOLDER: &str = ".grafbase";
/// the registry.json file generated from schema.graphql
pub const REGISTRY_FILE: &str = "registry.json";
/// the tracing filter to be used when tracing is on
pub const TRACE_LOG_FILTER: &str = "grafbase=trace,common=trace,dev_server=trace,local_gateway=trace,tower_http=debug";
/// the tracing filter to be used when tracing is off
pub const DEFAULT_LOG_FILTER: &str = "off";
