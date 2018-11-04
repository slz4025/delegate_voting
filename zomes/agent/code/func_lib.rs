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
fn hashset<T>(data: &[T]) -> HashSet<T> 
where T : Eq + std::hash::Hash + Clone,
{
    return HashSet::from_iter(data.iter().cloned());
}

// convert T-set to T-vector
fn vector<T>(data: HashSet<T>) -> Vec<T>
where T : Eq + std::hash::Hash + Clone,
{
    let mut v = Vec::new();
    for d in data.iter().cloned() {
        v.push(d);
    }
    return v;
}

// vec is a singleton, or should be
pub fn singleton<T>(list : Vec<T>) -> T
where for<'de> T: serde::Deserialize<'de> + Clone,
{
    return list.first().unwrap().clone();
}

// get the hash of the agent
fn get_self_hash() -> HashString {
    return HashString::from(hdk::AGENT_ADDRESS.to_string());
}

