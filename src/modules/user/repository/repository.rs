use crate::modules::user::model::{
    ActiveModel as UserActiveModel, Column as UserColumn, Entity as UserEntity, Model as UserModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

pub struct UserRepository;

impl UserRepository {
    pub async fn insert_user(
        db: &DatabaseConnection,
        name: &str,
        email: &str,
        password: &str,
    ) -> Result<UserModel, sea_orm::DbErr> {
        let user = UserActiveModel {
            name: Set(name.to_string()),
            email: Set(email.to_string()),
            password: Set(password.to_string()),
            ..Default::default()
        };

        let user = user.insert(db).await?;
        Ok(user)
    }

    pub async fn fetch_all_users(
        db: &DatabaseConnection,
    ) -> Result<Vec<UserModel>, sea_orm::DbErr> {
        let users = UserEntity::find().all(db).await?;
        Ok(users)
    }

    pub async fn get_user_by_email(
        db: &DatabaseConnection,
        email: &str,
    ) -> Result<UserModel, sea_orm::DbErr> {
        let user = UserEntity::find()
            .filter(UserColumn::Email.eq(email))
            .one(db)
            .await?;

        user.ok_or(sea_orm::DbErr::RecordNotFound(format!(
            "User with email {} not found",
            email
        )))
    }

    pub async fn get_user_by_id(
        db: &DatabaseConnection,
        id: Uuid,
    ) -> Result<UserModel, sea_orm::DbErr> {
        let user = UserEntity::find_by_id(id).one(db).await?;
        user.ok_or(sea_orm::DbErr::RecordNotFound("User not found".to_string()))
    }
}
