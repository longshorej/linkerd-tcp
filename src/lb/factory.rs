use super::DstAddr;
use super::balancer::{self, Balancer};
use super::connector::ConnectorFactory;
use super::endpoint::Endpoint;
use super::super::{ConfigError, Path, resolver};
use ordermap::OrderMap;
use std::cell::RefCell;
use std::collections::VecDeque;
use std::rc::Rc;

pub fn new(min_conns: usize, max_waiters: usize, cf: ConnectorFactory) -> BalancerFactory {
    BalancerFactory {
        minimum_connections: min_conns,
        maximum_waiters: max_waiters,
        connector_factory: Rc::new(RefCell::new(cf)),
    }
}

#[derive(Clone)]
pub struct BalancerFactory {
    minimum_connections: usize,
    maximum_waiters: usize,
    connector_factory: Rc<RefCell<ConnectorFactory>>,
}
impl BalancerFactory {
    pub fn mk_balancer(&self,
                       dst_name: &Path,
                       init: resolver::Result<Vec<DstAddr>>)
                       -> Result<Balancer, ConfigError> {
        let connector = self.connector_factory.borrow().mk_connector(dst_name)?;

        let b = balancer::new(dst_name.clone(),
                              self.minimum_connections,
                              self.maximum_waiters,
                              connector,
                              init);
        Ok(b)
    }
}