use super::ExampleNetwork;
use crate::hard_policies::HardPolicy;
use crate::netsim::config::{Config, ConfigExpr::*};
use crate::netsim::{AsId, BgpSessionType::*, Network, Prefix};

/// # Multi AS
/// Zibin Chen
/// Implement multi-as verification: test if it works
pub struct MultiAS {}

impl ExampleNetwork for MultiAS {
    fn net(_initial_variant: usize) -> Network {
        let mut net = Network::new();
        let r1 = net.add_router_asid("r1", AsId(123));
        let r2 = net.add_router_asid("r2", AsId(456));

        let ext = net.add_external_router("ext", AsId(999));

        net.add_link(r1, r2);
        net.add_link(r1, ext);

        let mut cf = Config::new();
        cf.add(IgpLinkWeight { source: r1, target: r2, weight: 1.0 }).unwrap();
        cf.add(IgpLinkWeight { source: r2, target: r1, weight: 1.0 }).unwrap();
        cf.add(BgpSession { source: r1, target: r2, session_type: EBgp }).unwrap();
        //cf.add(BgpSession { source: r2, target: r1, session_type: EBgp }).unwrap();
        cf.add(IgpLinkWeight { source: r1, target: ext, weight: 1.0 }).unwrap();
        cf.add(IgpLinkWeight { source: ext, target: r1, weight: 1.0 }).unwrap();
        cf.add(BgpSession { source: r1, target: ext, session_type: EBgp }).unwrap();

        net.set_config(&cf).unwrap();

        net.advertise_external_route(ext, Prefix(0), vec![AsId(999), AsId(483)], None, None)
            .unwrap();

        net
    }

    /// Get the initial configuration
    fn initial_config(_net: &Network, _variant: usize) -> Config {
        todo!()
    }

    /// Get the final configuration
    fn final_config(net: &Network, _variant: usize) -> Config {
        let mut cf = Config::new();

        let r1 = net.get_router_id("r1").unwrap();
        let r2 = net.get_router_id("r2").unwrap();

        let ext = net.get_router_id("ext").unwrap();

        cf.add(IgpLinkWeight { source: r1, target: r2, weight: 1.0 }).unwrap();
        cf.add(IgpLinkWeight { source: r2, target: r1, weight: 1.0 }).unwrap();
        cf.add(BgpSession { source: r1, target: r2, session_type: EBgp }).unwrap();
        // cf.add(BgpSession { source: r2, target: r1, session_type: EBgp }).unwrap();
        cf.add(IgpLinkWeight { source: r1, target: ext, weight: 1.0 }).unwrap();
        cf.add(IgpLinkWeight { source: ext, target: r1, weight: 1.0 }).unwrap();
        cf.add(BgpSession { source: r1, target: ext, session_type: EBgp }).unwrap();

        cf
    }

    /// Get the hard policies.
    fn get_policy(net: &Network, _variant: usize) -> HardPolicy {
        HardPolicy::reachability(net.get_routers().iter(), net.get_known_prefixes().iter())
    }
}
