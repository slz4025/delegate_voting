// worry about syntax, semicolons

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
fn hashset(data: &[T]) -> HashSet<T> {
    HashSet::from_iter(data.iter().cloned());
}

// convert T-set to T-vector
fn vector(data: HashSet<T>) -> Vec<T> {
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
pub fn get(input : HashString, query : String) -> Vec<HashString> {
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
pub fn boost_get(input : HashString, query : String,
    specific_queries : Vec<(HashString, String)>) -> Vec<HashString> {
    let res_list = get(input, query);
    let res_set = hashset(res_list); 
    for sq in specific_queries {
        let (target, tag) = sq;
        for e in res_list { // iterate over list, not set
           let r = get(e, tag); 
           if (r != e) { res_set.remove(e); } // remove from set
        };
    } 
    let res = vector(res_set);
    return res;
}

// handling of unwraps
// has generic output type
// make sure the outputs make sense
pub fn get_desc(hash : HashString) -> T {
    let res : Result<Option<Task>, ZomeApiError> = hdk::get_entry(hash);
    let desc_ = match result {
        Ok(Some(desc)) => desc,
        Ok(None) =>  {}, // container at hash address is empty 
        Err(_) => {}, // hash was not a valid address 
    };
    return desc_;
}

// vec is a singleton, or should be
pub fn singleton(list : Vec<T>) -> T {
    return list.first();
}

// create delegation between to-agent, from-agent, decision
pub fn push_decision(decision : HashString, agent : HashString) -> () {
    let delegation = HashString::new(); // make new hash
    let link_from_del = hdk::link_entries(
        get_self_hash(),
        delegation,
        "from->del"
    );
    let link_del_from = hdk::link_entries(
        get_self_hash(),
        delegation,
        "del->from"
    );
    let link_to_del = hdk::link_entries(
        agent,
        delegation,
        "to->del"
    );
    let link_del_to = hdk::link_entries(
        delegation,
        agent,
        "del->to"
    );
    let link_del_dec = hdk::link_entries(
        decision,
        delegation,
        "del->dec"
    );
    let link_dec_del = hdk::link_entries(
        delegation,
        decision,
        "dec->del"
    );
}

// get vector of agents
pub fn get_agents() ->  Vec::HashString {
    let agent_list = get(&hdk::DNA_HASH, "has agent");
    return agent_list; 
}

// get vector of decisions
pub fn get_decisions() ->  Vec::HashString {
    let decision_list = get(&hdk::DNA_HASH, "has decision");
    return decision_list; 
}

// get agent you gave vote to on decision 
pub fn get_sent_agent(decision : HashString) ->  HashString {
    // list of pairs of a HashString target and String tag (backwards)
    let specific_queries : Vec<(HashString, String)> = Vec::new();
    specific_queries.push((decision, "del->dec"));
    // query your delegations that are also related to decision
    let delegation = singleton(boost_get(get_self_hash(), "from->del",
        specific_queries));
    let to_ = singleton(get(delegation_, "del->to")); 
    return to_;
}

// get agents that gave votes to you on decision
pub fn get_recv_agent(decision : HashString) ->  HashString {
    // list of pairs of a HashString target and String tag (backwards)
    let specific_queries : Vec<(HashString, String)> = Vec::new();
    specific_queries.push((decision, "del->dec"));
    // query your delegations that are also related to decision
    let delegation = singleton(boost_get(get_self_hash(), "to->del",
        specific_queries));
    let from_ = singleton(get(delegation_, "del->from")); 
    return from_;
}

// get number of agents that gave votes to you on decision
pub fn get_vote_weight(decision : HashString) -> u32 {
    let vec = get_recv_agent(decision);
    return vec.len();
}

// get decision info
pub fn get_dec_desc(decision : HashString) -> String {
    let str = get(decision, "description");
    // maybe more complex
    return str;
}

define_zome! {
    entries: [
        entry!(
            name: "agent",
            description: "tools for the agent to use",
            sharing: Sharing::Public,
            native_type: Agent,
         
            validation_package: || {
                hdk::ValidationPackageDefinition::ChainFull
            },
         
            validation: |agent: Agent, _ctx: hdk::ValidationData| {
                (post.content.len() < 280)
                    .ok_or_else(|| String::from("Content too long"))
            }
        )
    ]

    genesis: || {
        let link_result = hdk::link_entries(
            hdk::DNA_HASH, 
            &HashString::from(hdk::AGENT_ADDRESS.to_string()),
            "has agent"
        );

        Ok(())
    }
 
    functions: {
        main (Public) {
            push_decision: {
                inputs: |decision : HashString, agent : HashString|,
                outputs: ||,
                handler: push_decision
            }            

            get_agents: {
                inputs: ||,
                outputs: ||,
                handler: 
            }

            get_agents: {
                inputs: ||,
                outputs: |Vec::HashString|,
                handler: get_agents 
            }

            get_decisions: {
                inputs: ||,
                outputs: |Vec::HashString|,
                handler: get_decisions 
            }

            get_sent_agent: {
                inputs: |decision : HashString|,
                outputs: |HashString|,
                handler: get_sent_agent
            }

            get_recv_agent: {
                inputs: |decision : HashString|,
                outputs: |HashString|,
                handler: get_recv_agent
            }

            get_vote_weight: {
                inputs: |decision : HashString|,
                outputs: |u32|,
                handler: get_vote_weight
            }

            get_dec_desc: {
                inputs: |decision : HashString|,
                outputs: |String|,
                handler: get_dec_desc
            }
        }
    }
}
