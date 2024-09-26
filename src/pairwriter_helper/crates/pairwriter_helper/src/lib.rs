use neon::prelude::*;
use pairwriter::prelude::RPC;
use tokio::sync::Mutex;
mod client_api;
mod server_api; 
mod funcs;

lazy_static::lazy_static! {
    static ref RT: tokio::runtime::Runtime = tokio::runtime::Runtime::new().unwrap();
}


/// take a json and convert it to RPC
pub fn js_to_rpc(json:&str) -> RPC {
    serde_json::from_str(json).unwrap()
}


#[macro_export]
macro_rules! add_functions {
    ($obj:ident = {
        $($name:literal : $fn:ident,)*
    }, $cx:ident)
    => {{
        $(
            let name = JsFunction::new(&mut $cx, $fn)?;
            $obj.set(&mut $cx, $name, name)?;
        )*
    }};
}
#[neon::main]
pub fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("startServer", server_api::start_server_js)?;
    cx.export_function("clientAsConnect", client_api::connect_as_client_js)?;
    Ok(())
}
