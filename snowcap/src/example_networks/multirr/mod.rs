use super::ExampleNetwork;
// use crate::hard_policies::Condition::*;
use crate::hard_policies::HardPolicy;
// use crate::hard_policies::*;
use crate::netsim::config::{Config, ConfigExpr::*};
use crate::netsim::{AsId, BgpSessionType::*, Network, Prefix};

/// # MultiRR
///
/// Zibin Chen
/// Implementing MultiRR case:
/// @param: initial-variant: number of core routers
/// number of clients per core is hard coded as 3
///
pub struct MultiRR {}

impl ExampleNetwork for MultiRR {
    fn net(initial_variant: usize) -> Network {
        let mut net = Network::new();
        let mut cf = Config::new();

        for core in 1..initial_variant + 1 {
            let core_router = net.add_router(format!("core_{}", core));
            for client in 1..4 {
                let client_router = net.add_router(format!("client_{}_{}", core, client));
                let igp_w = core + client + 100;
                // connect core router with client router
                net.add_link(core_router, client_router);
                cf.add(IgpLinkWeight {
                    source: core_router,
                    target: client_router,
                    weight: igp_w as f32,
                })
                .unwrap();
                cf.add(IgpLinkWeight {
                    target: core_router,
                    source: client_router,
                    weight: igp_w as f32,
                })
                .unwrap();
                cf.add(BgpSession {
                    source: core_router,
                    target: client_router,
                    session_type: IBgpClient,
                })
                .unwrap();
                // create external router, announcing prefix to 10 + Core
                let external_router = net.add_external_router(
                    format!("EXT_{}_{}", core, client),
                    AsId(10 + core as u32),
                );
                net.add_link(client_router, external_router);
                cf.add(IgpLinkWeight {
                    source: client_router,
                    target: external_router,
                    weight: 1.0,
                })
                .unwrap();
                cf.add(IgpLinkWeight {
                    source: external_router,
                    target: client_router,
                    weight: 1.0,
                })
                .unwrap();
                cf.add(BgpSession {
                    source: client_router,
                    target: external_router,
                    session_type: EBgp,
                })
                .unwrap();
                net.advertise_external_route(
                    external_router,
                    Prefix(10 + core as u32),
                    vec![AsId(10 + core as u32)],
                    None,
                    None,
                )
                .unwrap();
            }
        }

        // config fully meshed iBGP
        for core1 in 1..initial_variant + 1 {
            let core1_router = net.get_router_id(format!("core_{}", core1)).unwrap();
            for core2 in 1..core1 {
                let core2_router = net.get_router_id(format!("core_{}", core2)).unwrap();
                let igp_w = core1 + core2;
                net.add_link(core1_router, core2_router);
                cf.add(IgpLinkWeight {
                    source: core1_router,
                    target: core2_router,
                    weight: igp_w as f32,
                })
                .unwrap();
                cf.add(IgpLinkWeight {
                    source: core2_router,
                    target: core1_router,
                    weight: igp_w as f32,
                })
                .unwrap();
                cf.add(BgpSession {
                    source: core1_router,
                    target: core2_router,
                    session_type: IBgpPeer,
                })
                .unwrap();
            }
        }

        net.set_config(&cf).unwrap();
        net
    }

    fn initial_config(_net: &Network, _variant: usize) -> Config {
        todo!();
    }

    fn final_config(net: &Network, variant: usize) -> Config {
        let mut cf = Config::new();

        for core in 1..variant + 1 {
            let core_router = net.get_router_id(format!("core_{}", core)).unwrap();
            for client in 1..4 {
                let client_router =
                    net.get_router_id(format!("client_{}_{}", core, client)).unwrap();
                let igp_w = core + client + 100;
                // connect core router with client router
                cf.add(IgpLinkWeight {
                    source: core_router,
                    target: client_router,
                    weight: igp_w as f32,
                })
                .unwrap();
                cf.add(IgpLinkWeight {
                    target: core_router,
                    source: client_router,
                    weight: igp_w as f32,
                })
                .unwrap();
                cf.add(BgpSession {
                    source: core_router,
                    target: client_router,
                    session_type: IBgpClient,
                })
                .unwrap();
                // create external router, announcing prefix to 10 + Core
                let external_router =
                    net.get_router_id(format!("EXT_{}_{}", core, client)).unwrap();
                cf.add(IgpLinkWeight {
                    source: client_router,
                    target: external_router,
                    weight: 1.0,
                })
                .unwrap();
                cf.add(IgpLinkWeight {
                    source: external_router,
                    target: client_router,
                    weight: 1.0,
                })
                .unwrap();
                cf.add(BgpSession {
                    source: client_router,
                    target: external_router,
                    session_type: EBgp,
                })
                .unwrap();
            }
        }

        // let new_core = net.get_router_id("core_1").unwrap();
        for core1 in 1..variant + 1 {
            let core1_router = net.get_router_id(format!("core_{}", core1)).unwrap();
            for core2 in 1..core1 {
                let core2_router = net.get_router_id(format!("core_{}", core2)).unwrap();
                let igp_w = core1 + core2;
                cf.add(IgpLinkWeight {
                    source: core1_router,
                    target: core2_router,
                    weight: igp_w as f32,
                })
                .unwrap();
                cf.add(IgpLinkWeight {
                    source: core2_router,
                    target: core1_router,
                    weight: igp_w as f32,
                })
                .unwrap();

                if core1 == 1 {
                    cf.add(BgpSession {
                        source: core1_router,
                        target: core2_router,
                        session_type: IBgpClient,
                    })
                    .unwrap();
                } else if core2 == 1 {
                    cf.add(BgpSession {
                        source: core2_router,
                        target: core1_router,
                        session_type: IBgpClient,
                    })
                    .unwrap();
                }
            }
        }

        cf
    }

    fn get_policy(net: &Network, _variant: usize) -> HardPolicy {
        HardPolicy::reachability(net.get_routers().iter(), net.get_known_prefixes().iter())
    }
    // fn get_policy(net: &Network, _variant: usize) -> HardPolicy {
    //     let routers = net.get_routers();
    //     let egress_test = net.get_router_id("c_1_1").unwrap();
    //     let final_prop_vars = routers
    //         .iter()
    //         .map(|r| Reachable(*r, Prefix(0), Some(PathCondition::Node(egress_test))))
    //         .collect::<Vec<_>>();

    //     HardPolicy::globally(final_prop_vars)
    // }
}
