use menu::libra::Libra;
use anyhow::{anyhow, Result};
use menu::backend::{ConfigBackend, BACKEND_URL};
use menu::device::Device;
use menu::read::Read;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};

fn main() -> Result<()> {
    let home = std::env::home_dir().ok_or(anyhow!("no home directory"))?;
    let path = home.join(".config/libra/config.toml");
    let auth_token = std::env::var("AUTH_TOKEN")?;
    // let libras = Libra::read_as_vec(&path)?;
    let backend = ConfigBackend::new(BACKEND_URL.into(), auth_token);
    // let addresses: Result<Vec<String>> = libras.iter().map(|libra| {
    //     backend.get_address(libra.device.clone()).map_err(anyhow::Error::from)
    // }).collect();
    // println!("{:?}", addresses?);
    get_ip_addr();
    Ok(())
}

fn get_ip_addr() {
    println!("Scanning for a wireless IP address...");

    let network_interfaces = match NetworkInterface::show() {
        Ok(interfaces) => interfaces,
        Err(e) => {
            eprintln!("Error getting network interfaces: {}", e);
            return;
        }
    };

    // On many Linux systems, wireless interface names start with "wlan" or "wlp".
    // We filter interfaces based on this naming convention to find the wireless one,
    // then find its IPv4 address.
    let wireless_ip = network_interfaces
        .iter()
        .filter(|iface| iface.name.starts_with("wl")) // Filter for wireless interfaces by name convention
        .flat_map(|iface| &iface.addr) // Get all addresses from those interfaces
        .find(|addr| addr.ip().is_ipv4()) // Find the first IPv4 address
        .map(|addr| addr.ip());

    match wireless_ip {
        Some(ip) => println!("Found wireless IPv4 address: {}", ip),
        None => {
            println!("Could not find a wireless IPv4 address.");
            println!("Interfaces: {:?}", network_interfaces);
        },
    }
}