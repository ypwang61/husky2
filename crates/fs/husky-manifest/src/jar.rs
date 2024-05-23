use crate::*;

#[salsa::jar]
pub struct ManifestJar(
    package_manifest_aux,
    PackageManifest,
    PackageDependenciesSection,
    package_dependencies,
    full_dependent_package_paths_aux,
    PackageDevDependenciesSection,
    package_dev_dependencies_unchecked,
    crate::helpers::upstream::linktime_target_path_all_upstream_packages,
);
