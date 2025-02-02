use backend::errors::{BackendError, ServerError};
use common::{errors::CommonError, traits::ToExitCode};
use std::io::ErrorKind;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CliError {
    /// returned if a the shell passed to completions is unsupported or unrecognized
    #[error("received an unknown or unsupported shell for completion generation: {0}")]
    UnsupportedShellForCompletions(String),
    // TODO: this might be better as `expect`
    /// returned if the development server panics
    #[error("{0}")]
    ServerPanic(String),
    /// wraps a server error
    #[error(transparent)]
    ServerError(ServerError),
    /// wraps an error originating in the local-gateway crate
    #[error(transparent)]
    BackendError(BackendError),
    /// wraps an error originating in the common crate
    #[error(transparent)]
    CommonError(CommonError),
}

impl ToExitCode for CliError {
    fn to_exit_code(&self) -> i32 {
        match &self {
            Self::UnsupportedShellForCompletions(_) => exitcode::USAGE,
            Self::ServerPanic(_) | Self::ServerError(_) => exitcode::SOFTWARE,
            Self::BackendError(inner) => inner.to_exit_code(),
            Self::CommonError(inner) => inner.to_exit_code(),
        }
    }
}

#[cfg(target_family = "windows")]
const WINDOWS_DIR_NOT_EMPTY_CODE: i32 = 145;

impl CliError {
    /// returns the appropriate hint for a [`CliError`]
    pub fn to_hint(&self) -> Option<String> {
        match self {
            Self::BackendError(BackendError::AlreadyAProject(_)) => Some("try running 'grafbase dev'".to_owned()),
            Self::BackendError(BackendError::AvailablePort) => Some("try supplying a larger port range to search by supplying a lower --port number".to_owned()),
            Self::BackendError(BackendError::DeleteDotGrafbaseDirectory(error)) => {
                match error.kind() {
                    ErrorKind::NotFound => Some("this may be caused by the project previously being reset or by running 'grafbase reset' on a new project".to_owned()),
                    ErrorKind::PermissionDenied => Some("it appears that you do not have sufficient permissions to delete the .grafbase directory, try modifying its permissions".to_owned()),
                    // TODO: replace with ErrorKind::DirectoryNotEmpty once stable 
                    #[cfg(target_family="windows")] 
                    _ => error
                            .raw_os_error()
                            .filter(|raw| raw == &WINDOWS_DIR_NOT_EMPTY_CODE)
                            .map(|_| "this may be caused by the .grafbase directory being in use by another instance of 'grafbase'".to_owned()),
                    #[cfg(target_family="unix")] 
                    _ => None
                }
            }
            Self::BackendError(BackendError::DownloadRepoArchive(_)) => Some("this may be caused by an incorrect URL or trying to use a private repository as a template".to_owned()),
            Self::BackendError(BackendError::TemplateNotFound) => Some("this is likely to be caused by an incorrect template name or URL, or by an external template directory not containing a grafbase directory".to_owned()),
            Self::BackendError(BackendError::PortInUse(_)) => Some("try using a different --port number or supplying the --search flag".to_owned()),
            Self::BackendError(BackendError::ProjectDirectoryExists(_)) => Some("try using a different name for your new project".to_owned()),
            Self::BackendError(BackendError::StartDownloadRepoArchive(_, _)) => Some("this may be caused by connection issues".to_owned()),
            Self::BackendError(BackendError::UnsupportedTemplateURL(_)) => Some("try using a GitHub URL of the following structure: 'https://github.com/org/repo'".to_owned()),
            Self::BackendError(BackendError::MalformedTemplateURL(_)) => Some("try including the URL scheme (e.g. 'https://') and verifying the URL contents".to_owned()),
            Self::CommonError(CommonError::FindGrafbaseDirectory) => Some("try running the CLI in your Grafbase project or any nested directory".to_owned()),
            Self::ServerError(ServerError::NodeInPath) => Some("please install Node.js and make sure it is in your $PATH to continue (https://nodejs.org/en/download/)".to_owned()),
            Self::ServerError(ServerError::OutdatedNode(_, min_version)) => Some(format!("please update your Node.js version to {min_version} or higher to continue (https://nodejs.org/en/download)")),
            _ => None,
        }
    }
}
