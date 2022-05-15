use crate::{Result, handler::{HtmlView, render, log_error}, view::client::index::Index};

pub async fn index() ->Result<HtmlView> {
    let handler_name = "client/index/index";
    let tmpl = Index{};
    render(tmpl).map_err(log_error(handler_name))
}
