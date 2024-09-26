use std::sync::OnceLock;

use super::funcs::*;
use super::*;
use pairwriter::server_import::*;
use tokio::{runtime::Runtime, sync::mpsc::UnboundedReceiver};
/// this should add functions to obj in the form of ("name", function)
pub fn start_server_js(mut cx: FunctionContext) -> JsResult<JsObject> {
    let port = cx.argument::<JsNumber>(0)?.value(&mut cx) as u16;
    let current_dir = cx.argument::<JsString>(1)?.value(&mut cx);
    // console.log
    let console_log = cx
        .global::<JsObject>("console")?
        .get::<JsFunction, _, _>(&mut cx, "log")?;
    // get current working directory
    // set current working directory
    std::env::set_current_dir(current_dir).unwrap();
    let cwd = std::env::current_dir().unwrap();
    let args = [cx.string(cwd.to_str().unwrap()).upcast::<JsValue>()];
    let this = cx.undefined();
    console_log.call(&mut cx, this, args.as_slice())?;
    // use std to spawn the server_api
    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(start_server(port));
    });
    RT.block_on(async {
        let _ = RESEIVER.set(Mutex::new(server_api.lock().await.take_receiver()));
    });
    MODE.set(Mode::Server).unwrap();
    let js_object = JsObject::new(&mut cx);
    add_functions!(js_object = {
        "readFile" : read_file,
        "editBuf" : edit_buf_with_postion,
        "readFileTree" : read_file_tree,
        "updateBuf" : update_buf,
        "fileChange" : file_change,
        "sendRpc" : send_rpc,
    },cx);

    Ok(js_object)
}
