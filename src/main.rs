
mod argman;

fn create_global_args() -> argman::ArgMan {
    let mut g_args = argman::ArgMan::new();
    g_args.add_arg_multi("-chain", vec!["regtest".to_string()],
                   "Selected chain to operate with (Can be repeated to operate with several chains simultaneously)");
    g_args.add_arg("-p2phost", "localhost:9999".to_string(),
                   "Address to listen to as a p2p lightning node");
    g_args.add_arg_unset("-rpchost",
                   "bitcoind RPC host to connect to ");
    g_args.add_arg_unset("-rpcuser",
                   "bitcoind RPC username");
    g_args.add_arg_unset("-rpcpass",
                   "bitcoind RPC password");
    g_args.add_arg_bool("-daemon", "0".to_string(),
                   "Run in background");

    g_args
}

fn sim_get_arg(g_args: &argman::ArgMan, arg_name: &str) {
    println!("g_args.get_arg(\"{}\"): {:?}", arg_name, g_args.get(arg_name));
}

fn sim_get_arg_multi(g_args: &argman::ArgMan, arg_name: &str) {
    println!("g_args.get_arg(\"{}\"): {:?}", arg_name, g_args.get_multi(arg_name));
}

fn main() {

    let mut g_args = create_global_args();
    if !g_args.parse_args() {
        println!("\nThe daemon is not running.");
        return;
    }

    println!("\nStarting daemon....");
    g_args.dev_print_selected_args();

    println!("\nLet's simulate getting some options as if we were in the code...\n");
    sim_get_arg_multi(&g_args, "-chain");
    sim_get_arg(&g_args, "-p2phost");
    if !g_args.is_none("-rpchost") {
        sim_get_arg(&g_args, "-rpchost");
    }

    if g_args.get_bool("-daemon") {
        println!("\nRunning the daemon in the background...");
    }

}
