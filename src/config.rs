use crate::cli::Cli;
use std::path::Path;

#[derive(Debug)]
pub struct Config<'a> {
    pub path: &'a Path,
}

impl<'a> TryFrom<&'a Cli> for Config<'a> {
    type Error = String;

    fn try_from(cli: &'a Cli) -> Result<Self, Self::Error> {
        let path = Path::new(&cli.path);

        if !path.exists() {
            return Err(format!("Could not find a file using path {}", &cli.path));
        }

        if !path.is_file() {
            return Err(String::from("Path is not pointing to a file"));
        }

        let extension = path
            .extension()
            .expect("Could not retrieve the file extension");

        if extension != "md" {
            return Err(format!(
                "File extension {:?} is not supported, please provide a markdown (.md) file",
                extension
            ));
        }

        Ok(Self { path })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_creation_fails_when_file_extension_is_unsupported() {
        let file_path_pointing_to_unsupported_file = String::from("./src/test.txt");
        let cli = Cli {
            path: file_path_pointing_to_unsupported_file,
        };
        let result = Config::try_from(&cli);
        assert!(result.is_err());
    }

    #[test]
    fn config_creation_fails_when_file_path_points_to_nothing() {
        let file_path_pointing_to_nothing = String::from("./src/test.js");
        let cli = Cli {
            path: file_path_pointing_to_nothing,
        };
        let result = Config::try_from(&cli);
        assert!(result.is_err());
    }

    #[test]
    fn config_creation_fails_when_path_is_not_pointing_to_a_file() {
        let file_path_pointing_to_directory = String::from("./src");
        let cli = Cli {
            path: file_path_pointing_to_directory,
        };
        let result = Config::try_from(&cli);
        assert!(result.is_err());
    }

    #[test]
    fn config_creation_succeeds_in_retrieving_a_supported_file() {
        let file_path_pointing_to_supported_file = String::from("./src/test.md");
        let cli = Cli {
            path: file_path_pointing_to_supported_file,
        };
        let result = Config::try_from(&cli);
        assert!(result.is_ok());
    }
}
