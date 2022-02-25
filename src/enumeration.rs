use crate::*;

#[near_bindgen]
impl Contract {

    // lấy tổng số dự án có trong contract
    pub fn get_number_of_projects(&self) -> U128 {
        U128(self.project_metadata_by_id.len() as u128)
    }

    // lấy ra thông tin một project
    pub fn get_project_info(&self, project_id: ProjectId) -> Option<JsonProject> {
        let project = self.projects_by_id.get(&project_id);

        if let Some(project) = project {
            let metadata = self.project_metadata_by_id.get(&project_id).unwrap();

            Some(JsonProject {
                owner_id: project.owner_id,
                project_id,
                metadata
            })
        } else {
            None
        }
    }

    // lấy danh sách supporters của một project
    pub fn get_project_supporters(&self, project_id: ProjectId) -> Vec<AccountId> {
        let supporters = self.supporters_per_project.get(&project_id);
        if let Some(supporters) = supporters {
            supporters.to_vec()
        } else {
            return vec![];
        }
        
    }

    // lấy danh sách voters của một project
    pub fn get_project_voters(&self, project_id: ProjectId) -> Vec<AccountId> {
        let voters = self.voters_per_project.get(&project_id);
        if let Some(voters) = voters {
            voters.to_vec()
        } else {
            return vec![];
        }
        
    }

    pub fn get_project_funded(&self, project_id: ProjectId) -> Option<u128> {
        let project = self.projects_by_id.get(&project_id);

        if let Some(project) = project {
            let metadata = self.project_metadata_by_id.get(&project_id).unwrap();
            let funded = u128::from(metadata.funded.unwrap());
            Some(funded)
        } else {
            None
        }
    }

    pub fn get_project_target(&self, project_id: ProjectId) -> Option<u128> {
        let project = self.projects_by_id.get(&project_id);

        if let Some(project) = project {
            let metadata = self.project_metadata_by_id.get(&project_id).unwrap();

            Some(u128::from(metadata.target))
        } else {
            None
        }
    }

    // lấy ra danh sách các dự án có paging
    pub fn get_projects_info(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonProject> {
        let project_keys = self.project_metadata_by_id.keys_as_vector();

        let start = u128::from(from_index.unwrap_or(U128(0)));

        project_keys.iter()
        .skip(start as usize)
        .take(limit.unwrap_or(0) as usize)
        .map(|project_id| self.get_project_info(project_id.clone()).unwrap() )
        .collect()
    }

    // Lấy danh sách project của một chủ dự án có account_id tương ứng (có paging)
    pub fn get_projects_by_owner(&self, account_id: AccountId, from_index: Option<U128>, limit: Option<u64>) -> Vec<JsonProject> {
        let projects = self.projects_per_owner.get(&account_id);

        let keys = if let Some(projects) = projects {
            projects
        } else {
            return vec![];
        };

        let start = u128::from(from_index.unwrap_or(U128(0)));

        keys.as_vector()
        .iter()
        .skip(start as usize)
        .take(limit.unwrap_or(0) as usize)
        .map(|project_id| self.get_project_info(project_id.clone()).unwrap() )
        .collect()
    }

    pub fn get_project_started_time(&self, project_id: ProjectId) -> Option<u64> {
        let project = self.projects_by_id.get(&project_id);

        if let Some(project) = project {
            let metadata = self.project_metadata_by_id.get(&project_id).unwrap();

            metadata.started_at
        } else {
            None
        }
    }

    // check if specific project was over deadline
    pub fn is_project_ended(&self, project_id: ProjectId) -> Option<bool> {
        let project = self.projects_by_id.get(&project_id);

        if let Some(project) = project {
            let metadata = self.project_metadata_by_id.get(&project_id).unwrap();
            let ended_time = metadata.ended_at.unwrap();
            let is_ended = ended_time < env::block_timestamp();
            Some(is_ended)
        } else {
            None
        }
    }
}