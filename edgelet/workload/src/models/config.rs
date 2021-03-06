/* 
 * IoT Edge Module Workload API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2020-07-22
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
  #[serde(rename = "settings")]
  settings: Value,
  #[serde(rename = "env")]
  env: Option<Vec<::models::EnvVar>>
}

impl Config {
  pub fn new(settings: Value) -> Config {
    Config {
      settings: settings,
      env: None
    }
  }

  pub fn set_settings(&mut self, settings: Value) {
    self.settings = settings;
  }

  pub fn with_settings(mut self, settings: Value) -> Config {
    self.settings = settings;
    self
  }

  pub fn settings(&self) -> &Value {
    &self.settings
  }


  pub fn set_env(&mut self, env: Vec<::models::EnvVar>) {
    self.env = Some(env);
  }

  pub fn with_env(mut self, env: Vec<::models::EnvVar>) -> Config {
    self.env = Some(env);
    self
  }

  pub fn env(&self) -> Option<&Vec<::models::EnvVar>> {
    self.env.as_ref()
  }

  pub fn reset_env(&mut self) {
    self.env = None;
  }

}



