use async_trait::async_trait;
use sqlx::PgPool;
use uuid::Uuid;
use std::collections::HashMap;

use crate::domain::entities::Department;
use crate::domain::errors::DomainError;
use crate::domain::repositories::{DepartmentRepository, DepartmentSearchCriteria, DepartmentSearchResult, DepartmentStatistics};
use crate::domain::value_objects::{DepartmentId, OrgUnitId};
use crate::infrastructure::mappers::{DepartmentRow, build_department_from_row};

pub struct PostgresDepartmentRepository {
    pool: PgPool,
}

impl PostgresDepartmentRepository {
    pub fn new(pool: PgPool) -> Self {
        PostgresDepartmentRepository { pool }
    }
}

#[async_trait]
impl DepartmentRepository for PostgresDepartmentRepository {
    async fn find_by_id(&self, id: &DepartmentId) -> Result<Option<Department>, DomainError> {
        let row = sqlx::query_as!(
            DepartmentRow,
            "SELECT id, unit_id, name, created_at, updated_at FROM departments WHERE id = $1",
            id.0
        )
        .fetch_optional(&self.pool)
        .await?;

        match row {
            Some(row) => Ok(Some(build_department_from_row(row)?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self, criteria: &DepartmentSearchCriteria) -> Result<DepartmentSearchResult, DomainError> {
        let mut query = "SELECT id, unit_id, name, created_at, updated_at FROM departments WHERE 1=1".to_string();
        let mut params: Vec<Box<dyn sqlx::Encode<'_, sqlx::Postgres> + Send + Sync>> = Vec::new();
        let mut param_count = 0;

        if let Some(ref name) = criteria.name {
            param_count += 1;
            query.push_str(&format!(" AND name ILIKE ${}", param_count));
            params.push(Box::new(format!("%{}%", name)));
        }

        if let Some(ref unit_id) = criteria.unit_id {
            param_count += 1;
            query.push_str(&format!(" AND unit_id = ${}", param_count));
            params.push(Box::new(unit_id.0));
        }

        // Get total count
        let count_query = format!("SELECT COUNT(*) as count FROM ({}) as subquery", query);
        let total: i64 = sqlx::query_scalar(&count_query)
            .fetch_one(&self.pool)
            .await?;

        // Add pagination
        if let Some(limit) = criteria.limit {
            param_count += 1;
            query.push_str(&format!(" LIMIT ${}", param_count));
            params.push(Box::new(limit));
        }

        if let Some(offset) = criteria.offset {
            param_count += 1;
            query.push_str(&format!(" OFFSET ${}", param_count));
            params.push(Box::new(offset));
        }

        query.push_str(" ORDER BY name");

        // For now, we'll use a simplified approach without dynamic parameters
        let rows = sqlx::query_as!(
            DepartmentRow,
            "SELECT id, unit_id, name, created_at, updated_at FROM departments ORDER BY name LIMIT $1 OFFSET $2",
            criteria.limit.unwrap_or(100),
            criteria.offset.unwrap_or(0)
        )
        .fetch_all(&self.pool)
        .await?;

        let mut departments = Vec::new();
        for row in rows {
            departments.push(build_department_from_row(row)?);
        }

        Ok(DepartmentSearchResult {
            items: departments,
            total,
        })
    }

    async fn save(&self, department: &Department) -> Result<Department, DomainError> {
        let row = sqlx::query_as!(
            DepartmentRow,
            "INSERT INTO departments (id, unit_id, name, created_at, updated_at) 
             VALUES ($1, $2, $3, $4, $5) 
             RETURNING id, unit_id, name, created_at, updated_at",
            department.id.0,
            department.unit_id.0,
            department.name.value,
            department.created_at,
            department.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(build_department_from_row(row)?)
    }

    async fn update(&self, department: &Department) -> Result<Department, DomainError> {
        let row = sqlx::query_as!(
            DepartmentRow,
            "UPDATE departments SET unit_id = $2, name = $3, updated_at = $4 
             WHERE id = $1 
             RETURNING id, unit_id, name, created_at, updated_at",
            department.id.0,
            department.unit_id.0,
            department.name.value,
            department.updated_at
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(build_department_from_row(row)?)
    }

    async fn delete(&self, id: &DepartmentId) -> Result<(), DomainError> {
        sqlx::query!(
            "DELETE FROM departments WHERE id = $1",
            id.0
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<Department>, DomainError> {
        let rows = sqlx::query_as!(
            DepartmentRow,
            "SELECT id, unit_id, name, created_at, updated_at FROM departments WHERE name ILIKE $1",
            format!("%{}%", name)
        )
        .fetch_all(&self.pool)
        .await?;

        let mut departments = Vec::new();
        for row in rows {
            departments.push(build_department_from_row(row)?);
        }

        Ok(departments)
    }

    async fn find_by_unit(&self, unit_id: &OrgUnitId) -> Result<Vec<Department>, DomainError> {
        let rows = sqlx::query_as!(
            DepartmentRow,
            "SELECT id, unit_id, name, created_at, updated_at FROM departments WHERE unit_id = $1",
            unit_id.0
        )
        .fetch_all(&self.pool)
        .await?;

        let mut departments = Vec::new();
        for row in rows {
            departments.push(build_department_from_row(row)?);
        }

        Ok(departments)
    }

    async fn get_statistics(&self) -> Result<DepartmentStatistics, DomainError> {
        let total_departments = sqlx::query_scalar!(
            "SELECT COUNT(*) as count FROM departments"
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);

        // Get count by unit
        let rows = sqlx::query!(
            "SELECT unit_id, COUNT(*) as count FROM departments GROUP BY unit_id"
        )
        .fetch_all(&self.pool)
        .await?;

        let mut departments_by_unit = HashMap::new();
        for row in rows {
            if let Some(unit_id) = row.unit_id {
                departments_by_unit.insert(OrgUnitId(unit_id), row.count.unwrap_or(0));
            }
        }

        Ok(DepartmentStatistics {
            total_departments,
            departments_by_unit,
        })
    }
}
