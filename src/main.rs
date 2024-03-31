mod utils;
use std::time::Duration;
use reqwest;
use serde_json;
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use winconsole::console::set_title;

const CAT_API: &str = "https://catfact.ninja/fact";
const TRANSLATE_API: &str = "https://ftapi.pythonanywhere.com/translate";

#[derive(Serialize, Deserialize, Debug)]
struct CatResponse{
  fact: String,
  length: i32,
}

#[derive(Serialize, Deserialize, Debug)]
struct TranslateText {
  fact: String
}

#[tokio::main]
async fn main() {
  set_title("Facts About Cats (by FarryDay for practice)").expect("Error");
  println!("------------------[ Факт о кошках ]------------------");
  sleep(Duration::from_secs(2)).await;
  utils::clear_terminal_screen();

  let client = reqwest::Client::new();

  println!("------------------[ Получаем факт ]------------------");

  let res_cat_api = client
    .get(CAT_API).send().await.unwrap();

  let mut fact = match res_cat_api.status(){
    reqwest::StatusCode::OK => {
      let body = res_cat_api.json::<CatResponse>().await.unwrap();
      body.fact
    }
    _ => panic!("Error request to cat API")
  };

  println!("------------------[ Переводим ]------------------");

  let res_translator = client
    .get(format!("{TRANSLATE_API}?sl=en&dl=ru&text={fact}")).send().await.unwrap();

  fact = match res_translator.status(){
    reqwest::StatusCode::OK => {
      let s = res_translator.text().await.unwrap();
      let json_data: serde_json::Value = serde_json::from_str(&s)
        .expect("Can't parse json");
      format!("{}", json_data["destination-text"])
    }
    _ => panic!("Error request to translator API")
  };

  utils::clear_terminal_screen();
  println!("------------------[ Факт о кошках ]------------------");
  println!("{}", &fact[1..fact.len()-1]);

  std::io::stdin().read_line(&mut String::new()).unwrap();
}
