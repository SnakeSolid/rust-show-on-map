macro_rules! check_text {
    ( $x: expr, $msg: expr ) => {{
        match $x {
            Ok(value) => value,
            Err(_) => {
                warn!("Handler error: {}", $msg);

                return Ok(Response::with(status::InternalServerError));
            }
        }
    }};
}

macro_rules! check_error {
    ( $x: expr, $content_type: expr, $callback: expr ) => {{
        match $x {
            Ok(value) => value,
            Err(error) => {
                warn!("Handler error: {}", error);

                return match serde_json::to_string(&$callback(error)) {
                    Ok(body) => Ok(Response::with(($content_type, status::Ok, body))),
                    Err(_) => Ok(Response::with(status::InternalServerError)),
                };
            }
        }
    }};
}

macro_rules! check_server_error {
    ( $x: expr ) => {{
        match $x {
            Ok(value) => value,
            Err(error) => {
                warn!("Handler error: {}", error);

                return Ok(Response::with(status::InternalServerError));
            }
        }
    }};
}
