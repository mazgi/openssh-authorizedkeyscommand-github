use async_std::task;
use docopt::Docopt;
use serde::Deserialize;

const USAGE: &'static str = "
OpenSSH AuthorizedKeysCommand GitHub

Usage:
  openssh-authorizedkeyscommand-github --user=<user>
  openssh-authorizedkeyscommand-github (--help | --version)

Options:
  --user=<user>   GitHub user.
  -V, --version   Show version.
  -h, --help      Show this screen.
";

#[derive(Debug, Deserialize)]
struct Args {
  flag_version: bool,
  flag_user: String,
}

fn get_user_pubkeys(user: &str) -> Result<(), surf::Exception> {
  #[derive(Deserialize)]
  struct PubKey {
    pub id: u64,
    pub key: String,
  }

  task::block_on(async {
    let uri = format!("https://api.github.com/users/{user}/keys", user = user);
    let pubkeys: Vec<PubKey> = surf::get(uri).recv_json().await?;
    for pubkey in &pubkeys {
      println!("{}", pubkey.key)
    }
    Ok(())
  })
}

fn main() {
  let args: Args = Docopt::new(USAGE)
    .and_then(|d| d.deserialize())
    .unwrap_or_else(|e| e.exit());

  if args.flag_version {
    println!("{} {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
    std::process::exit(0)
  }

  std::process::exit(match get_user_pubkeys(&args.flag_user) {
    Ok(_) => 0,
    Err(err) => {
      eprintln!("error: {:?}", err);
      1
    }
  });
}
