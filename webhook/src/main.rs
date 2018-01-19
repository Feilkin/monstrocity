//! Botti webhookki juttu

extern crate telegram;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use telegram::objects::UpdateKind;

#[derive(Debug, Deserialize)]
struct Config {
    bind_address: String,
    redis_address: String,
    // TODO
    //spam_limit: 10,
}

fn main() {
    println!("Hello, world!");

    let mut f = File::open("webhook.toml").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents).expect(
        "something went wrong reading the file",
    );

    let config = toml::from_str::<Config>(&contents).expect("failed to parse config");

    //    fork while fork or die

    // setup the webhook
    let webhook_server = Server::http(&config.bind_address).unwrap();

    let redis_client = redis::Client::open(&config.redis_address).unwrap();
    let con = redis_client.get_connection().unwrap();

    loop {
        // get updates
        while let Ok(Some(mut request)) = webhook_server.try_recv() {
            // TODO: check token so we know it is form Telegram

            let mut body = String::new();
            request.as_reader().read_to_string(&mut body).unwrap();

            let update: objects::UpdateKind = match serde_json::from_str(&body) {
                Ok(update) => update,
                Err(err) => panic!("!!! Failed to parse:\n{:?}\n{:}\n", request, err),
            };

            // TODO: spam filtering

            let response = Response::from_string("").with_status_code(200);
            request.respond(response).unwrap();
        }
    }
}
