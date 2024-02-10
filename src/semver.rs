use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SemVer {
    major: u16,
    minor: u16,
    patch: u16,
}

impl SemVer {
    pub fn validate_str(sem_ver: &str) -> bool {
        let mut is_valid = true;
        for (i, sv) in sem_ver.split('.').enumerate() {
            match sv.parse::<u16>() {
                Ok(_) => (),
                Err(err) => {
                    if i <= 1 {
                        println!("Semver at index: {} could not be converted to u16", i);
                        println!("Err message: {}", err);
                        is_valid = false;
                    }
                }
            }
        }
        is_valid
    }

    pub fn from_str(sem_ver: &str) -> Option<Self> {
        if !(SemVer::validate_str(sem_ver)) {
            return None;
        }

        let components: Vec<u16> = sem_ver
            .split('.')
            .map(|sv: &str| sv.parse::<u16>().unwrap_or(0))
            .collect();

        match components.len() {
            3 => Some(Self {
                major: components[0],
                minor: components[1],
                patch: components[2],
            }),
            _ => Some(Self {
                major: components[0],
                minor: components[1],
                patch: 0,
            }),
        }
    }
}

impl fmt::Display for SemVer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.patch == 0 {
            return write!(f, "{}.{}", self.major, self.minor);
        }
        write!(f, "{}.{}.{}", self.major, self.minor, self.patch)
    }
}
