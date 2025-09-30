use axum::{http::StatusCode, response::Html, routing::get, Router};

const OPENAPI_SPEC_YAML: &str =
    include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../openapi.yaml"));

pub fn routes() -> Router<std::sync::Arc<crate::AppState>> {
    Router::new()
        .route("/docs", get(swagger_ui))
        .route("/docs/openapi.yaml", get(openapi_spec))
}

async fn swagger_ui() -> Html<&'static str> {
    const HTML: &str = r#"<!DOCTYPE html>
<html lang=\"en\">
  <head>
    <meta charset=\"utf-8\" />
    <title>SUT API Docs</title>
    <link rel=\"stylesheet\" href=\"https://cdn.jsdelivr.net/npm/swagger-ui-dist@5/swagger-ui.css\" />
    <style>
      html, body { margin: 0; padding: 0; background: #fafafa; }
      #swagger-ui { width: 100%; box-sizing: border-box; }
    </style>
  </head>
  <body>
    <div id=\"swagger-ui\"></div>
    <script src=\"https://cdn.jsdelivr.net/npm/swagger-ui-dist@5/swagger-ui-bundle.js\"></script>
    <script src=\"https://cdn.jsdelivr.net/npm/swagger-ui-dist@5/swagger-ui-standalone-preset.js\"></script>
    <script>
      window.addEventListener('load', () => {
        window.ui = SwaggerUIBundle({
          url: '/docs/openapi.yaml',
          dom_id: '#swagger-ui',
          presets: [SwaggerUIBundle.presets.apis, SwaggerUIStandalonePreset],
          layout: 'StandaloneLayout'
        });
      });
    </script>
  </body>
</html>"#;
    Html(HTML)
}

async fn openapi_spec() -> Result<axum::response::Response, StatusCode> {
    axum::response::Response::builder()
        .header(axum::http::header::CONTENT_TYPE, "application/yaml")
        .body(OPENAPI_SPEC_YAML.into())
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}
