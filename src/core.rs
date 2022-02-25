use crate::*;

#[near_bindgen]
impl Contract {

    #[payable]
    pub fn create_project(&mut self, project_id: ProjectId, mut metadata: ProjectMetadata, project_owner_id: AccountId) {
        let before_storage_usage = env::storage_usage();

        let project = Project {
            owner_id: project_owner_id
        };

        assert!(
            self.projects_by_id.insert(&project_id, &project).is_none(),
            "Project already exsits"
        );

        metadata.started_at = Some(env::block_timestamp());

        self.project_metadata_by_id.insert(&project_id, &metadata);

        // set project per owner
        self.internal_add_project_to_owner(&project_id, &project.owner_id);

        let project_create_log: EventLog = EventLog {
            version: "1.0.0".to_string(),
            event: EventLogVariant::ProjectCreate(vec![ ProjectCreateLog {
                owner_id: project.owner_id.to_string(),
                project_ids: vec![project_id.to_string()],
                memo: None
            } ])
        };
        env::log(&project_create_log.to_string().as_bytes());

        let after_storage_usage = env::storage_usage();
        // Refund near
        refund_deposit(after_storage_usage - before_storage_usage);
    }

    #[payable]
    pub fn donate_project(&mut self, project_id: ProjectId) {
        let amount = env::attached_deposit();
        // let amount = deposit.clone() as u128;
        // let before_storage_usage = env::storage_usage();

        let supporter_id = env::predecessor_account_id();

        let project = self.projects_by_id.get(&project_id);
        // get funded captial of project
        if let Some(project) = project {
            let mut metadata = self.project_metadata_by_id.get(&project_id).unwrap();

            let project_owner_id = project.owner_id;
            let mut funded = u128::from(metadata.funded.unwrap_or(U128(0)));
            let target = u128::from(metadata.target);
            let minimum_pledge = u128::from(metadata.minimum_pledge);
            
            // make sure funded capital < target
            assert!(funded < target, "Project's target already reached!");

            // make sure amount must be >= minimum_pledge
            assert!(amount >= minimum_pledge, "Donation must greater than or equal to {}", minimum_pledge);

            funded = funded + amount;
            metadata.funded = Some(U128(funded));
            self.project_metadata_by_id.insert(&project_id, &metadata);

            if funded >= target {
                // transfer 95% of donation (funded) from contract account to project owner account
                let transfer_capital = 95 * funded / 100; 
                let real_transfer = transfer_capital as u128;
                Promise::new(project_owner_id).transfer(Balance::from(real_transfer));
            }

            // set supporter per project
            self.internal_add_supporter_to_project(&supporter_id, &project_id);

        } else {
            panic!("Project doesn't exist!");
        }

        // let after_storage_usage = env::storage_usage();
        // Refund near
        // refund_deposit(after_storage_usage - before_storage_usage);
    }

    pub fn vote_project(&mut self, project_id: ProjectId) {
        let voter_id = env::predecessor_account_id();
        let project = self.projects_by_id.get(&project_id);

        if let Some(project) = project {
            self.internal_add_voter_to_project(&voter_id, &project_id);
        } else {
            panic!("Project doesn't exist!");
        }
    }
}