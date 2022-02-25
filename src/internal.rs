use crate::*;

#[near_bindgen]
impl Contract {
    pub(crate) fn internal_add_project_to_owner(&mut self, project_id: &ProjectId, account_id: &AccountId) {

        let mut projects_set = self.projects_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(hash_account_id(account_id).to_vec())
        });

        projects_set.insert(project_id);

        self.projects_per_owner.insert(account_id, &projects_set);
    }

    pub(crate) fn internal_add_supporter_to_project(&mut self, supporter_id: &AccountId, project_id: &ProjectId) {
        let mut supporters_set = self.supporters_per_project.get(project_id).unwrap_or_else(|| {
            UnorderedSet::new(hash_project_id(project_id).to_vec())
        });

        supporters_set.insert(supporter_id);

        self.supporters_per_project.insert(project_id, &supporters_set);
    }

    pub(crate) fn internal_add_voter_to_project(&mut self, voter_id: &AccountId, project_id: &ProjectId) {
        let mut voters_set = self.voters_per_project.get(project_id).unwrap_or_else(|| {
            UnorderedSet::new(hash_project_id(project_id).to_vec())
        });

        voters_set.insert(voter_id);

        self.voters_per_project.insert(project_id, &voters_set);
    }
}