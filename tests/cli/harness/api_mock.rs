use serde_json::{Value, json};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockBuilder};

pub fn api_request(verb: &str, route: &str) -> MockBuilder {
    Mock::given(method(verb))
        .and(path(route))
        .and(header("Gallium-CLI", env!("CARGO_PKG_VERSION")))
        .and(header(
            "User-Agent",
            "Gallium Cloud CLI (https://gallium.cloud)",
        ))
}

pub fn login_response(org: Value) -> Value {
    json!({
        "accessToken": "access-1",
        "refreshToken": "refresh-1",
        "org": org,
        "user": {
            "email": "dev@gallium.cloud.example",
            "name": "Dev",
            "flags": [],
            "darkMode": false,
        },
        "availableOrgs": [
            {"name": "Gallium Cloud Inc", "slug": "gallium", "parentOrg": null},
            {"name": "Gallium Cloud Inc - Demo", "slug": "gallium-demo", "parentOrg": {"name": "Gallium Cloud Inc", "slug": "gallium"}},
        ],
        "mfaRequired": false,
    })
}

pub fn gallium_org() -> Value {
    json!({"name": "Gallium Cloud Inc", "slug": "gallium", "parentOrg": null})
}
