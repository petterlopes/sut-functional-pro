use crate::domain::entities::OrgUnit;
use crate::domain::repositories::OrgUnitRepository;
use crate::domain::value_objects::*;
use crate::domain::errors::DomainError;
use crate::application::dto::*;

pub struct CreateOrgUnitUseCase<'a> {
    org_unit_repository: &'a dyn OrgUnitRepository,
}

impl<'a> CreateOrgUnitUseCase<'a> {
    pub fn new(org_unit_repository: &'a dyn OrgUnitRepository) -> Self {
        CreateOrgUnitUseCase { org_unit_repository }
    }

    pub async fn execute(&self, request: CreateOrgUnitRequest) -> Result<OrgUnitResponse, DomainError> {
        let name = OrgUnitName::new(request.name)
            .map_err(|e| DomainError::ValidationError(e))?;

        let parent_id = request.parent_id.map(OrgUnitId);

        let org_unit = OrgUnit::new(name, parent_id);
        let saved_org_unit = self.org_unit_repository.save(&org_unit).await?;
        Ok(saved_org_unit.into())
    }
}

pub struct UpdateOrgUnitUseCase<'a> {
    org_unit_repository: &'a dyn OrgUnitRepository,
}

impl<'a> UpdateOrgUnitUseCase<'a> {
    pub fn new(org_unit_repository: &'a dyn OrgUnitRepository) -> Self {
        UpdateOrgUnitUseCase { org_unit_repository }
    }

    pub async fn execute(&self, request: UpdateOrgUnitRequest) -> Result<OrgUnitResponse, DomainError> {
        let org_unit_id = OrgUnitId::from_string(&request.id)
            .map_err(|e| DomainError::ValidationError(format!("Invalid org unit ID: {}", e)))?;

        let mut org_unit = self.org_unit_repository.find_by_id(&org_unit_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("OrgUnit with ID {} not found", request.id)))?;

        if let Some(name) = request.name {
            let org_unit_name = OrgUnitName::new(name)
                .map_err(|e| DomainError::ValidationError(e))?;
            org_unit.update_name(org_unit_name);
        }

        if let Some(parent_id) = request.parent_id {
            org_unit.set_parent(Some(OrgUnitId(parent_id)));
        }

        let updated_org_unit = self.org_unit_repository.update(&org_unit).await?;
        Ok(updated_org_unit.into())
    }
}

pub struct DeleteOrgUnitUseCase<'a> {
    org_unit_repository: &'a dyn OrgUnitRepository,
}

impl<'a> DeleteOrgUnitUseCase<'a> {
    pub fn new(org_unit_repository: &'a dyn OrgUnitRepository) -> Self {
        DeleteOrgUnitUseCase { org_unit_repository }
    }

    pub async fn execute(&self, id: &str) -> Result<(), DomainError> {
        let org_unit_id = OrgUnitId::from_string(id)
            .map_err(|e| DomainError::ValidationError(format!("Invalid org unit ID: {}", e)))?;

        // Check if org unit exists
        self.org_unit_repository.find_by_id(&org_unit_id).await?
            .ok_or_else(|| DomainError::NotFound(format!("OrgUnit with ID {} not found", id)))?;

        // Check if org unit has children
        let children = self.org_unit_repository.find_children(&org_unit_id).await?;
        if !children.is_empty() {
            return Err(DomainError::BusinessRuleViolation(
                "Cannot delete org unit with children".to_string()
            ));
        }

        self.org_unit_repository.delete(&org_unit_id).await?;
        Ok(())
    }
}

pub struct GetOrgUnitsUseCase<'a> {
    org_unit_repository: &'a dyn OrgUnitRepository,
}

impl<'a> GetOrgUnitsUseCase<'a> {
    pub fn new(org_unit_repository: &'a dyn OrgUnitRepository) -> Self {
        GetOrgUnitsUseCase { org_unit_repository }
    }

    pub async fn execute(&self, request: OrgUnitSearchRequest) -> Result<OrgUnitSearchResponse, DomainError> {
        let parent_id = request.parent_id.map(OrgUnitId);

        let criteria = crate::domain::repositories::OrgUnitSearchCriteria {
            name: request.search_term,
            parent_id,
            limit: request.limit,
            offset: request.offset,
        };

        let result = self.org_unit_repository.find_all(&criteria).await?;
        let items = result.items.into_iter().map(|org_unit| org_unit.into()).collect();

        Ok(OrgUnitSearchResponse {
            items,
            total: result.total,
        })
    }

    pub async fn execute_by_id(&self, id: &OrgUnitId) -> Result<OrgUnitResponse, DomainError> {
        let org_unit = self.org_unit_repository.find_by_id(id).await?
            .ok_or_else(|| DomainError::NotFound(format!("OrgUnit with ID {} not found", id)))?;
        Ok(org_unit.into())
    }

    pub async fn execute_hierarchy(&self, id: &OrgUnitId) -> Result<OrgUnitHierarchyResponse, DomainError> {
        let hierarchy = self.org_unit_repository.get_hierarchy(id).await?;
        let mut items = Vec::new();
        let mut children_map = std::collections::HashMap::new();

        for org_unit in hierarchy {
            let response: OrgUnitResponse = org_unit.clone().into();
            items.push(response);

            let child_org_units = self.org_unit_repository.find_children(&org_unit.id).await?;
            let child_responses: Vec<OrgUnitResponse> = child_org_units.into_iter().map(|child| child.into()).collect();
            children_map.insert(org_unit.id.0, child_responses);
        }

        Ok(OrgUnitHierarchyResponse {
            items,
            children: children_map,
        })
    }
}
