/* 
 * IoT Edge Management API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2020-07-22
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */


#[allow(unused_imports)]
use serde_json::Value;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SystemInfo {
  #[serde(rename = "osType")]
  os_type: String,
  #[serde(rename = "architecture")]
  architecture: String,
  #[serde(rename = "version")]
  version: Option<String>
}

impl SystemInfo {
  pub fn new(os_type: String, architecture: String) -> SystemInfo {
    SystemInfo {
      os_type: os_type,
      architecture: architecture,
      version: None
    }
  }

  pub fn set_os_type(&mut self, os_type: String) {
    self.os_type = os_type;
  }

  pub fn with_os_type(mut self, os_type: String) -> SystemInfo {
    self.os_type = os_type;
    self
  }

  pub fn os_type(&self) -> &String {
    &self.os_type
  }


  pub fn set_architecture(&mut self, architecture: String) {
    self.architecture = architecture;
  }

  pub fn with_architecture(mut self, architecture: String) -> SystemInfo {
    self.architecture = architecture;
    self
  }

  pub fn architecture(&self) -> &String {
    &self.architecture
  }


  pub fn set_version(&mut self, version: String) {
    self.version = Some(version);
  }

  pub fn with_version(mut self, version: String) -> SystemInfo {
    self.version = Some(version);
    self
  }

  pub fn version(&self) -> Option<&String> {
    self.version.as_ref()
  }

  pub fn reset_version(&mut self) {
    self.version = None;
  }

}



