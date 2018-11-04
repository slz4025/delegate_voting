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

// descriptions
pub struct Description {
    message : String,
}

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
    let res_set = hashset(res_list); 
    for sq in specific_queries {
        let (target, tag) = sq;
        for e in res_list { // iterate over list, not set
           let r = get(e, tag); 
           if r != e { res_set.remove(e); } // remove from set
        };
    } 
    let res = vector(res_set);
    return res;
}

// handling of unwraps
// has generic output type
// make sure the outputs make sense
pub fn get_desc<T>(hash : &HashString) -> T {
    let res : Result<T, ZomeApiError> = hdk::get_entry(hash);
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

// unpacks address
pub fn verify(maybe_address : ZomeApiResult<HashString>) -> HashString {
    match maybe_address {
        Ok(address) => address,
        Err(hdk_error) => jdk_error.to_json(),
    }
}

// create decision
pub fn create_decision(message : String) -> bool {
    let decision = HashString::new();
    let desc = verify(hdk::hash_entry("desc", json!({
        "message" : message;
    })));

    let link_desc = hdk::link_entries(
        &decision,
        &desc,
        "has description"
    );
    let link_dec = hdk::link_entries(
        hdk::DNA_HASH,
        &decision,
        "has decision"
    );
    return true;
}

hdk::define_zome! {
    entries: [
        entry!(
            name: "master",
            description: "what the master can do",
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
 
    functions: {
        main (Public) {
            master_create_decision: {
                inputs: |message : String|,
                outputs: |result : bool|,
                handler: create_decision
            }            
        }
    }
}
