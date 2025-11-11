pub fn run_cli() -> bool {
  let mut args = std::env::args();

  if args.nth(1) == None {
    return true;
  }

  let pattern = args.nth(1).unwrap();

  match pattern.as_str() {
    "--open" => open_demo(),
    _ => println!("Unknown command"),
  }

  true
}

fn open_demo() {
  if let None = std::env::args().nth(2) {
    return;
  }

  let demo_path = std::env::args().nth(2).unwrap();
}
