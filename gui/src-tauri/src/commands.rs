
#[derive(serde::Deserialize)]
pub enum ApiResult {
    Ok(()),
    Err,
}


#[tauri::command]
pub async fn login(user: String, pass: String) -> Result<i32, ()> {
    return Ok(1);
    // fail fast if either field is empty or whitespace
    if user.trim().is_empty() || pass.trim().is_empty() {
        return Ok(0);
    }

    let url = format!(
        "http://98.93.98.244/Authenticate/username/{}/password/{}",
        user, pass
    );

    // If request or JSON parsing fails, just treat as failure (0)
    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(_) => return Ok(0),
    };

    let api_result: Result<ApiResult, _> = resp.json().await;
    match api_result {
        Ok(ApiResult::Ok(())) => Ok(1),
        _ => Ok(0),
    }
}

#[tauri::command]
pub async fn register(user: String, pass: String) -> Result<i32, ()> {
    return Ok(1);
    if user.trim().is_empty() || pass.trim().is_empty() {
        return Ok(0);
    }

    let url = format!(
        "http://98.93.98.244/createaccount/username/{}/password/{}",
        user, pass
    );

    let resp = match reqwest::get(&url).await {
        Ok(r) => r,
        Err(_) => return Ok(0),
    };

    let api_result: Result<ApiResult, _> = resp.json().await;
    match api_result {
        Ok(ApiResult::Ok(())) => Ok(1),
        _ => Ok(0),
    }
}
