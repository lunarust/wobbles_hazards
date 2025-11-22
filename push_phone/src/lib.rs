use reqwest::Client;
use serde_json::{Value};
use serde::{Deserialize};
use std::time::{Duration};

#[derive(Deserialize, Debug)]
#[allow(non_snake_case)]
struct Ret {
    response: String,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
pub async fn push(acct_key: &str, resturl: &str, message: &str, title: &str, priority: &str) -> String {
    let myurl = format!("{}?accountKey={}&title={}&message={}&priority={}",
        resturl, acct_key, title, message, priority);

    let doge: Value = Client::new()
        .post(myurl)
        .timeout(Duration::from_secs(30))
        .send()
        .await
        .expect("failed to get response")
        .json()
        .await
        .expect("failed to get payload");

      let r: Ret = serde_json::from_value(doge).unwrap();
      r.response

}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);

    }
}
