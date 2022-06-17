use crate::errors::LocalGatewayError;
use crate::types::ServerMessage;
use common::utils::find_available_port;
use common::{consts::DEFAULT_PORT, types::LocalAddressType};
use dev_server::errors::DevServerError;
use std::sync::mpsc::Receiver;
use std::thread;

type ServerInfo = (thread::JoinHandle<Result<(), DevServerError>>, Receiver<ServerMessage>);

/// starts the dev server if an available port can be found
///
/// # Errors
///
/// returns [`LocalGatewayError::AvailablePort`] if no available port can  be found
///
/// returns [`LocalGatewayError::PortInUse`] if search is off and the supplied port is in use
pub fn start_dev_server(external_port: Option<u16>, search: bool) -> Result<ServerInfo, LocalGatewayError> {
    let start_port = external_port.unwrap_or(DEFAULT_PORT);
    match find_available_port(search, start_port, LocalAddressType::Unspecified) {
        Some(port) => {
            let (handle, receiver) = dev_server::start(port);
            Ok((handle, receiver))
        }
        None => {
            if search {
                Err(LocalGatewayError::AvailablePort)
            } else {
                Err(LocalGatewayError::PortInUse(start_port))
            }
        }
    }
}
