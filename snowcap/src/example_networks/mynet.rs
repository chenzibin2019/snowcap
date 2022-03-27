use super::ExampleNetwork;
use crate::hard_policies::HardPolicy;
use crate::netsim::config::{Config, ConfigExpr::*};
use crate::netsim::{AsId, BgpSessionType::*, Network, Prefix};

/// # MyNet
///
/// Zibin Chen
/// Implementing Fully meshed iBGP to Route Reflection case: 
/// @param: initial-variant: number of clusters
/// number of clients are hard coded as 5
/// number of RRs are hard coded as 2
/// 
pub struct MyNet {}

impl ExampleNetwork for MyNet {
    fn net(initial_variant: usize) -> Network {
        let mut net = Network::new();
        let r1 = net.add_router("r1");
        let r2 = net.add_router("r2");
        let e1 = net.add_external_router(format!("e{}", initial_variant), AsId(65101));

        net.add_link(r1, r2);
        net.add_link(r1, e1);

        let cf = Self::initial_config(&net, initial_variant);
        net.set_config(&cf).unwrap();

        // advertise prefixes
        net.advertise_external_route(e1, Prefix(0), vec![AsId(65101), AsId(65200)], None, None)
            .unwrap();

        net
    }

    fn initial_config(net: &Network, variant: usize) -> Config {
        let mut c = Config::new();

        let r1 = net.get_router_id("r1").unwrap();
        let r2 = net.get_router_id("r2").unwrap();
        let e1 = net.get_router_id(format!("e{}", variant)).unwrap();

        c.add( IgpLinkWeight { source: r1, target: r2, weight: 1.0 }).unwrap();
        c.add( IgpLinkWeight { source: r2, target: r1, weight: 1.0 }).unwrap();
        c.add( IgpLinkWeight { source: r1, target: e1, weight: 1.0 }).unwrap();
        c.add( BgpSession { source: r1, target: r2, session_type: IBgpPeer }).unwrap();
        c.add( BgpSession { source: r1, target: e1, session_type: EBgp }).unwrap();

        c
    }

    fn final_config(net: &Network, variant: usize) -> Config {
        let mut c = Config::new();

        let r1 = net.get_router_id("r1").unwrap();
        let r2 = net.get_router_id("r2").unwrap();
        let e1 = net.get_router_id(format!("e{}", variant)).unwrap();

        c.add( IgpLinkWeight { source: r1, target: r2, weight: 1.0 }).unwrap();
        c.add( IgpLinkWeight { source: r2, target: r1, weight: 1.0 }).unwrap();
        c.add( IgpLinkWeight { source: r1, target: e1, weight: 1.0 }).unwrap();
        c.add( BgpSession { source: r1, target: r2, session_type: IBgpClient }).unwrap();
        c.add( BgpSession { source: r1, target: e1, session_type: EBgp }).unwrap();

        c
    }

    fn get_policy(net: &Network, _variant: usize) -> HardPolicy {
        HardPolicy::reachability(net.get_routers().iter(), net.get_known_prefixes().iter())
    }
}