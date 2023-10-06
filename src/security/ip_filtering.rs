use super::error::AuthorizationError;
use actix_web::dev::{forward_ready, Service, ServiceResponse, Transform};
use actix_web::{dev::ServiceRequest, Error};

use ipnet::Ipv4Net;
use iprange::IpRange;
use std::net::Ipv4Addr;

use std::env;
use std::{
    future::{ready, Future, Ready},
    pin::Pin,
    rc::Rc,
};
pub struct IPFiltering;

// `S` - type of the next service
// `B` - type of response's body
impl<S, B> Transform<S, ServiceRequest> for IPFiltering
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = IPFilteringMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(IPFilteringMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct IPFilteringMiddleware<S> {
    /// The next service to call
    service: Rc<S>,
}

// This future doesn't have the requirement of being `Send`.
// See: futures_util::future::LocalBoxFuture
type LocalBoxFuture<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

// `S`: type of the wrapped service
// `B`: type of the body - try to be generic over the body where possible
impl<S, B> Service<ServiceRequest> for IPFilteringMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<Result<Self::Response, Self::Error>>;

    // This service is ready when its next service is ready
    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        log::debug!(
            "IP Filtering security control. You requested: {}",
            req.path()
        );

        let ips = env::var("AUTHORIZED_IP_RANGES").expect("$AUTHORIZED_IP_RANGES is not set");

        let ips: Vec<&str> = ips.split(",").collect();
        let iprange: IpRange<Ipv4Net> = ips.iter().map(|s| s.parse().unwrap()).collect();

        let svc = self.service.clone();

        Box::pin(async move {
            let _peer_addr = match req.peer_addr() {
                Some(ip) => ip.ip().to_string(),
                None => "Unknown".to_string(),
            };

            let conn_info = req.connection_info().clone();
            let real_remote_addr = conn_info.realip_remote_addr().unwrap_or("unknown");

            let success = iprange.contains(&real_remote_addr.parse::<Ipv4Addr>().unwrap());

            match success {
                true => (),
                false => {
                    log::error!(
                        "Unauthorized access attempt for ip {} for {}",
                        real_remote_addr,
                        req.path()
                    );
                    return Err(Error::from(AuthorizationError::InvalidIPAddress).into());
                }
            }
            log::info!(
                "Authorized access attempt for ip {} for {}",
                real_remote_addr,
                req.path()
            );

            let res = svc.call(req).await?;

            Ok(res)
        })
    }
}
