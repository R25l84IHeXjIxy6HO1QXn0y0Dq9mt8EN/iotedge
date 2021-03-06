/* 
 * IoT Edge Management API
 *
 * No description provided (generated by Swagger Codegen https://github.com/swagger-api/swagger-codegen)
 *
 * OpenAPI spec version: 2020-07-22
 * 
 * Generated by: https://github.com/swagger-api/swagger-codegen.git
 */

use std::borrow::Borrow;
use std::sync::Arc;

use hyper;
use serde_json;
use futures;
use futures::{Future, Stream};

use super::{Error, configuration};

pub struct DeviceActionsApiClient<C: hyper::client::connect::Connect> {
    configuration: Arc<configuration::Configuration<C>>,
}

impl<C: hyper::client::connect::Connect> DeviceActionsApiClient<C> {
    pub fn new(configuration: Arc<configuration::Configuration<C>>) -> DeviceActionsApiClient<C> {
        DeviceActionsApiClient {
            configuration: configuration,
        }
    }
}

pub trait DeviceActionsApi: Send + Sync {
    fn reprovision_device(&self, api_version: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>> + Send>;
}


impl<C: hyper::client::connect::Connect + 'static> DeviceActionsApi for DeviceActionsApiClient<C> {
    fn reprovision_device(&self, api_version: &str) -> Box<dyn Future<Item = (), Error = Error<serde_json::Value>> + Send> {
        let configuration: &configuration::Configuration<C> = self.configuration.borrow();

        let method = hyper::Method::POST;

        let query = ::url::form_urlencoded::Serializer::new(String::new())
            .append_pair("api-version", &api_version.to_string())
            .finish();
        let uri_str = format!("{}/device/reprovision{}", configuration.base_path, query);

        let uri: hyper::Uri = uri_str.parse().unwrap();
        // TODO(farcaller): handle error
        // if let Err(e) = uri {
        //     return Box::new(futures::future::err(e));
        // }
        let mut builder = hyper::Request::builder();

        builder.method(method);
        builder.uri(uri);

        if let Some(ref user_agent) = configuration.user_agent {
            builder.header(hyper::header::USER_AGENT, hyper::header::HeaderValue::from_str(user_agent).unwrap());
        }


        let req = builder
            .body(hyper::Body::empty())
            .expect("could not build hyper::Request");

        // send request
        Box::new(
        configuration.client.request(req)
            .map_err(|e| Error::from(e))
            .and_then(|resp| {
                let status = resp.status();
                resp.into_body().concat2()
                    .and_then(move |body| Ok((status, body)))
                    .map_err(|e| Error::from(e))
            })
            .and_then(|(status, body)| {
                if status.is_success() {
                    Ok(body)
                } else {
                    Err(Error::from((status, &*body)))
                }
            })
            .and_then(|_| futures::future::ok(()))
        )
    }

}
