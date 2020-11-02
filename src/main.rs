extern crate base64;
extern crate json;

fn main() {
  let args = kubesecret::read_args();
  let secret_name = args.value_of("secretName").unwrap();

  let cmd_output = kubesecret::kube::get_secret(secret_name);
  
  if cmd_output == "" {
    eprintln!("No such secret '{}'", secret_name);
    std::process::exit(1);
  }

  let parsed_json = json::parse(&cmd_output).unwrap();
  let secret_data = &parsed_json["data"];

  if args.is_present("secretKey") {
    let secret_key = args
      .values_of("secretKey")
      .unwrap()
      .map(|k| k.to_ascii_uppercase())
      .collect::<Vec<String>>()
      .join("_");

    if !secret_data.has_key(&secret_key) {
      eprintln!("Secret ({}) does not have '{}'", secret_name, secret_key);
      std::process::exit(1);
    }

    print!("{}", b64_encoded_json_value_to_string(&secret_data[secret_key]));
    std::process::exit(0);
  }

  for entry in secret_data.entries() {
    let value = b64_encoded_json_value_to_string(entry.1);
    print!("{}: {}", entry.0, value);
  }
}

fn b64_encoded_json_value_to_string(json: &json::JsonValue) -> String {
  let base64_encoded_value = json.as_str().expect("test");
  let decoded_value = base64::decode(base64_encoded_value).unwrap();

  return String::from_utf8_lossy(&decoded_value).into_owned();
}
