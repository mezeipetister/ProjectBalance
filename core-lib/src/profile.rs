use crate::files::*;
use std::fs;
use std::fs::File;
use std::path::Path;

// We should extend it later
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
pub struct Profile {
    name: String,
    alias: String, // This will be the path name
}

impl Profile {
    // New profile
    pub fn new(name: String, alias: String) -> Self {
        Profile { name, alias }
    }
    // Save profil
    pub fn save(&self) {
        // Profile path to work with
        let profile_path = &get_home_path()
            .unwrap()
            .join(".ledger")
            .join(format!("{}", self.alias));
        // Create profile folder!
        fs::create_dir_all(profile_path).expect("Error while creating profile path");
        write_string_to_file(
            &mut create_file_from_path(&profile_path.join("profile.yaml"))
                .expect("Error while create file by path"),
            &serde_yaml::to_string(self).expect("Error while trying encode profile during save"),
        )
        .expect("Error while writing string to file during save profile");
    }

    // Get profile name
    pub fn get_name(&self) -> &String {
        &self.name
    }

    // Clear profil,
    // Delete all the related data
    pub fn clear() {}

    // Init
    pub fn init() {}

    // Rename
    pub fn rename(&mut self, name: String) {
        self.name = name
    }
}

pub struct Profiles {
    profiles: Vec<Profile>,
}

impl Profiles {
    // Init profiles
    pub fn init() -> Self {
        // Declare an empty profiles
        let mut profiles = Profiles {
            profiles: Vec::new(),
        };

        // Init path
        let path = get_home_path().unwrap().join(".ledger");

        // Create home path in case of it does not exist.
        fs::create_dir_all(&path).expect("Error while creating ledger core path");

        // Get a list of the app home folder content.
        let folder_content = get_files_from_dir(&path);

        // Iterate over the folder contents.
        for item in folder_content {
            // Declare a possible profile folder
            let possible_profile = Path::new(&path).join(item);

            // Check wheter its a folder or not.
            if possible_profile.is_dir() {
                // If its a folder, then try to init it it
                try_init_from_path(&possible_profile, &mut profiles);
            }
        }

        // Return profiles
        profiles
    }

    pub fn get_profiles(&self) -> &Vec<Profile> {
        &self.profiles
    }
}

// Try to init a folder
pub fn try_init_from_path(path: &Path, profiles: &mut Profiles) {
    // Declare how to find profile yaml
    let profile_path = path.join("profile.yaml");

    // Check if profile file exists in the given folder
    if profile_path.exists() {
        // Try to read and decode profile information
        let profile: Profile = serde_yaml::from_str(
            &read_file_to_string(
                &mut File::open(&profile_path).expect("Error while trying to open profile."),
            )
            .expect("Error while trying to read profile from file path"),
        )
        .expect("Error during profile decoding");

        // Push read profile into profiles holder
        profiles.profiles.push(profile);
    }
}
