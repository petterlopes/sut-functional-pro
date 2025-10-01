use crate::application::dto::*;
use crate::domain::entities::User;
use crate::domain::errors::DomainError;
use crate::domain::repositories::UserRepository;
use crate::domain::value_objects::*;

pub struct CreateUserUseCase<'a> {
    user_repository: &'a dyn UserRepository,
}

impl<'a> CreateUserUseCase<'a> {
    pub fn new(user_repository: &'a dyn UserRepository) -> Self {
        CreateUserUseCase { user_repository }
    }

    pub async fn execute(&self, request: CreateUserRequest) -> Result<UserResponse, DomainError> {
        let username =
            Username::new(request.username).map_err(|e| DomainError::ValidationError(e))?;
        let email = UserEmail::new(request.email).map_err(|e| DomainError::ValidationError(e))?;
        let password =
            Password::new(request.password).map_err(|e| DomainError::ValidationError(e))?;

        let roles = request
            .roles
            .into_iter()
            .map(|role_str| Role::new(role_str))
            .collect::<Result<Vec<Role>, String>>()
            .map_err(|e| DomainError::ValidationError(e))?;

        let user = User::new(username, email, password, roles);
        let saved_user = self.user_repository.save(&user).await?;
        Ok(saved_user.into())
    }
}

pub struct UpdateUserUseCase<'a> {
    user_repository: &'a dyn UserRepository,
}

impl<'a> UpdateUserUseCase<'a> {
    pub fn new(user_repository: &'a dyn UserRepository) -> Self {
        UpdateUserUseCase { user_repository }
    }

    pub async fn execute(&self, request: UpdateUserRequest) -> Result<UserResponse, DomainError> {
        let user_id = UserId::from_string(&request.id)
            .map_err(|e| DomainError::ValidationError(format!("Invalid user ID: {}", e)))?;

        let mut user = self
            .user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("User with ID {} not found", request.id))
            })?;

        if let Some(username) = request.username {
            let username_vo =
                Username::new(username).map_err(|e| DomainError::ValidationError(e))?;
            user.update_username(username_vo);
        }

        if let Some(email) = request.email {
            let email_vo = UserEmail::new(email).map_err(|e| DomainError::ValidationError(e))?;
            user.update_email(email_vo);
        }

        if let Some(password) = request.password {
            let password_vo =
                Password::new(password).map_err(|e| DomainError::ValidationError(e))?;
            user.update_password(password_vo);
        }

        if let Some(roles) = request.roles {
            let role_vos = roles
                .into_iter()
                .map(|role_str| Role::new(role_str))
                .collect::<Result<Vec<Role>, String>>()
                .map_err(|e| DomainError::ValidationError(e))?;
            user.roles = role_vos;
        }

        let updated_user = self.user_repository.update(&user).await?;
        Ok(updated_user.into())
    }
}

pub struct DeleteUserUseCase<'a> {
    user_repository: &'a dyn UserRepository,
}

impl<'a> DeleteUserUseCase<'a> {
    pub fn new(user_repository: &'a dyn UserRepository) -> Self {
        DeleteUserUseCase { user_repository }
    }

    pub async fn execute(&self, id: &str) -> Result<(), DomainError> {
        let user_id = UserId::from_string(id)
            .map_err(|e| DomainError::ValidationError(format!("Invalid user ID: {}", e)))?;

        // Check if user exists
        self.user_repository
            .find_by_id(&user_id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("User with ID {} not found", id)))?;

        self.user_repository.delete(&user_id).await?;
        Ok(())
    }
}

pub struct GetUsersUseCase<'a> {
    user_repository: &'a dyn UserRepository,
}

impl<'a> GetUsersUseCase<'a> {
    pub fn new(user_repository: &'a dyn UserRepository) -> Self {
        GetUsersUseCase { user_repository }
    }

    pub async fn execute(
        &self,
        request: UserSearchRequest,
    ) -> Result<UserSearchResponse, DomainError> {
        let criteria = crate::domain::repositories::UserSearchCriteria {
            username: request.search_term.clone(),
            email: request.search_term,
            role: request.role,
            limit: request.limit,
            offset: request.offset,
        };

        let result = self.user_repository.find_all(&criteria).await?;
        let items = result.items.into_iter().map(|user| user.into()).collect();

        Ok(UserSearchResponse {
            items,
            total: result.total,
        })
    }

    pub async fn execute_by_id(&self, id: &UserId) -> Result<UserResponse, DomainError> {
        let user = self
            .user_repository
            .find_by_id(id)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("User with ID {} not found", id)))?;
        Ok(user.into())
    }

    pub async fn execute_by_username(&self, username: &str) -> Result<UserResponse, DomainError> {
        let user = self
            .user_repository
            .find_by_username(username)
            .await?
            .ok_or_else(|| {
                DomainError::NotFound(format!("User with username {} not found", username))
            })?;
        Ok(user.into())
    }

    pub async fn execute_by_email(&self, email: &str) -> Result<UserResponse, DomainError> {
        let user = self
            .user_repository
            .find_by_email(email)
            .await?
            .ok_or_else(|| DomainError::NotFound(format!("User with email {} not found", email)))?;
        Ok(user.into())
    }

    pub async fn execute_by_role(&self, role: &str) -> Result<UserSearchResponse, DomainError> {
        let users = self.user_repository.find_by_role(role).await?;
        let items = users
            .into_iter()
            .map(|user| user.into())
            .collect::<Vec<_>>();
        let total = items.len() as i64;

        Ok(UserSearchResponse { items, total })
    }
}
