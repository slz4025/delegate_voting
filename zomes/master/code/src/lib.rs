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

extern crate common;
use common::{
    struct_library::*,
    function_library::*,
};

// create decision
fn create_decision(message : String) -> serde_json::Value {
    let decision = HashString::new();
    let desc = hdk::hash_entry("desc", json!({
        "message" : message;
    }))?;

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
    return json!({"decision": decision});
}

hdk::define_zome! {
    entries: [
        entry!(
            name: "master",
            description: "what the master can do",
            sharing: Sharing::Public,
            native_type: Description,
         
            validation_package: || {
                hdk::ValidationPackageDefinition::ChainFull
            },
         
            validation: |description: Description, _ctx: hdk::ValidationData| {
                (description.content.len() < 280)
                    .ok_or_else(|| String::from("Content too long"))
            }
        ),
        // change
        entry!(
            name: "master",
            description: "what the master can do",
            sharing: Sharing::Public,
            native_type: Decision,
         
            validation_package: || {
                hdk::ValidationPackageDefinition::ChainFull
            },
         
            validation: |description: Description, _ctx: hdk::ValidationData| {
                (description.content.len() < 280)
                    .ok_or_else(|| String::from("Content too long"))
            }
        )
    ]

    genesis: || {
        Ok(())
    }
 
    // return serde::Value
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
