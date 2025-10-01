use crate::application::dto::*;
use crate::domain::entities::Department;
use crate::domain::errors::DomainError;
use crate::domain::repositories::DepartmentRepository;
use crate::domain::value_objects::*;

pub struct CreateDepartmentUseCase<'a> {
    department_repository: &'a dyn DepartmentRepository,
}

impl<'a> CreateDepartmentUseCase<'a> {
    pub fn new(department_repository: &'a dyn DepartmentRepository) -> Self {
        CreateDepartmentUseCase {
            department_repository,
        }
    }

    pub async fn execute(
        &self,
        request: CreateDepartmentRequest,
    ) -> Result<DepartmentResponse, DomainError> {
        let unit_id = OrgUnitId(request.unit_id);
        let name =
            DepartmentName::new(request.name).map_err(|e| DomainError::ValidationError(e))?;

        let department = Department::new(unit_id, name);
        let saved_department = self.department_repository.save(&department).await?;
        Ok(saved_department.into())
    }
}

pub struct UpdateDepartmentUseCase<'a> {
    department_repository: &'a dyn DepartmentRepository,
}

impl<'a> UpdateDepartmentUseCase<'a> {
    pub fn new(department_repository: &'a dyn DepartmentRepository) -> Self {
        UpdateDepartmentUseCase {
            department_repository,
        }
    }

    pub async fn execute(
        &self,
        request: UpdateDepartmentRequest,
    ) -> Result<DepartmentResponse, DomainError> {
        let department_id = DepartmentId::from_string(&request.id)
            .map_err(|e| DomainError::ValidationError(format!("Invalid department ID: {}", e)))?;

        let mut department = self
            .department_repository
            .find_by_id(&department_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("Department with ID {} not found", request.id))
            })?;

        if let Some(name) = request.name {
            let department_name =
                DepartmentName::new(name).map_err(|e| DomainError::ValidationError(e))?;
            department.update_name(department_name);
        }

        if let Some(unit_id) = request.unit_id {
            department.update_unit(OrgUnitId(unit_id));
        }

        let updated_department = self.department_repository.update(&department).await?;
        Ok(updated_department.into())
    }
}

pub struct DeleteDepartmentUseCase<'a> {
    department_repository: &'a dyn DepartmentRepository,
}

impl<'a> DeleteDepartmentUseCase<'a> {
    pub fn new(department_repository: &'a dyn DepartmentRepository) -> Self {
        DeleteDepartmentUseCase {
            department_repository,
        }
    }

    pub async fn execute(&self, id: &str) -> Result<(), DomainError> {
        let department_id = DepartmentId::from_string(id)
            .map_err(|e| DomainError::ValidationError(format!("Invalid department ID: {}", e)))?;

        // Check if department exists
        self.department_repository
            .find_by_id(&department_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Department with ID {} not found", id)))?;

        self.department_repository.delete(&department_id).await?;
        Ok(())
    }
}

pub struct GetDepartmentsUseCase<'a> {
    department_repository: &'a dyn DepartmentRepository,
}

impl<'a> GetDepartmentsUseCase<'a> {
    pub fn new(department_repository: &'a dyn DepartmentRepository) -> Self {
        GetDepartmentsUseCase {
            department_repository,
        }
    }

    pub async fn execute(
        &self,
        request: DepartmentSearchRequest,
    ) -> Result<DepartmentSearchResponse, DomainError> {
        let unit_id = request.unit_id.map(OrgUnitId);

        let criteria = crate::domain::repositories::DepartmentSearchCriteria {
            name: request.search_term,
            unit_id,
            limit: request.limit,
            offset: request.offset,
        };

        let result = self.department_repository.find_all(&criteria).await?;
        let items = result
            .items
            .into_iter()
            .map(|department| department.into())
            .collect();

        Ok(DepartmentSearchResponse {
            items,
            total: result.total,
        })
    }

    pub async fn execute_by_id(
        &self,
        id: &DepartmentId,
    ) -> Result<DepartmentResponse, DomainError> {
        let department = self
            .department_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("Department with ID {} not found", id)))?;
        Ok(department.into())
    }

    pub async fn execute_by_unit(
        &self,
        unit_id: &OrgUnitId,
    ) -> Result<DepartmentSearchResponse, DomainError> {
        let departments = self.department_repository.find_by_unit(unit_id).await?;
        let items = departments
            .into_iter()
            .map(|department| department.into())
            .collect::<Vec<_>>();
        let total = items.len() as i64;

        Ok(DepartmentSearchResponse { items, total })
    }
}

pub struct GetDepartmentStatisticsUseCase<'a> {
    department_repository: &'a dyn DepartmentRepository,
}

impl<'a> GetDepartmentStatisticsUseCase<'a> {
    pub fn new(department_repository: &'a dyn DepartmentRepository) -> Self {
        GetDepartmentStatisticsUseCase {
            department_repository,
        }
    }

    pub async fn execute(&self) -> Result<DepartmentStatisticsResponse, DomainError> {
        let stats = self.department_repository.get_statistics().await?;
        let departments_by_unit = stats
            .departments_by_unit
            .into_iter()
            .map(|(k, v)| (k.0, v))
            .collect();

        Ok(DepartmentStatisticsResponse {
            total_departments: stats.total_departments,
            departments_by_unit,
        })
    }
}
