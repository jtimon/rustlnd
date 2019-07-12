
mod argman;

use std::collections::HashMap;

fn create_global_args() -> argman::ArgMan {
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_multi("-chain", vec!["regtest".to_string()],
                   "Selected chain to operate with (Can be repeated to operate with several chains simultaneously)");

    // Common arguments
    g_args.add_arg("-p2phost", "localhost:9999".to_string(),
                   "Address to listen to as a p2p lightning node");
    g_args.add_arg_bool("-daemon", "0".to_string(),
                   "Run in background");

    // Per chain arguments
    let default_empty: HashMap<String, String> = HashMap::new();
    g_args.add_arg_with_category("-rpcuser", default_empty.clone(),
                   "bitcoind RPC username");
    g_args.add_arg_with_category("-rpcpass", default_empty,
                   "bitcoind RPC password");
    let mut default_host: HashMap<String, String> = HashMap::new();
    default_host.insert("main".to_string(), "localhost:8332".to_string());
    default_host.insert("test".to_string(), "localhost:18332".to_string());
    default_host.insert("regtest".to_string(), "localhost:18443".to_string());
    g_args.add_arg_with_category("-rpchost", default_host,
                   "bitcoind RPC host to connect to");

    g_args
}

fn main() {

    let mut g_args = create_global_args();
    if !g_args.parse_args() {
        println!("\nThe daemon is not running.");
        return;
    }

    println!("\nStarting daemon...");
    g_args.dev_print_selected_args();

    if g_args.get_bool("-daemon") {
        println!("\nRunning the daemon in the background...");
        // TODO actually run in the background
    }

    let chains = g_args.get_multi("-chain");
    for chain in chains {
        println!("\nConnecting to chain {}'s daemon in host {}", chain, g_args.get_by_category(chain, "-rpchost"));
        println!("rpchost: {:?}", g_args.get_by_category(chain, "-rpchost"));
        println!("rpcuser: {:?}", g_args.get_by_category(chain, "-rpcuser"));
        println!("rpcpass: {:?}", g_args.get_by_category(chain, "-rpcpass"));
        // TODO ping the daemon for every chain via rpc and store things for convenience
    }

}
