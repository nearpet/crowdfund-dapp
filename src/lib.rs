use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::{env, near_bindgen, setup_alloc, Balance, CryptoHash, Promise, PanicOnDefault, Timestamp};
use near_sdk::collections::{UnorderedSet, UnorderedMap, LookupMap};
use near_sdk::AccountId;

setup_alloc!();

use crate::utils::*;
pub use crate::metadata::*;
pub use crate::internal::*;
pub use crate::enumeration::*;
pub use crate::core::*;
pub use crate::events::*;

mod utils;
mod metadata;
mod internal;
mod enumeration;
mod core;
mod events;

pub type ProjectId = String;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    // mapping user ID với danh sách dự án của người này
    pub projects_per_owner: LookupMap<AccountId, UnorderedSet<ProjectId>>,
    // mapping project ID với project struct tương ứng (có thể lưu metadata của project struct off-chain)
    pub projects_by_id: LookupMap<ProjectId, Project>,
    // mapping project ID với project metadata
    pub project_metadata_by_id: UnorderedMap<ProjectId, ProjectMetadata>,
    // mapping project ID với danh sách supporters
    pub supporters_per_project: LookupMap<ProjectId, UnorderedSet<AccountId>>,
    // mapping project ID với danh sách voters
    pub voters_per_project: LookupMap<ProjectId, UnorderedSet<AccountId>>
}

#[near_bindgen]
impl Contract {
    #[init]

    pub fn new(owner_id: AccountId) -> Self {
        Self {
            owner_id,
            projects_per_owner: LookupMap::new(b"a".to_vec()),
            projects_by_id: LookupMap::new(b"b".to_vec()),
            project_metadata_by_id: UnorderedMap::new(b"c".to_vec()),
            supporters_per_project: LookupMap::new(b"d".to_vec()),
            voters_per_project: LookupMap::new(b"e".to_vec())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use near_sdk::test_utils::{VMContextBuilder, accounts};
    use near_sdk::{testing_env};
    use near_sdk::MockedBlockchain;

    const CREATE_PROJECT_STORAGE_COST: u128 = 60000000000000000000000;
    const DONATE_PROJECT_AMOUNT: u128 = 3000_000_000_000_000_000_000_000;
    const TARGET: u128 = 10_000_000_000_000_000_000_000_000;
    const MINIMUM_PLEDGE: u128 = 1_000_000_000_000_000_000_000_000;
    // const DONATE_PROJECT_DEPOSIT: u128 = 10000000000000000000000000;

    fn get_context(is_view: bool) -> VMContextBuilder {
        let mut builder = VMContextBuilder::new();
        builder.
        current_account_id(accounts(0))
        .signer_account_id(accounts(0))
        .predecessor_account_id(accounts(0))
        .is_view(is_view);

        builder
    }

    fn get_sample_metadata() -> ProjectMetadata {
        ProjectMetadata { 
            title: String::from("PROJECT_TEST"), 
            description: String::from("PROJECT_DESCRIPTION"), 
            target: TARGET,
            minimum_pledge: MINIMUM_PLEDGE,
            started_at: env::block_timestamp(),
            ended_at: env::block_timestamp() + 2000_000_000 as u64,
            funded: Some(0 as u128),
            media: None, 
            media_hash: None, 
            extra: None, 
            reference: None, 
            reference_hash: None
         }
    }

    #[test]
    fn test_create_project() {
        let mut context = get_context(false);
        testing_env!(context.build());
        
        // Init contract
        let mut contract = Contract::new(accounts(0).to_string());

        testing_env!(
            context.storage_usage(env::storage_usage())
            .attached_deposit(CREATE_PROJECT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build()
        );

        let project_id = "VBI_FUND".to_string();
        contract.create_project(project_id.clone(), get_sample_metadata(), accounts(0).to_string());

        let project = contract.get_project_info(project_id.clone()).unwrap();

        assert_eq!(accounts(0).to_string(), project.owner_id);
        assert_eq!(project_id.clone(), project.project_id);
        assert_eq!(project.metadata, get_sample_metadata());
        assert_eq!(contract.get_number_of_projects(), U128(1));
    }

    #[test]
    fn test_get_target() {
        let mut context = get_context(false);
        testing_env!(context.build());
        
        // Init contract
        let mut contract = Contract::new(accounts(0).to_string());

        testing_env!(
            context.storage_usage(env::storage_usage())
            .attached_deposit(CREATE_PROJECT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build()
        );

        let project_id = "VBI_FUND".to_string();
        contract.create_project(project_id.clone(), get_sample_metadata(), accounts(0).to_string());

        assert_eq!(contract.get_project_target(project_id.clone()), Some(10));
    }

    #[test]
    fn test_get_funded() {
        let mut context = get_context(false);
        testing_env!(context.build());
        
        // Init contract
        let mut contract = Contract::new(accounts(0).to_string());

        testing_env!(
            context.storage_usage(env::storage_usage())
            .attached_deposit(CREATE_PROJECT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build()
        );

        let project_id = "VBI_FUND".to_string();
        contract.create_project(project_id.clone(), get_sample_metadata(), accounts(0).to_string());

        assert_eq!(contract.get_project_funded(project_id.clone()), Some(0));
    }

    #[test]
    fn test_donate() {
        let mut context = get_context(false);
        testing_env!(context.build());
        
        // Init contract
        let mut contract = Contract::new(accounts(0).to_string());

        testing_env!(
            context.storage_usage(env::storage_usage())
            .attached_deposit(CREATE_PROJECT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build()
        );

        let project_id = "VBI_FUND".to_string();
        contract.create_project(project_id.clone(), get_sample_metadata(), accounts(0).to_string());

        contract.donate_project(project_id.clone());
        assert_eq!(contract.get_project_voters(project_id.clone()).len(), 1);
    }

    #[test]
    fn test_vote() {
        let mut context = get_context(false);
        testing_env!(context.build());
        
        // Init contract
        let mut contract = Contract::new(accounts(0).to_string());

        testing_env!(
            context.storage_usage(env::storage_usage())
            .attached_deposit(CREATE_PROJECT_STORAGE_COST)
            .predecessor_account_id(accounts(0))
            .build()
        );

        let project_id = "VBI_FUND".to_string();
        contract.create_project(project_id.clone(), get_sample_metadata(), accounts(0).to_string());

        contract.vote_project(project_id.clone());
        assert_eq!(contract.get_project_voters(project_id.clone()).len(), 1);
    }

}
