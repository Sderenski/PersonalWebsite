use {std::future::Future, wasm_bindgen_futures::spawn_local};

// These function return futures which themselves return either the actual data 
// or returns an error. 


// Takes a future F (future that was converted from a JS Promise) and a Handler H.
// The future returns a value of T. 

pub fn handle_future<F, H, T>(future: F, handler: H) 
where 
    F: Future<Output = T> + 'static,
    H: Fn(T) + 'static,
{
    spawn_local(async move {
        let rs: T = future.await;
        handler(rs);
    });
}