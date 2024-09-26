use std::sync::OnceLock;

use super::*;
use pairwriter::client_import::*;
use tokio::{runtime::Runtime, sync::mpsc::UnboundedReceiver};

use super::funcs::*;

pub fn connect_as_client_js(mut cx: FunctionContext) -> JsResult<JsObject> {
    let url = cx.argument::<JsString>(0)?.value(&mut cx);
    let username = cx.argument::<JsString>(1)?.value(&mut cx);
    // use std to spawn the client_api.get().unwrap()
    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(connect_as_client(url, username));
    });
    let mut i = 0;
    while client_api.get().is_none() && i < 5 {
        std::thread::sleep(std::time::Duration::from_secs(1));
        i += 1;
    }
    if i == 5 {
        return cx.throw_error("could not connect to the server");
    }
    MODE.set(Mode::Client).unwrap();
    RT.block_on(async {
        let _ = RESEIVER.set(Mutex::new(
            client_api
                .get()
                .unwrap()
                .lock()
                .await
                .get_receiver()
                .unwrap(),
        ));
    });

    let js_object = JsObject::new(&mut cx);

    add_functions!( js_object = {
        "readFile" : read_file,
        "editBuf" : edit_buf_with_postion,
        "readFileTree" : read_file_tree,
        "updateBuf" : update_buf,
        "fileChange" : file_change,
        "sendRpc" : send_rpc,
    },cx);

    Ok(js_object)
}
