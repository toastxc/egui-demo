use crate::App;
use reywen_http::engines::hyper::Method;
use std::sync::Arc;

impl App {
    // this function calls the requests the internet
    pub fn request(&mut self) {
        // all fields that are a part of self that you use MUST be called here
        let runtime = Arc::clone(&self.runtime);
        let engine = Arc::clone(&self.engine);
        let label_text = Arc::clone(&self.label1_text);
        let spin = Arc::clone(&self.spin);

        // async code can only be executed in this block
        runtime.spawn(async move {
            // set the spinner to appear
            *spin.write().unwrap() = true;
            // http request - turn into string
            let str = String::from_utf8(
                engine
                    .request_raw(Method::GET, "https://repo.toastxc.xyz/test", None)
                    .await
                    .unwrap_or("Oops, Something went wrong...".to_string().into_bytes()),
            )
            .unwrap();
            // save data
            *label_text.write().unwrap() = str;

            // remove spinner
            *spin.write().unwrap() = false;
        });
    }

    pub fn do_stuff(&mut self) {
        // this function, and any function called by the update function will run once per frame, be careful!
    }
}
