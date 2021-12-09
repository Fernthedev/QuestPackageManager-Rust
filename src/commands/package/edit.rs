use clap::{AppSettings, Clap};
use semver::Version;

use crate::data::{package::PackageConfig, shared_package::SharedPackageConfig};

#[derive(Clap, Debug, Clone)]
#[clap(setting = AppSettings::ColoredHelp)]
pub struct EditArgs {
    ///Edit the id property of the package
    #[clap(long)]
    pub id: Option<String>,
    ///Edit the name property of the package
    #[clap(long)]
    pub name: Option<String>,
    ///Edit the url property of the package
    #[clap(long)]
    pub url: Option<String>,
    ///Edit the version property of the package
    #[clap(long)]
    pub version: Option<Version>,
}

pub fn package_edit_operation(edit_parameters: EditArgs) {
    let mut package = PackageConfig::read();
    let mut any_changed = false;
    if let Some(id) = edit_parameters.id {
        package_set_id(&mut package, id);
        any_changed = true;
    }
    if let Some(name) = edit_parameters.name {
        package_set_name(&mut package, name);
        any_changed = true;
    }
    if let Some(url) = edit_parameters.url {
        package_set_url(&mut package, url);
        any_changed = true;
    }
    if let Some(version) = edit_parameters.version {
        package_set_version(&mut package, version);
        any_changed = true;
    }

    if any_changed {
        package.write();
        let mut shared_package = SharedPackageConfig::read();
        shared_package.config = package;
        shared_package.write();

        // TODO: Edit qpm defines.cmake
    }
}

fn package_set_id(package: &mut PackageConfig, id: String) {
    println!("Setting package id: {}", id);
    package.info.id = id;
}

fn package_set_name(package: &mut PackageConfig, name: String) {
    println!("Setting package name: {}", name);
    package.info.name = name;
}

fn package_set_url(package: &mut PackageConfig, url: String) {
    println!("Setting package url: {}", url);
    package.info.url = Option::Some(url);
}

fn package_set_version(package: &mut PackageConfig, version: Version) {
    println!("Setting package version: {}", version);
    package.info.version = version;
}
