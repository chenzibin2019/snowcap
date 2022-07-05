use super::ExampleNetwork;
use crate::hard_policies::Condition::*;
use crate::hard_policies::HardPolicy;
use crate::hard_policies::*;
use crate::netsim::config::{Config, ConfigExpr::*};
use crate::netsim::{AsId, BgpSessionType::*, Network, Prefix};
use rand::prelude::*;
use std::collections::HashMap;

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
        let mut id_name_map = HashMap::new();
        // process each cluster
        for c in 1..initial_variant + 1 {
            // generate RRs
            for r in 1..3 {
                let name = format!("r_{}_{}", c, r);
                let rr = net.add_router(name);
                routers.push(rr);
                id_name_map.insert(rr, false);
            }
            // generate RCs
            for cn in 1..6 {
                let name = format!("c_{}_{}", c, cn);
                let rc = net.add_router(name);
                routers.push(rc);
                id_name_map.insert(rc, true);
            }
        }

        // fully meshed link connection
        for r1 in &routers {
            for r2 in &routers {
                if r1.index() > r2.index() {
                    net.add_link(*r1, *r2);
                    let mut w: f32 = 1.0;
                    if *id_name_map.get(r1).unwrap() && *id_name_map.get(r2).unwrap() {
                        w = rand::thread_rng().gen_range(1, 5) as f32;
                    }
                    cf.add(IgpLinkWeight { source: *r1, target: *r2, weight: w }).unwrap();
                    cf.add(IgpLinkWeight { source: *r2, target: *r1, weight: w }).unwrap();
                    cf.add(BgpSession { source: *r1, target: *r2, session_type: IBgpPeer })
                        .unwrap();
                }
            }
        }
        // external routers
        for e in 1..initial_variant + 1 {
            let egress = net.get_router_id(format!("c_{}_1", e)).unwrap();
            let ext = net.add_external_router(format!("EXT_{}", e), AsId(56789));
            net.add_link(egress, ext);
            cf.add(IgpLinkWeight { source: egress, target: ext, weight: 1.0 }).unwrap();
            cf.add(IgpLinkWeight { target: egress, source: ext, weight: 1.0 }).unwrap();
            cf.add(BgpSession { source: egress, target: ext, session_type: EBgp }).unwrap();
            let mut path = vec![AsId(56789)];
            if e > 1 {
                path.push(AsId(34567));
            }
            net.advertise_external_route(ext, Prefix(0), path, None, None).unwrap();
        }
        // let egress = net.get_router_id("c_1_1").unwrap();
        // let ext = net.add_external_router("ext", AsId(56789));
        // net.add_link(egress, ext);
        // cf.add(IgpLinkWeight { source: egress, target: ext, weight: 1.0 }).unwrap();
        // cf.add(IgpLinkWeight { target: egress, source: ext, weight: 1.0 }).unwrap();
        // cf.add(BgpSession { source: egress, target: ext, session_type: EBgp }).unwrap();

        // let cf = Self::initial_config(&net, initial_variant);
        net.set_config(&cf).unwrap();

        // advertise prefixes

        net
    }

    fn initial_config(_net: &Network, _variant: usize) -> Config {
        todo!();
    }

    fn final_config(net: &Network, variant: usize) -> Config {
        let mut cf = Config::new();
        let mut routers = Vec::new();
        let mut id_name_map = HashMap::new();

        for c in 1..variant + 1 {
            // get RRs
            for r in 1..3 {
                let rr = net.get_router_id(format!("r_{}_{}", c, r)).unwrap();
                routers.push(rr);
                id_name_map.insert(rr, false);
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
                id_name_map.insert(rc, true);
            }
        }

        for r1 in &routers {
            for r2 in &routers {
                if r1.index() > r2.index() {
                    let mut w: f32 = 1.0;
                    if *id_name_map.get(r1).unwrap() && *id_name_map.get(r2).unwrap() {
                        w = rand::thread_rng().gen_range(5, 10) as f32;
                    }
                    cf.add(IgpLinkWeight { source: *r1, target: *r2, weight: w }).unwrap();
                    cf.add(IgpLinkWeight { target: *r1, source: *r2, weight: w }).unwrap();
                }
            }
        }

        // external routers
        for e in 1..variant + 1 {
            let egress = net.get_router_id(format!("c_{}_1", e)).unwrap();
            let ext = net.get_router_id(format!("EXT_{}", e)).unwrap();
            cf.add(IgpLinkWeight { source: egress, target: ext, weight: 1.0 }).unwrap();
            cf.add(IgpLinkWeight { target: egress, source: ext, weight: 1.0 }).unwrap();
            cf.add(BgpSession { source: egress, target: ext, session_type: EBgp }).unwrap();
        }

        cf
    }

    // fn get_policy(net: &Network, _variant: usize) -> HardPolicy {
    //     HardPolicy::reachability(net.get_routers().iter(), net.get_known_prefixes().iter())
    // }
    fn get_policy(net: &Network, _variant: usize) -> HardPolicy {
        let routers = net.get_routers();
        let egress_test = net.get_router_id("c_1_1").unwrap();
        let final_prop_vars = routers
            .iter()
            .map(|r| Reachable(*r, Prefix(0), Some(PathCondition::Node(egress_test))))
            .collect::<Vec<_>>();

        HardPolicy::globally(final_prop_vars)
    }
}
