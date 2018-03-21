use sapper::{Error as SapperError, Request, Response, Result as SapperResult, SapperModule,
             SapperRouter};
use sapper_std::{render, Context, QueryParams, SessionVal};
use uuid::Uuid;

use super::super::{admin_verification_cookie, ArticlesWithTag, Postgresql, Redis, Tags};

pub struct Admin;

impl Admin {
    fn admin(_req: &mut Request) -> SapperResult<Response> {
        let web = Context::new();
        res_html!("admin/admin.html", web)
    }

    fn admin_list(_req: &mut Request) -> SapperResult<Response> {
        let web = Context::new();
        res_html!("admin/admin_list.html", web)
    }

    fn new(req: &mut Request) -> SapperResult<Response> {
        let mut web = Context::new();
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        match Tags::view_list_tag(&pg_pool) {
            Ok(ref data) => web.add("tags", data),
            Err(err) => println!("No tags, {}", err),
        }
        res_html!("admin/article_new.html", web)
    }

    fn admin_view_article(req: &mut Request) -> SapperResult<Response> {
        let params = get_query_params!(req);
        let article_id = t_param_parse!(params, "id", Uuid);
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        let mut web = Context::new();

        match ArticlesWithTag::query_article(&pg_pool, article_id, true) {
            Ok(ref data) => web.add("article", data),
            Err(err) => println!("{}", err),
        }
        res_html!("admin/article_view.html", web)
    }

    fn article_edit(req: &mut Request) -> SapperResult<Response> {
        let params = get_query_params!(req);
        let article_id = t_param_parse!(params, "id", String);
        let mut web = Context::new();
        web.add("id", &article_id);
        let pg_pool = req.ext().get::<Postgresql>().unwrap().get().unwrap();
        match Tags::view_list_tag(&pg_pool) {
            Ok(ref data) => web.add("tags", data),
            Err(err) => println!("No tags, {}", err),
        }
        res_html!("admin/article_edit.html", web)
    }

    fn tags(_req: &mut Request) -> SapperResult<Response> {
        let web = Context::new();
        res_html!("admin/tags.html", web)
    }

    fn users(_req: &mut Request) -> SapperResult<Response> {
        let web = Context::new();
        res_html!("admin/users.html", web)
    }

    fn visitor_ip_log(_req: &mut Request) -> SapperResult<Response> {
        let web = Context::new();
        res_html!("admin/ip.html", web)
    }
}

impl SapperModule for Admin {
    #[allow(unused_assignments)]
    fn before(&self, req: &mut Request) -> SapperResult<()> {
        let cookie = req.ext().get::<SessionVal>();
        let redis_pool = req.ext().get::<Redis>().unwrap();
        match admin_verification_cookie(cookie, redis_pool) {
            true => Ok(()),
            false => {
                let res = json!({
                    "status": false,
                    "error": String::from("Verification error")
                });
                Err(SapperError::CustomJson(res.to_string()))
            }
        }
    }

    fn after(&self, _req: &Request, _res: &mut Response) -> SapperResult<()> {
        Ok(())
    }

    fn router(&self, router: &mut SapperRouter) -> SapperResult<()> {
        // http get /admin
        router.get("/admin", Admin::admin);

        router.get("/admin/list", Admin::admin_list);

        router.get("/admin/new", Admin::new);

        router.get("/admin/article/view", Admin::admin_view_article);

        router.get("/admin/article/edit", Admin::article_edit);

        router.get("/admin/tags", Admin::tags);

        router.get("/admin/users", Admin::users);

        router.get("/admin/ip", Admin::visitor_ip_log);

        Ok(())
    }
}
