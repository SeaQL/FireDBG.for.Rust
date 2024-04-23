use crate::{raw, Binary, Dependency, Example, Package, Test, Workspace};

pub(crate) fn parse_workspace(raw_workspace: raw::Workspace) -> Workspace {
    let root_dir = raw_workspace.workspace_root;
    Workspace {
        packages: parse_packages(raw_workspace.packages, &root_dir),
        target_dir: raw_workspace.target_directory,
        root_dir,
    }
}

fn parse_packages(raw_packages: Vec<raw::Package>, workspace_root_dir: &str) -> Vec<Package> {
    let get_package_root_dir = |id: &str| -> String {
        let (_, after) = id.split_once("path+file://").expect("Have path");
        let root_dir = after.chars().take_while(|c| '#'.ne(c)).collect();
        root_dir
    };
    raw_packages
        .into_iter()
        .filter(|raw_package| {
            let package_id = &raw_package.id;
            // Is local package
            if package_id.starts_with("path+file://") {
                let root_dir = get_package_root_dir(&package_id);
                // Is within the root workspace directory
                if root_dir.starts_with(workspace_root_dir) {
                    return true;
                }
            }
            false
        })
        .map(|raw_package| {
            let root_dir = get_package_root_dir(&raw_package.id);
            let has_lib = raw_package
                .targets
                .iter()
                .any(|target| target.kind.contains(&format!("lib")));
            Package {
                name: raw_package.name,
                version: raw_package.version,
                root_dir,
                dependencies: parse_dependencies(raw_package.dependencies),
                binaries: parse_binaries(raw_package.targets.clone()),
                tests: parse_tests(raw_package.targets.clone()),
                examples: parse_examples(raw_package.targets),
                has_lib,
            }
        })
        .collect()
}

fn parse_dependencies(raw_dependencies: Vec<raw::Dependency>) -> Vec<Dependency> {
    raw_dependencies
        .into_iter()
        .filter(|raw_dependency| raw_dependency.path.is_some())
        .map(|raw_dependency| Dependency {
            name: raw_dependency.name,
            default_features: raw_dependency.uses_default_features,
            features: raw_dependency.features,
            root_dir: raw_dependency.path.expect("Have path"),
        })
        .collect()
}

fn parse_binaries(raw_target: Vec<raw::Target>) -> Vec<Binary> {
    raw_target
        .into_iter()
        .filter(|raw_target| raw_target.kind.contains(&format!("bin")))
        .map(|raw_target| Binary {
            name: raw_target.name,
            src_path: raw_target.src_path,
            required_features: raw_target.required_features,
        })
        .collect()
}

fn parse_tests(raw_target: Vec<raw::Target>) -> Vec<Test> {
    raw_target
        .into_iter()
        .filter(|raw_target| raw_target.kind.contains(&format!("test")))
        .map(|raw_target| Test {
            name: raw_target.name,
            src_path: raw_target.src_path,
            required_features: raw_target.required_features,
        })
        .collect()
}

fn parse_examples(raw_target: Vec<raw::Target>) -> Vec<Example> {
    raw_target
        .into_iter()
        .filter(|raw_target| raw_target.kind.contains(&format!("example")))
        .map(|raw_target| Example {
            name: raw_target.name,
            src_path: raw_target.src_path,
            required_features: raw_target.required_features,
        })
        .collect()
}
