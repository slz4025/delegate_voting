use crate::struct_lib::Description;

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
fn singleton<T>(list : Vec<T>) -> T
where for<'de> T: serde::Deserialize<'de> + Clone,
{
    return list.first().unwrap().clone();
}

// get the hash of the agent
fn get_self_hash() -> HashString {
    return HashString::from(hdk::AGENT_ADDRESS.to_string());
}

// handling of gets
fn get(input : &HashString, query : &str) -> Vec<HashString> {
    match hdk::get_links(input, query) {
        Ok(proc) => {
            let mut list: Vec<HashString> = Vec::with_capacity(proc.addresses.len());
            for e_ in proc.addresses {
                let e = hdk::get_entry(e_);
                match e {
                    Ok(Some(h)) => list.push(h),
                    Ok(None) => {},
                    Err(_) => {},
                }
            }
            list
        },
        Err(hdk_error) => Vec::new(),
    }
}

// handling of gets, with added filters
fn boost_get(input : &HashString, query : &str,
    specific_queries : Vec<(HashString, &str)>) -> Vec<HashString> {
    let res_list = get(input, query);
    let mut res_set = hashset::<HashString>(&res_list); 
    for sq in specific_queries {
        let (target, tag) = sq;
        for e in &res_list { // iterate over list, not set
           let r = singleton(get(&e, tag)); 
           if !r.eq(&e) { res_set.remove(&e); } // remove from set
        };
    } 
    let res = vector::<HashString>(res_set);
    return res;
}

// handling of unwraps
// has generic output type
// make sure the outputs make sense
fn get_desc(address : HashString) -> String {
    let res : Result<Option<Description>, ZomeApiError> = hdk::get_entry(address);
    return match res {
        Ok(Some(desc)) => desc.message,
        Ok(None) =>  String::from(""), // container at hash address is empty 
        Err(_) => String::from(""), // hash was not a valid address 
    }
}

