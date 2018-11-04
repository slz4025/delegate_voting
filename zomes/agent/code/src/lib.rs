// move Description to central
// fix genesis error

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive; 
#[macro_use]
extern crate serde_json; 

extern crate difcommon;

use hdk::{
    error::ZomeApiError,
    holochain_core_types::hash::HashString,
    holochain_dna::zome::entry_types::Sharing,
};
use std::{
    collections::HashSet,
    iter::FromIterator,
};
use difcommon::{
    func_lib,
    struct_lib,
};

// create delegation between to-agent, from-agent, decision
fn push_decision(decision : &HashString, agent : &HashString) ->
serde_json::Value {
    let delegation = HashString::new(); // make new hash
    let link_send_del = hdk::link_entries(
        get_self_hash(),
        &delegation,
        "send->del"
    )?;
    let link_del_send = hdk::link_entries(
        get_self_hash(),
        &delegation,
        "del->send"
    )?;
    let link_recv_del = hdk::link_entries(
        agent,
        &delegation,
        "recv->del"
    )?;
    let link_del_recv = hdk::link_entries(
        &delegation,
        agent,
        "del->recv"
    )?;
    let link_del_dec = hdk::link_entries(
        decision,
        &delegation,
        "del->dec"
    )?;
    let link_dec_del = hdk::link_entries(
        &delegation,
        decision,
        "dec->del"
    )?;
    return json!({ "success" : true }); 
}

// get vector of agents
fn get_agents() ->  serde_json::Value {
    let agent_list = get(hdk::DNA_HASH, "has agent");
    return json!(agent_list); 
}

// get vector of decisions
fn get_decisions() ->  serde_json::Value {
    let decision_list = get(hdk::DNA_HASH, "has decision");
    return json!(decision_list); 
}

// get agent you gave vote to on decision 
fn get_recv_agent(decision : &HashString) -> serde_json::Value {
    // list of pairs of a HashString target and String tag (backwards)
    let specific_queries : Vec<(HashString, &str)> = Vec::new();
    specific_queries.push((decision, "del->dec"));
    // query your delegations that are also related to decision
    let delegation = singleton::<HashString>(boost_get(get_self_hash(), "send->del",
        specific_queries));
    let recv = singleton::<HashString>(get(delegation, "del->recv")); 
    return json!(recv);
}

// get agents that gave votes to you on decision
fn get_send_agents(decision : &HashString) -> serde_json::Value {
    // list of pairs of a HashString target and &str tag (backwards)
    let mut specific_queries : Vec<(HashString, &str)> = Vec::new();
    specific_queries.push((decision, "del->dec"));
    // query your delegations that are also related to decision
    let delegation = singleton::<HashString>(boost_get(get_self_hash(), "recv->del",
        specific_queries));
    let send = get(delegation, "del->send"); 
    return json!(send);
}

// get number of agents that gave votes to you on decision
fn get_vote_weight(decision : &HashString) -> serde_json::Value {
    let vec = get_recv_agent(decision);
    return json!({ "weight" : vec.len() });
}

// get decision info
fn get_dec_desc(decision : &HashString) -> serde_json::Value {
    let str_addr = get(decision, "has description");
    let desc = get_desc::<Description>(str_addr);
    return json!(desc);
}

// client calls this at beginning
fn register_agent() -> serde_json::Value {
    let link_result = verify_link(hdk::link_entries(
        hdk::DNA_HASH, 
        &HashString::from(hdk::AGENT_ADDRESS.to_string()),
        "has agent"
    ));
    return json!({ "sucess" : link_result });
}

hdk::define_zome! {
    entries: [
        entry!(
            name: "agent",
            description: "what an agent can do",
            sharing: Sharing::Public,
            native_type: bool,
         
            validation_package: || {
                hdk::ValidationPackageDefinition::ChainFull
            },
         
            validation: |agent: bool, _ctx: hdk::ValidationData| {
                (agent.content.len() < 280)
                    .ok_or_else(|| String::from("Content too long"))
            }
        )
    ]

    genesis: || {
        Ok(())
    }
 
    // makes a public version of the private functions
    functions: {
        main (Public) {
            agent_push_decision: {
                inputs: |decision : HashString, agent : HashString|,
                outputs: |result : serde_json::Value|,
                handler: push_decision
            }            

            agent_get_agents: {
                inputs: | |,
                outputs: |agents : serde_json::Value|,
                handler: get_agents 
            }

            agent_get_decisions: {
                inputs: | |,
                outputs: |decisions : serde_json::Value|,
                handler: get_decisions 
            }

            agent_get_recv_agent: {
                inputs: |decision : HashString|,
                outputs: |recv_agent : serde_json::Value|,
                handler: get_recv_agent
            }

            agent_get_send_agents: {
                inputs: |decision : HashString|,
                outputs: |send_agents : serde_json::Value|,
                handler: get_send_agents
            }

            agent_get_vote_weight: {
                inputs: |decision : HashString|,
                outputs: |weight : serde_json::Value|,
                handler: get_vote_weight
            }

            agent_get_dec_desc: {
                inputs: |decision : HashString|,
                outputs: |dec_desc : serde_json::Value|,
                handler: get_dec_desc
            }
        }
    }
}
