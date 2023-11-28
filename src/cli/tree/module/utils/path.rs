use super::error::UtilsError;


pub fn calculate_path(substate: Option<String>, state_id: Option<i32>) -> Result<String,UtilsError> {
    Ok(
        match (substate, state_id) {
            (Some(shard), Some(state)) => format!("substates/{}/{}/", shard, state).to_string(),
            (Some(_), None) => return Err(UtilsError::PathError()),
            _ => String::from("state/"),
        }
    )
}