mod cli;
mod logger;
mod auditor;
mod signer;

fn main() {
    let cli = cli::build_cli();
    let matches = cli.get_matches();

    match matches.subcommand() {
        Some(("record", sub_m)) => logger::handle_record(sub_m),
        Some(("audit", sub_m)) => auditor::handle_audit(sub_m),
        
        Some(("hash", sub_m)) => signer::handle_hash(sub_m),
        Some(("sign", sub_m)) => {
            let file = sub_m.get_one::<String>("file").unwrap();
            signer::sign_file(file)
        }
        Some(("keygen", _)) => signer::generate_keypair(),
        _ => eprintln!("Invalid command. Use --help."),
    }
}

