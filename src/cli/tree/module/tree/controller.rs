use actix_web::http::StatusCode;
use actix_web::web;

use crate::module::tree::ports::*;
use crate::rest::error::Error;

pub fn configure<T: 'static + TreeService>(service: web::Data<T>, cfg: &mut web::ServiceConfig) {
    cfg.app_data(service);
    cfg.route("/update_state", web::post().to(update::<T>));
    cfg.route("/proof", web::post().to(proof::<T>));
    cfg.route("/health", web::get().to(health::<T>));
}

pub async fn update<T: TreeService>(
    service: web::Data<T>,
    body: web::Json<UpdateTreeInput>,
) -> Result<(web::Json<UpdateTreeOutput>, StatusCode), Error> {
    match service.update(body.0) {
        Ok(root) => Ok((web::Json(root), StatusCode::OK)),
        Err(e) => Err(Error::TreeError(e)),
    }
}

pub async fn proof<T: TreeService>(
    service: web::Data<T>,
    body: web::Json<GetProofInput>,
) -> Result<(web::Json<GetProofOutput>, StatusCode), Error> {
    match service.proof(body.0) {
        Ok(proof) => Ok((web::Json(proof), StatusCode::OK)),
        Err(e) => Err(Error::TreeError(e)),
    }
}

pub async fn health<T: TreeService>(
    _service: web::Data<T>,
) -> Result<(web::Json<HealthOutput>, StatusCode), Error> {
    Ok((web::Json(HealthOutput { success: true }), StatusCode::OK))
}

#[cfg(test)]
mod tests {

    use crate::module::tree::controller::*;
    use crate::module::tree::error::TreeError;
    use crate::module::tree::ports::{MockTreeService, UpdateTreeInput, UpdateTreeOutput};
    use crate::module::utils::error::UtilsError;
    use actix_web::web;

    #[actix_rt::test]
    async fn test_update_ok() {
        let mut tree_service = MockTreeService::new();

        tree_service.expect_update().times(1).returning(|_| {
            Ok(UpdateTreeOutput {
                root: "b1f40443435cf3d9885bd0ba7de5a51b2e94dc18d5634c0d5e295b7bdab92786"
                    .to_string(),
            })
        });

        assert!(update(
            web::Data::new(tree_service),
            web::Json(UpdateTreeInput {
                leaves: vec![
                    "b1f40443435cf3d9885bd0ba7de5a51b2e94dc18d5634c0d5e295b7bdab92781".to_string()
                ],
                shard_id: None,
                state_id: None,
                root: None,
            }),
        )
        .await
        .is_ok());
    }

    #[actix_rt::test]
    async fn test_update_err() {
        let mut tree_service = MockTreeService::new();

        tree_service
            .expect_update()
            .times(1)
            .returning(|_| Err(TreeError::UtilsError(UtilsError::PathError())));

        assert!(update(
            web::Data::new(tree_service),
            web::Json(UpdateTreeInput {
                leaves: vec![
                    "b1f40443435cf3d9885bd0ba7de5a51b2e94dc18d5634c0d5e295b7bdab92781".to_string()
                ],
                shard_id: None,
                state_id: None,
                root: None,
            }),
        )
        .await
        .is_err());
    }

    #[actix_rt::test]
    async fn test_proof_ok() {
        let mut tree_service = MockTreeService::new();

        tree_service.expect_proof().times(1).returning(|_| {
            Ok(GetProofOutput {
                nodes: vec![
                    "b1f40443435cf3d9885bd0ba7de5a51b2e94dc18d5634c0d5e295b7bdab92786".to_string(),
                ],
                bitmap: "001".to_string(),
                depth: "000200020001".to_string(),
            })
        });

        assert!(proof(
            web::Data::new(tree_service),
            web::Json(GetProofInput {
                leaves: vec![
                    "b1f40443435cf3d9885bd0ba7de5a51b2e94dc18d5634c0d5e295b7bdab92781".to_string(),
                    "b1f40443435cf3d9885bd0ba7de5a51b2e94dc18d5634c0d5e295b7bdab92782".to_string()
                ],
                shard_id: None,
                state_id: None,
                root: "b1f40443435cf3d9885bd0ba7de5a51b2e94dc18d5634c0d5e295b7bdab92789"
                    .to_string()
            }),
        )
        .await
        .is_ok());
    }

    #[actix_rt::test]
    async fn test_proof_err() {
        let mut tree_service = MockTreeService::new();

        tree_service
            .expect_proof()
            .times(1)
            .returning(|_| Err(TreeError::UtilsError(UtilsError::PathError())));

        assert!(proof(
            web::Data::new(tree_service),
            web::Json(GetProofInput {
                leaves: vec![
                    "b1f40443435cf3d9885bd0ba7de5a51b2e94dc18d5634c0d5e295b7bdab92781".to_string()
                ],
                shard_id: Some("shard1".to_string()),
                state_id: None,
                root: "b1f40443435cf3d9885bd0ba7de5a51b2e94dc18d5634c0d5e295b7bdab92789"
                    .to_string()
            }),
        )
        .await
        .is_err());
    }
}
