use std::convert::Infallible;

use bytes::BufMut;
use futures::{TryFutureExt, TryStreamExt};
use uuid::Uuid;
use warp::{Reply, hyper::StatusCode, multipart::{FormData, Part}};

use crate::{api::{models::welcome::WelcomeRequestData, util}, db::{Database, queries::welcome::WelcomeModuleInsert}, modules::welcome::{WelcomeContent, WelcomeExpanded, WelcomeJoinContent, WelcomeJoinDmContent, WelcomeJoinRolesContent, WelcomeLeaveContent}};

pub async fn fetch_module_for_guild(guild_id: u64, db: Database) -> Result<impl warp::Reply, Infallible> {
    match _fetch_expanded_module(&db, guild_id).await {
        Ok(m) => Ok(warp::reply::json(&m).into_response()),
        Err(err) => Ok(err)
    }
}

pub async fn update_module_for_guild(guild_id: u64, data: WelcomeRequestData, db: Database) -> Result<impl warp::Reply, Infallible> {
    let mut module = match _fetch_expanded_module(&db, guild_id).await {
        Ok(m) => m,
        Err(err) => {
            return Ok(err);
        }
    };

    let mut tx = match db.pool.begin().await {
        Ok(tx) => tx,
        Err(err) => {
            return Ok(util::create_error_response(
                StatusCode::INTERNAL_SERVER_ERROR, 
                format!("Internal database error obtaining transaction connection: {:?}", err)
            ));
        }
    };
    
    if let Some(enabled) = data.enabled { 
        module.enabled = enabled;

        if let Err(err) = db.welcome().module_upsert_with(guild_id, &WelcomeContent {
            enabled: Some(module.enabled)
        }, &mut tx).await {
            return Ok(util::create_error_response(
                StatusCode::INTERNAL_SERVER_ERROR, 
                format!("Failed to update welcome content: {:?}", err)
            ));
        };
    }
    if let Some(join) = data.join {
        if let Some(mut m_join) = module.join.or(Some(WelcomeJoinContent::default())) {
            m_join.enabled = join.enabled;
            m_join.message_type = join.message_type;
            m_join.channel_id = join.channel_id;
            m_join.content = join.content;
            m_join.embed = join.embed;
    
            if let Err(err) = db.welcome().module_join_upsert_with(module.id,&m_join, &mut tx).await {
                return Ok(util::create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to update welcome_join content: {:?}", err)
                ));
            };
        }
    }
    if let Some(join_dm) = data.join_dm {
        if let Some(mut m_join_dm) = module.join_dm.or(Some(WelcomeJoinDmContent::default())) {
            m_join_dm.enabled = join_dm.enabled;
            m_join_dm.message_type = join_dm.message_type;
            m_join_dm.content = join_dm.content;
            m_join_dm.embed = join_dm.embed;
    
            if let Err(err) = db.welcome().module_join_dm_upsert_with(module.id,&m_join_dm, &mut tx).await {
                return Ok(util::create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    format!("Failed to update welcome_join_dm content: {:?}", err)
                ));
            };
        }
    }
    if let Some(join_roles) = data.join_roles {
        if let Some(mut m_join_roles) = module.join_roles.or(Some(WelcomeJoinRolesContent::default())) {
            m_join_roles.enabled = join_roles.enabled;
    
            if let Err(err) = db.welcome().module_join_roles_upsert_with(module.id,&m_join_roles, &mut tx).await {
                return Ok(util::create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    format!("Failed to update welcome_join_roles content: {:?}", err)
                ));
            };
        }
    }
    if let Some(leave) = data.leave {
        if let Some(mut m_leave) = module.leave.or(Some(WelcomeLeaveContent::default())) {
            m_leave.enabled = leave.enabled;
    
            if let Err(err) = db.welcome().module_leave_update_with(module.id,&m_leave, &mut tx).await {
                return Ok(util::create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    format!("Failed to update welcome_leave content: {:?}", err)
                ));
            };
        }
    }

    if let Err(err) = tx.commit().await {
        return Ok(util::create_error_response(
            StatusCode::INTERNAL_SERVER_ERROR, 
            format!("Internal database error while committing welcome module update: {:?}", err)
        ));
    }
    
    Ok(warp::reply::reply().into_response())
}

