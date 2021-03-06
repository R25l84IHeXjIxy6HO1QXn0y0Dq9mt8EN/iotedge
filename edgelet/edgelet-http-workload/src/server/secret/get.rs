use crate::IntoResponse;
use crate::error::{Error, ErrorKind};

use edgelet_core::{SecretManager, SecretOperation};
use edgelet_http::Error as HttpError;
use edgelet_http::route::{Handler, Parameters};

use failure::ResultExt;
use futures::{Future, IntoFuture};
use hyper::{Body, Request, Response};
use serde_json::to_string;

pub struct GetSecret<S> {
    secret_manager: S
}

impl<S> GetSecret<S> {
    pub fn new(secret_manager: S) -> Self {
        Self {
            secret_manager
        }
    }
}

impl<S> Handler<Parameters> for GetSecret<S>
where
    S: 'static + SecretManager + Send
{
    fn handle(&self, _req: Request<Body>, params: Parameters) -> Box<dyn Future<Item = Response<Body>, Error = HttpError> + Send> {
        let response = params.name("name")
            .ok_or_else(|| Error::from(ErrorKind::MissingRequiredParameter("name")))
            .and_then(|name| {
                let id = params.name("id")
                    .ok_or_else(|| Error::from(ErrorKind::MissingRequiredParameter("id")))?;
                Ok((name, id))
            })
            .map(|(name, id)| {
                let id = format!("{}/{}", name, id);
                self.secret_manager.get(&id)
                    .then(|result| {
                        let val = result.context(ErrorKind::SecretOperation(SecretOperation::Get(id)))?;
                        Ok(val)
                    })
            })
            .into_future()
            .flatten()
            .and_then(|val| Ok(Response::new(Body::from(to_string(&val).unwrap()))))
            .or_else(|e: Error| Ok(e.into_response()));

        Box::new(response)
    }
}

