use serde::Deserialize;
use std::collections::HashMap;
use std::env;

#[derive(Deserialize)]
#[allow(non_snake_case)]
struct Package {
    dependencies: Option<HashMap<String, String>>,
    peerDependencies: Option<HashMap<String, String>>,
    optionalDependencies: Option<HashMap<String, String>>,
}

fn get_deps(package_name: String) -> Result<Vec<String>, ureq::Error> {
    let mut deps: Vec<String> = vec![];

    let body: Package = ureq::get(&format!(
        "https://registry.npmjs.com/{}/latest",
        package_name
    ))
    .call()?
    .into_json()?;

    if body.dependencies.is_some() {
        for (dep, _) in body.dependencies.unwrap() {
            if !deps.contains(&dep) {
                deps.push(dep);
            }
        }
    }

    if body.peerDependencies.is_some() {
        for (dep, _) in body.peerDependencies.unwrap() {
            if !deps.contains(&dep) {
                deps.push(dep);
            }
        }
    }

    if body.optionalDependencies.is_some() {
        for (dep, _) in body.optionalDependencies.unwrap() {
            if !deps.contains(&dep) {
                deps.push(dep);
            }
        }
    }

    Ok(deps)
}

fn main() -> Result<(), ureq::Error> {
    let args: Vec<String> = env::args().collect();
    let package_name = args[1].to_string();
    println!("Counting dependencies...");

    let mut pkg_deps = get_deps(package_name.to_string())?;
    let mut pkg_count = pkg_deps.len();
    let mut d = 0;

    while d < pkg_count {
        let deps = get_deps(pkg_deps[d].to_string())?;

        for i in deps {
            if !pkg_deps.contains(&i) {
                pkg_deps.push(i);
                pkg_count += 1;
            }
        }

        d += 1;
    }

    let deps_count = pkg_deps.len();

    let mut plural = "dependencies";
    if deps_count == 1 {
        plural = "dependency";
    }

    println!("The {package_name} package has {deps_count} {plural}.",);

    Ok(())
}
