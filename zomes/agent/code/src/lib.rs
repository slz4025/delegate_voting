// move Description to central
// fix genesis error

#[macro_use]
extern crate hdk;
extern crate serde;
#[macro_use]
extern crate serde_derive; 
#[macro_use]
extern crate serde_json; 

use hdk::{
    error::ZomeApiError,
    holochain_core_types::hash::HashString,
    holochain_dna::zome::entry_types::Sharing,
};
use std::{
    collections::HashSet,
    iter::FromIterator,
};

// convert T-iterator to T-set
fn hashset<T>(data: &[T]) -> HashSet<T> {
    HashSet::from_iter(data.iter().cloned());
}

// convert T-set to T-vector
fn vector<T>(data: HashSet<T>) -> Vec<T> {
    let v = Vec::new();
    for d in data.iter() {
        v.push(d);
    }
    return v;
}

// get the hash of the agent
pub fn get_self_hash() -> HashString {
    return HashString::from(hdk::AGENT_ADDRESS.to_string());
}

// handling of gets
pub fn get(input : &HashString, query : &str) -> Vec<HashString> {
    let raw = hdk::get_links(input, query);
    let proc = match raw{
        Ok(raw_) => raw_,
        Err(hdk_error) => {},
    };
    let mut list: Vec<HashString> = Vec::with_capacity(proc.links.len());
    for e in proc.links {
        match e {
            Some(e_) => list.push(e_),
            None =>  {}
        };
    }
    return list;
}

// handling of gets, with added filters
pub fn boost_get(input : &HashString, query : &str,
    specific_queries : Vec<(HashString, &str)>) -> Vec<HashString> {
    let res_list = get(input, query);
    let res_set = hashset::<HashString>(res_list); 
    for sq in specific_queries {
        let (target, tag) = sq;
        for e in res_list { // iterate over list, not set
           let r = get(e, tag); 
           if r.eq(e) { res_set.remove(e); } // remove from set
        };
    } 
    let res = vector::<HashString>(res_set);
    return res;
}

// handling of unwraps
// has generic output type
// make sure the outputs make sense
pub fn get_desc<T>(address : &HashString) -> T {
    let res : Result<T, ZomeApiError> = hdk::get_entry(address);
    let desc_ = match res {
        Ok(Some(desc)) => desc,
        Ok(None) =>  {}, // container at hash address is empty 
        Err(_) => {}, // hash was not a valid address 
    };
    return desc_;
}

// vec is a singleton, or should be
pub fn singleton<T>(list : Vec<T>) -> T {
    return list.first();
}

// create delegation between to-agent, from-agent, decision
pub fn push_decision(decision : &HashString, agent : &HashString) -> bool {
    let delegation = HashString::new(); // make new hash
    let link_send_del = hdk::link_entries(
        get_self_hash(),
        &delegation,
        "send->del"
    );
    let link_del_send = hdk::link_entries(
        get_self_hash(),
        &delegation,
        "del->send"
    );
    let link_recv_del = hdk::link_entries(
        agent,
        &delegation,
        "recv->del"
    );
    let link_del_recv = hdk::link_entries(
        &delegation,
        agent,
        "del->recv"
    );
    let link_del_dec = hdk::link_entries(
        decision,
        &delegation,
        "del->dec"
    );
    let link_dec_del = hdk::link_entries(
        &delegation,
        decision,
        "dec->del"
    );
    return true;
}

// get vector of agents
pub fn get_agents() ->  Vec<HashString> {
    let agent_list = get(hdk::DNA_HASH, "has agent");
    return agent_list; 
}

// get vector of decisions
pub fn get_decisions() ->  Vec<HashString> {
    let decision_list = get(hdk::DNA_HASH, "has decision");
    return decision_list; 
}

// get agent you gave vote to on decision 
pub fn get_recv_agent(decision : &HashString) ->  HashString {
    // list of pairs of a HashString target and String tag (backwards)
    let specific_queries : Vec<(HashString, &str)> = Vec::new();
    specific_queries.push((decision, "del->dec"));
    // query your delegations that are also related to decision
    let delegation = singleton::<HashString>(boost_get(get_self_hash(), "send->del",
        specific_queries));
    let recv = singleton::<HashString>(get(delegation, "del->recv")); 
    return recv;
}

// get agents that gave votes to you on decision
pub fn get_send_agents(decision : &HashString) -> Vec<HashString> {
    // list of pairs of a HashString target and &str tag (backwards)
    let specific_queries : Vec<(HashString, &str)> = Vec::new();
    specific_queries.push((decision, "del->dec"));
    // query your delegations that are also related to decision
    let delegation = singleton::<HashString>(boost_get(get_self_hash(), "recv->del",
        specific_queries));
    let send = get(delegation, "del->send"); 
    return send;
}

// get number of agents that gave votes to you on decision
pub fn get_vote_weight(decision : &HashString) -> u32 {
    let vec = get_recv_agent(decision);
    return vec.len();
}

// get decision info
pub fn get_dec_desc(decision : &HashString) -> String {
    let str_addr = get(decision, "has description");
    let desc = get_desc::<Description>(str_addr);
    let message = desc.message; 
    return message;
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
        /*
        let link_result = hdk::link_entries(
            hdk::DNA_HASH, 
            &HashString::from(hdk::AGENT_ADDRESS.to_string()),
            "has agent"
        );
        */

        Ok(())
    }
 
    functions: {
        main (Public) {
            agent_push_decision: {
                inputs: |decision : HashString, agent : HashString|,
                outputs: |result : bool|,
                handler: push_decision
            }            

            agent_get_agents: {
                inputs: | |,
                outputs: |agents : Vec<HashString>|,
                handler: get_agents 
            }

            agent_get_decisions: {
                inputs: | |,
                outputs: |decisions : Vec<HashString>|,
                handler: get_decisions 
            }

            agent_get_recv_agent: {
                inputs: |decision : HashString|,
                outputs: |recv_agent : HashString|,
                handler: get_recv_agent
            }

            agent_get_send_agents: {
                inputs: |decision : HashString|,
                outputs: |send_agents : Vec<HashString>|,
                handler: get_send_agents
            }

            agent_get_vote_weight: {
                inputs: |decision : HashString|,
                outputs: |weight : u32|,
                handler: get_vote_weight
            }

            agent_get_dec_desc: {
                inputs: |decision : HashString|,
                outputs: |dec_desc : String|,
                handler: get_dec_desc
            }
        }
    }
}
