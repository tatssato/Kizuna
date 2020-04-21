#![feature(proc_macro_hygiene)]
#![allow(dead_code)]
#![allow(unused_imports)]

use hdk_proc_macros::zome;
use serde_derive::{Deserialize, Serialize};
use hdk::{
    entry_definition::ValidatingEntryType,
    error::ZomeApiResult,
    holochain_persistence_api::cas::content::Address
};
use crate::profile::{
    PrivateProfile,
    PrivateProfileEntry,
    PublicProfile,
    PublicProfileEntry,
    HashedEmail
};
pub mod profile;

// MAIN FILE FOR THE PROFILE ZOME
// contains calls to entry definitions and functions

// Crate              Modules
// profile __________ mod
//            |______ handlers
//            |______ validation

#[zome]
mod profile_zome {

    #[init]
    fn init() {
        Ok(())
    }

    #[validate_agent]
    pub fn validate_agent(validation_data: EntryValidationData<AgentId>) {
        // this is where you can actually have some validations for agents who want to join this network.
        // Since this is a public DHT wehere anyone can join, we might not have much of validation here. Let's see.
        Ok(())
    }

    #[entry_def]
    fn anchor_def() -> ValidatingEntryType {
        holochain_anchors::anchor_definition()
    }

    #[entry_def]
    fn private_profile_def() -> ValidatingEntryType {
        profile::private_profile_definition()
    }

    #[entry_def]
    fn public_profile_def() -> ValidatingEntryType {
        profile::public_profile_definition()
    }

    #[entry_def]
    fn hashed_email_def() -> ValidatingEntryType {
        profile::hashed_email_definition()
    }
    
    #[zome_fn("hc_public")]
    fn create_private_profile(input: PrivateProfileEntry) -> ZomeApiResult<PrivateProfile> {
        profile::handlers::create_private_profile(input)
    }
    
    #[zome_fn("hc_public")]
    fn create_public_profile(input: PublicProfileEntry) -> ZomeApiResult<PublicProfile> {
        profile::handlers::create_public_profile(input)
    }
    
    #[zome_fn("hc_public")]
    fn create_hashed_email(input: PrivateProfileEntry) -> ZomeApiResult<HashedEmail> {
        profile::handlers::create_hashed_email(input)
    }

    #[zome_fn("hc_public")]
    fn get_public_profile(id: Address) -> ZomeApiResult<PublicProfile> {
        profile::handlers::get_public_profile(id)
    }

    #[zome_fn("hc_public")]
    fn get_private_profile(id: Address) -> ZomeApiResult<PrivateProfile> {
        profile::handlers::get_private_profile(id)
    }

    #[zome_fn("hc_public")]
    fn list_public_profiles(initial: String) -> ZomeApiResult<Vec<PublicProfile>> {
        profile::handlers::list_public_profiles(initial)
    }
    
    #[zome_fn("hc_public")]
    fn search_username(username: String) -> ZomeApiResult<Option<PublicProfile>> {
        profile::handlers::search_username(username)
    }

    #[zome_fn("hc_public")]
    fn register(
        public_input: PublicProfileEntry, 
        private_input: PrivateProfileEntry
        ) -> ZomeApiResult<PublicProfile> {
            // profile::handlers::compare_hashes(private_input.clone())?;
            profile::handlers::create_private_profile(private_input.clone())?;
            profile::handlers::create_hashed_email(private_input)?;
            profile::handlers::create_public_profile(public_input)
    }

    #[zome_fn("hc_public")]
    fn get_linked_profile(username: String) -> ZomeApiResult<Option<PrivateProfile>> {
        profile::handlers::get_linked_profile(username)
    }

    #[zome_fn("hc_public")]
    pub fn compare_hashes (input: PrivateProfileEntry) -> ZomeApiResult<Option<HashedEmail>>{
        profile::handlers::compare_hashes(input)
    }

    #[zome_fn("hc_public")]
    pub fn get_hashed_emails(email: String) -> ZomeApiResult<Vec<HashedEmail>> {
        profile::handlers::get_hashed_emails(email)
    }
}
