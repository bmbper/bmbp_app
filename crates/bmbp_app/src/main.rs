use async_static::async_static;
use bmbp_app_util::{
    auth_token_middle, auth_user_middle, orm_middle, register_app_orm, BMBP_WHITE_URLS,
};
use bmbp_config::build_bmbp_config_router;
use bmbp_ctx_vars::BMBP_CONTEXT_VARS;
use bmbp_lib_ui::build_bmbp_ui_lib_router;
use bmbp_rdbc::RdbcOrm;
use bmbp_rdbc::{RdbcDataBase, RdbcDataSource};
use salvo::prelude::*;

async_static! {
    pub static ref RdbcOrmIns:RdbcOrm = build_orm().await;
}
pub async fn build_orm() -> RdbcOrm {
    let mut ds = RdbcDataSource::new();
    ds.set_typ(RdbcDataBase::Postgres)
        .set_host(Some("127.0.0.1".to_string()))
        .set_database(Some("bmbp".to_string()))
        .set_port(Some(5432))
        .set_username(Some("bmbp".to_string()))
        .set_password(Some("".to_string()));
    match RdbcOrm::new(ds).await {
        Ok(orm) => {
            tracing::info!("初始化数据库成功");
            orm
        }
        Err(err) => {
            panic!("初始化数据库失败:{}", err.get_msg())
        }
    }
}

pub async fn init_orm() {
    let rs = RdbcOrmIns.await;
    register_app_orm(rs);
}

#[tokio::main]
async fn main() {
    init_white_urls();
    init_orm().await;
    tracing_subscriber::fmt().init();
    let host = "0.0.0.0:9002";
    tracing::info!("启动初始化服务,监听地址:{}......", host);
    let acceptor = TcpListener::new(host).bind().await;
    let router = Router::new()
        .push(build_bmbp_ui_lib_router())
        .hoop(auth_token_middle)
        .hoop(auth_user_middle)
        .hoop(orm_middle)
        .push(build_bmbp_config_router());
    Server::new(acceptor).serve(router).await;
}

fn init_white_urls() {
    (&*BMBP_CONTEXT_VARS).set_value(BMBP_WHITE_URLS.to_string(), "/<**>");
}
