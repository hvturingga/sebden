use domain::wellknown::version::Version;

pub struct VersionService;

impl VersionService {
    pub fn get_version() -> Version {
        Version::new()
    }
}