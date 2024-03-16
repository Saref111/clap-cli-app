use super::wrap_html;
#[test]
fn test_wrap_html() {
    let html = wrap_html("Hello, world!", None);
    assert_eq!(html, "<!DOCTYPE html><html><head><meta charset=\"utf8\"></head><body>Hello, world!</body></html>");
}

#[test]
fn test_wrap_html_with_css() {
    let html = wrap_html("Hello, world!", Some("style.css"));
    assert_eq!(html, "<!DOCTYPE html><html><head><meta charset=\"utf8\"><link rel=\"stylesheet\" type=\"text/css\" href=\"style.css\"></head><body>Hello, world!</body></html>");
}
