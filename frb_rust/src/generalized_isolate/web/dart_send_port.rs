use super::into_dart::IntoDart;
use crate::for_generated::DartNativeSendPort;
use crate::generalized_isolate::web::box_into_dart::BoxIntoDart;

#[derive(Debug, Clone)]
pub struct DartSendPort(i64);

impl DartSendPort {
    pub fn new(native: DartNativeSendPort) -> Self {
        Self(todo!())
    }

    pub fn post(&self, msg: impl IntoDart + Send + 'static) -> bool {
        // to test whether "send to another thread" can compile
        let msg_boxed: Box<dyn BoxIntoDart + Send> = Box::new(msg);
        std::thread::spawn(move || {
            let dart_abi = msg_boxed.box_into_dart();
            let _ = dart_abi;
        });
        todo!()
    }
}
