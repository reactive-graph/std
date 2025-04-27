use reactive_graph_utils_deployment::deploy_plugin;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(deploy_plugin("libreactive_graph_std_color.*")?)
}
