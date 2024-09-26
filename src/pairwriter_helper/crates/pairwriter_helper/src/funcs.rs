use super::*;
use pairwriter::prelude::*;
use std::sync::OnceLock;
use tokio::sync::MutexGuard;
use tokio::{runtime::Runtime, sync::mpsc::UnboundedReceiver};

#[derive(Debug)]
pub enum Mode {
    Server,
    Client,
}
pub static MODE: OnceLock<Mode> = OnceLock::new();
pub static RESEIVER: OnceLock<Mutex<UnboundedReceiver<RPC>>> = OnceLock::new();
/// read the frist argument (path) and return the file
pub fn read_file(mut cx: FunctionContext) -> JsResult<JsUint8Array> {
    let path = cx.argument::<JsString>(0)?.value(&mut cx);

    // Initialize a Tokio runtime to run async code within this synchronous function
    let rt = Runtime::new().unwrap();

    // Run the async code within the Tokio runtime
    let changes = rt.block_on(async {
        match MODE.get().unwrap() {
            Mode::Server => {
                let mut api = server_api.lock().await;
                api.read_file_server(path).await
            }
            Mode::Client => {
                let mut api = client_api.get().unwrap().lock().await;
                api.read_file(path).await
            }
        }
    });

    match changes {
        Ok(data) => {
            // Create a new JsUint8Array and fill it with the data
            let js_array = JsUint8Array::from_slice(&mut cx, data.as_slice())?;
            Ok(js_array)
        }
        Err(e) => cx.throw_error(e.to_string()),
    }
}

pub fn edit_buf_with_postion(mut cx: FunctionContext) -> JsResult<JsValue> {
    // this workd for some reason so I will consider it safe
    let path: String = cx.argument::<JsString>(0)?.value(&mut cx);
    let pos: usize = cx.argument::<JsNumber>(1)?.value(&mut cx) as usize;
    let del: isize = cx.argument::<JsNumber>(2)?.value(&mut cx) as isize;
    let text: String = cx.argument::<JsString>(3)?.value(&mut cx);

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        client_api
            .get()
            .unwrap()
            .lock()
            .await
            .edit_buf(path, Some(pos), Some(del), &text)
            .await;
    });
    Ok(JsUndefined::new(&mut cx).upcast())
}

pub fn update_buf(mut cx: FunctionContext) -> JsResult<JsValue> {
    let path: String = cx.argument::<JsString>(0)?.value(&mut cx);
    let text: String = cx.argument::<JsString>(1)?.value(&mut cx);

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        match MODE.get().unwrap() {
            Mode::Server => {
                server_api
                    .lock()
                    .await
                    .edit_buf(path, None, None, &text)
                    .await
            }
            Mode::Client => {
                client_api
                    .get()
                    .unwrap()
                    .lock()
                    .await
                    .edit_buf(path, None, None, &text)
                    .await
            }
        }
    });
    Ok(JsUndefined::new(&mut cx).upcast())
}

pub fn read_file_tree(mut cx: FunctionContext) -> JsResult<JsObject> {
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        enum Either<'a> {
            Server(MutexGuard<'a, ServerApi>),
            Client(MutexGuard<'a, ClientApi>),
        }
        let api = match MODE.get().unwrap() {
            Mode::Server => Either::Server(server_api.lock().await),
            Mode::Client => Either::Client(client_api.get().unwrap().lock().await),
        };
        let (files, emty_dirs) = match api {
            Either::Server(ref api) => api.get_file_maps().await,
            Either::Client(ref api) => api.get_file_maps().await,
        };
        let (files_js, emty_dirs_js) = (cx.empty_array(), cx.empty_array());

        let obj = cx.empty_object();
        for (arr, arr_js) in [(files, files_js), (emty_dirs, emty_dirs_js)].iter() {
            for (i, path) in arr.iter().enumerate() {
                let js_path = cx.string(path);
                arr_js.set(&mut cx, i as u32, js_path)?;
            }
        }
        drop(api);
        obj.set(&mut cx, "files", files_js)?;
        obj.set(&mut cx, "emptyDirs", emty_dirs_js)?;
        Ok(obj)
    })
}

pub fn file_change(mut cx: FunctionContext) -> JsResult<JsPromise> {
    let (deferred, promise) = cx.promise();
    let channel = cx.channel();
    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        let res = rt.block_on(async {
            let mut receiver = RESEIVER.get().unwrap().lock().await;
            while let Some(rpc) = receiver.recv().await {
                match rpc {
                    RPC::EditBuffer { path, .. } | RPC::Undo { path } | RPC::Redo { path } => {
                        return Ok(path);
                    }
                    _ => {}
                }
            }
            Err(())
        });
        if let Ok(res) = res {
            deferred.settle_with(&channel, |mut cx| Ok(cx.string(res)))
        } else {
            deferred.settle_with(&channel, |mut cx| cx.error("error"))
        }
    });

    Ok(promise)
}

pub fn send_rpc(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let this = cx.this::<JsObject>()?;
    let args = [cx.argument::<JsObject>(0)?.upcast()];
    let json_rpc = cx
        .global::<JsObject>("JSON")?
        .get::<JsFunction, _, _>(&mut cx, "stringify")?
        .call(&mut cx, this, args)?
        .downcast_or_throw::<JsString, _>(&mut cx)?
        .value(&mut cx);

    let rpc = js_to_rpc(json_rpc.as_str());
    let rt = Runtime::new().unwrap();
    rt.block_on(async move {
        match MODE.get().unwrap() {
            Mode::Server => server_api.lock().await.send_rpc(rpc).await,
            Mode::Client => client_api.get().unwrap().lock().await.send_rpc(rpc).await,
        }
    });
    Ok(JsUndefined::new(&mut cx))
}
