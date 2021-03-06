/* 
 * Secret Service
 *
 * IoT Edge Secret Service API
 *
 * OpenAPI spec version: 2020-07-22
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use hyper;

pub struct Configuration<C: hyper::client::connect::Connect> {
  pub base_path: String,
  pub user_agent: Option<String>,
  pub client: hyper::client::Client<C>,
}

impl<C: hyper::client::connect::Connect> Configuration<C> {
  pub fn new(client: hyper::client::Client<C>) -> Configuration<C> {
    Configuration {
      base_path: "http://localhost".to_owned(),
      user_agent: Some("Swagger-Codegen/2020-07-22/rust".to_owned()),
      client: client,
    }
  }
}
