use actix_web::{web, HttpRequest, HttpResponse};
use api_models::user_role::{self as user_role_api, role as role_api};
use common_enums::TokenPurpose;
use router_env::Flow;

use super::AppState;
use crate::{
    core::{
        api_locking,
        user_role::{self as user_role_core, role as role_core},
    },
    services::{
        api,
        authentication::{self as auth},
        authorization::permissions::Permission,
    },
};

pub async fn get_authorization_info(
    state: web::Data<AppState>,
    http_req: HttpRequest,
) -> HttpResponse {
    let flow = Flow::GetAuthorizationInfo;
    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &http_req,
        (),
        |state, _: (), _, _| async move {
            user_role_core::get_authorization_info_with_groups(state).await
        },
        &auth::JWTAuth(Permission::UsersRead),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn get_role_from_token(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let flow = Flow::GetRoleFromToken;

    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &req,
        (),
        |state, user, _, _| async move {
            role_core::get_role_from_token_with_groups(state, user).await
        },
        &auth::DashboardNoPermissionAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn create_role(
    state: web::Data<AppState>,
    req: HttpRequest,
    json_payload: web::Json<role_api::CreateRoleRequest>,
) -> HttpResponse {
    let flow = Flow::CreateRole;
    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &req,
        json_payload.into_inner(),
        role_core::create_role,
        &auth::JWTAuth(Permission::UsersWrite),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn list_all_roles(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let flow = Flow::ListRoles;
    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &req,
        (),
        |state, user, _, _| async move {
            role_core::list_invitable_roles_with_groups(state, user).await
        },
        &auth::JWTAuth(Permission::UsersRead),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn get_role(
    state: web::Data<AppState>,
    req: HttpRequest,
    path: web::Path<String>,
) -> HttpResponse {
    let flow = Flow::GetRole;
    let request_payload = user_role_api::role::GetRoleRequest {
        role_id: path.into_inner(),
    };
    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &req,
        request_payload,
        |state, user, payload, _| async move {
            role_core::get_role_with_groups(state, user, payload).await
        },
        &auth::JWTAuth(Permission::UsersRead),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn update_role(
    state: web::Data<AppState>,
    req: HttpRequest,
    json_payload: web::Json<role_api::UpdateRoleRequest>,
    path: web::Path<String>,
) -> HttpResponse {
    let flow = Flow::UpdateRole;
    let role_id = path.into_inner();

    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &req,
        json_payload.into_inner(),
        |state, user, req, _| role_core::update_role(state, user, req, &role_id),
        &auth::JWTAuth(Permission::UsersWrite),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn update_user_role(
    state: web::Data<AppState>,
    req: HttpRequest,
    json_payload: web::Json<user_role_api::UpdateUserRoleRequest>,
) -> HttpResponse {
    let flow = Flow::UpdateUserRole;
    let payload = json_payload.into_inner();
    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &req,
        payload,
        user_role_core::update_user_role,
        &auth::JWTAuth(Permission::UsersWrite),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn accept_invitation(
    state: web::Data<AppState>,
    req: HttpRequest,
    json_payload: web::Json<user_role_api::AcceptInvitationRequest>,
) -> HttpResponse {
    let flow = Flow::AcceptInvitation;
    let payload = json_payload.into_inner();
    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &req,
        payload,
        |state, user, req_body, _| user_role_core::accept_invitation(state, user, req_body),
        &auth::DashboardNoPermissionAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn merchant_select(
    state: web::Data<AppState>,
    req: HttpRequest,
    json_payload: web::Json<user_role_api::MerchantSelectRequest>,
) -> HttpResponse {
    let flow = Flow::MerchantSelect;
    let payload = json_payload.into_inner();
    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &req,
        payload,
        |state, user, req_body, _| async move {
            user_role_core::merchant_select_token_only_flow(state, user, req_body).await
        },
        &auth::SinglePurposeJWTAuth(TokenPurpose::AcceptInvite),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn delete_user_role(
    state: web::Data<AppState>,
    req: HttpRequest,
    payload: web::Json<user_role_api::DeleteUserRoleRequest>,
) -> HttpResponse {
    let flow = Flow::DeleteUserRole;
    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &req,
        payload.into_inner(),
        user_role_core::delete_user_role,
        &auth::JWTAuth(Permission::UsersWrite),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn get_role_information(
    state: web::Data<AppState>,
    http_req: HttpRequest,
) -> HttpResponse {
    let flow = Flow::GetRolesInfo;

    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &http_req,
        (),
        |_, _: (), _, _| async move {
            user_role_core::get_authorization_info_with_group_tag().await
        },
        &auth::JWTAuth(Permission::UsersRead),
        api_locking::LockAction::NotApplicable,
    ))
    .await
}

pub async fn list_users_in_lineage(state: web::Data<AppState>, req: HttpRequest) -> HttpResponse {
    let flow = Flow::ListUsersInLineage;

    Box::pin(api::server_wrap(
        flow,
        state.clone(),
        &req,
        (),
        |state, user_from_token, _, _| {
            user_role_core::list_users_in_lineage(state, user_from_token)
        },
        &auth::DashboardNoPermissionAuth,
        api_locking::LockAction::NotApplicable,
    ))
    .await
}