pub async fn upload_images_for_guild(guild_id: u64, form: FormData) -> Result<impl warp::Reply, Infallible> {
    #[derive(serde::Serialize)]
    struct UploadResponseData {
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "join.author.image")]
        pub join_author_image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "join.image")]
        pub join_image: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "join.thumbnail")]
        pub join_thumbnail: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        #[serde(rename = "join.footer.image")]
        pub join_footer_image: Option<String>,
    }

    let imgur_client_id = match dotenv::var("IMGUR_CLIENT_ID") {
        Ok(s) => s,
        Err(e) => {
            eprintln!("No imgur client id set: {}", e);
            return Ok(util::create_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("No imgur client id set: {}", e)
            ));
        }
    };

    let mut response_data = UploadResponseData {
        join_author_image: None,
        join_image: None,
        join_thumbnail: None,
        join_footer_image: None,
    };
    let parts: Vec<Part> = match form.try_collect().await {
        Ok(parts) => parts,
        Err(e) => {
            eprintln!("welcome file upload error: {}", e);
            return Ok(util::create_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Failed to process file upload: {}", e)
            ));
        }
    };
    for p in parts {
        let name = match p.name() {
            "join.author.image" |
            "join.image" |
            "join.thumbnail" |
            "join.footer.image" => p.name().to_string(),
            _ => {
                eprintln!("Invalid part name supplied: {}", p.name());
                return Ok(util::create_error_response(
                    StatusCode::BAD_REQUEST,
                    format!("Invalid part name supplied: {}", p.name())
                ));
            }
        };
        
        let value = match p
            .stream()
            .try_fold(Vec::new(), |mut vec, data| {
                vec.put(data);
                async move { Ok(vec) }
            })
            .await {
                Ok(val) => val,
                Err(e) => {
                    eprintln!("reading file error: {}", e);
                    return Ok(util::create_error_response(
                        StatusCode::BAD_REQUEST,
                        format!("Failed to read file: {}", e)
                    ));
                }
            };
        
        let path = match imgur2018::imgur_upload(&imgur_client_id, value).await {
            Ok(url) => url.to_string(),
            Err(e) => {
                eprintln!("hosting file error: {}", e);
                return Ok(util::create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Failed to host file: {}", e)
                ));
            }
        };

        match name.as_str() {
            "join.author.image" => response_data.join_author_image = Some(path),
            "join.image" => response_data.join_image = Some(path),
            "join.thumbnail" => response_data.join_thumbnail = Some(path),
            "join.footer.image" => response_data.join_footer_image = Some(path),
            _ => {
                eprintln!("Should never get here: {}", name);
                return Ok(util::create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR,
                    format!("Invalid part name supplied: {}", name)
                ));
            }
        }
    }
    Ok(warp::reply::json(&response_data).into_response())
}

pub async fn _fetch_expanded_module(db: &Database, guild_id: u64) -> Result<WelcomeExpanded, warp::reply::Response> {
    match db.welcome().module_expanded_fetch_by_guild_id(guild_id).await {
        Ok(m) => Ok(m),
        Err(err) => if let sqlx::Error::RowNotFound = err {
            if let Ok(module) = db.welcome().module_insert(WelcomeModuleInsert {
                guild_id,
                enabled: false,
            }).await {
                Ok(WelcomeExpanded::from_module(module))
            } else {
                return Err(util::create_error_response(
                    StatusCode::INTERNAL_SERVER_ERROR, 
                    format!("Failed to create module data for \"welcome\": {:?}", err)
                ));
            }
        } else {
            return Err(util::create_error_response(
                StatusCode::INTERNAL_SERVER_ERROR, 
                format!("Internal database error when fetching expanded welcome module: {:?}", err)
            ));
        }
    }
}