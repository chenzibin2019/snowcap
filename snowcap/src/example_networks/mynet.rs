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
        let mut routers = Vec::new();
        let mut cf = Config::new();
        // process each cluster
        for c in 1..initial_variant + 1 {
            // generate RRs
            for r in 1..3 {
                let name = format!("r_{}_{}", c, r);
                let rr = net.add_router(name);
                routers.push(rr);
            }
            // generate RCs
            for cn in 1..6 {
                let name = format!("c_{}_{}", c, cn);
                let rc = net.add_router(name);
                routers.push(rc);
            }
        }

        // fully meshed link connection
        for r1 in &routers {
            for r2 in &routers {
                if r1.index() > r2.index() {
                    net.add_link(*r1, *r2);
                    cf.add(IgpLinkWeight { source: *r1, target: *r2, weight: 1.0 }).unwrap();
                    cf.add(IgpLinkWeight { source: *r2, target: *r1, weight: 1.0 }).unwrap();
                    cf.add(BgpSession { source: *r1, target: *r2, session_type: IBgpPeer })
                        .unwrap();
                }
            }
        }
        // external routers
        let egress = net.get_router_id("c_1_1").unwrap();
        let ext = net.add_external_router("ext", AsId(56789));
        net.add_link(egress, ext);
        cf.add(IgpLinkWeight { source: egress, target: ext, weight: 1.0 }).unwrap();
        cf.add(IgpLinkWeight { target: egress, source: ext, weight: 1.0 }).unwrap();
        cf.add(BgpSession { source: egress, target: ext, session_type: EBgp }).unwrap();

        // let cf = Self::initial_config(&net, initial_variant);
        net.set_config(&cf).unwrap();

        // advertise prefixes
        net.advertise_external_route(ext, Prefix(0), vec![AsId(56789), AsId(65200)], None, None)
            .unwrap();

        net
    }

    fn initial_config(_net: &Network, _variant: usize) -> Config {
        todo!();
    }

    fn final_config(net: &Network, variant: usize) -> Config {
        let mut cf = Config::new();
        let mut routers = Vec::new();

        for c in 1..variant + 1 {
            // get RRs
            for r in 1..3 {
                let rr = net.get_router_id(format!("r_{}_{}", c, r)).unwrap();
                routers.push(rr);
                // todo 1: connect with RR within cluster w/ smaller client id
                for intra_rr_peer in 1..r {
                    let peer = net.get_router_id(format!("r_{}_{}", c, intra_rr_peer)).unwrap();
                    // cf.add(IgpLinkWeight { source: rr, target: peer, weight: 1.0 }).unwrap();
                    // cf.add(IgpLinkWeight { target: rr, source: peer, weight: 1.0 }).unwrap();
                    cf.add(BgpSession { source: rr, target: peer, session_type: IBgpPeer })
                        .unwrap();
                }
                // todo 2: connect with ALL RRs external w/ smaller cluster id
                for c_peer in 1..c {
                    for inter_rr_peer in 1..3 {
                        let peer =
                            net.get_router_id(format!("r_{}_{}", c_peer, inter_rr_peer)).unwrap();
                        // cf.add(IgpLinkWeight { source: rr, target: peer, weight: 1.0 }).unwrap();
                        // cf.add(IgpLinkWeight { target: rr, source: peer, weight: 1.0 }).unwrap();
                        cf.add(BgpSession { source: rr, target: peer, session_type: IBgpPeer })
                            .unwrap();
                    }
                }
                // todo 3: connect with ALL RCs internal
                for intra_client_peer in 1..6 {
                    let peer = net.get_router_id(format!("c_{}_{}", c, intra_client_peer)).unwrap();
                    // cf.add(IgpLinkWeight { source: rr, target: peer, weight: 1.0 }).unwrap();
                    // cf.add(IgpLinkWeight { target: rr, source: peer, weight: 1.0 }).unwrap();
                    cf.add(BgpSession { source: rr, target: peer, session_type: IBgpClient })
                        .unwrap();
                }
            }
            // get RCs
            for cn in 1..6 {
                let rc = net.get_router_id(format!("c_{}_{}", c, cn)).unwrap();
                routers.push(rc);
            }
        }

        for r1 in &routers {
            for r2 in &routers {
                if r1.index() != r2.index() {
                    cf.add(IgpLinkWeight { source: *r1, target: *r2, weight: 1.0 }).unwrap();
                }
            }
        }

        // external routers
        let egress = net.get_router_id("c_1_1").unwrap();
        let ext = net.get_router_id("ext").unwrap();
        cf.add(IgpLinkWeight { source: egress, target: ext, weight: 1.0 }).unwrap();
        cf.add(IgpLinkWeight { target: egress, source: ext, weight: 1.0 }).unwrap();
        cf.add(BgpSession { source: egress, target: ext, session_type: EBgp }).unwrap();

        cf
    }

    fn get_policy(net: &Network, _variant: usize) -> HardPolicy {
        HardPolicy::reachability(net.get_routers().iter(), net.get_known_prefixes().iter())
    }
}
