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
pub struct EncryptResponse {
  /// The encrypted form of the data encoded in base 64.
  #[serde(rename = "ciphertext")]
  ciphertext: String
}

impl EncryptResponse {
  pub fn new(ciphertext: String) -> EncryptResponse {
    EncryptResponse {
      ciphertext: ciphertext
    }
  }

  pub fn set_ciphertext(&mut self, ciphertext: String) {
    self.ciphertext = ciphertext;
  }

  pub fn with_ciphertext(mut self, ciphertext: String) -> EncryptResponse {
    self.ciphertext = ciphertext;
    self
  }

  pub fn ciphertext(&self) -> &String {
    &self.ciphertext
  }


}



