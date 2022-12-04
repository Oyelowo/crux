//! TODO mod docs

use crux_core::{
    capability::CapabilityContext, channels::Sender, executor::Spawner, Capability, Command,
};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Display)]
pub enum HttpMethod {
    #[display(fmt = "GET")]
    Get,
    #[display(fmt = "HEAD")]
    Head,
    #[display(fmt = "POST")]
    Post,
    #[display(fmt = "PUT")]
    Put,
    #[display(fmt = "DELETE")]
    Delete,
    #[display(fmt = "CONNECT")]
    Connect,
    #[display(fmt = "OPTIONS")]
    Options,
    #[display(fmt = "TRACE")]
    Trace,
    #[display(fmt = "PATCH")]
    Patch,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    // TODO support headers
}

#[derive(Serialize, Deserialize)]
pub struct HttpResponse {
    pub status: u16,   // FIXME this probably should be a giant enum instead.
    pub body: Vec<u8>, // TODO support headers
}

impl crux_core::Effect for HttpRequest {
    type Response = HttpResponse;
}

pub struct Http<Ev> {
    context: CapabilityContext<HttpRequest, Ev>,
}

impl<Ev> Http<Ev>
where
    Ev: 'static,
{
    pub fn new(context: CapabilityContext<HttpRequest, Ev>) -> Self {
        Self { context }
    }

    pub fn get<F>(&self, url: Url, callback: F)
    where
        Ev: 'static,
        F: Fn(HttpResponse) -> Ev + Send + 'static,
    {
        self.send(HttpMethod::Get, url, callback)
    }

    pub fn get_json<T, F>(&self, url: Url, callback: F)
    where
        T: serde::de::DeserializeOwned,
        F: Fn(T) -> Ev + Send + Clone + 'static,
    {
        let ctx = self.context.clone();
        self.context.spawn(async move {
            let request = HttpRequest {
                method: HttpMethod::Get.to_string(),
                url: url.to_string(),
            };
            let resp = ctx.effect(request).await;

            let data =
                serde_json::from_slice::<T>(&resp.body).expect("TODO: do something sensible here");

            ctx.send_event(callback(data))
        });
    }

    pub fn send<F>(&self, method: HttpMethod, url: Url, callback: F)
    where
        Ev: 'static,
        F: Fn(HttpResponse) -> Ev + Send + 'static,
    {
        let request = HttpRequest {
            method: method.to_string(),
            url: url.to_string(),
        };

        self.context.run_command(Command::new(request, callback))
    }
}

impl<Ef> Capability<Ef> for Http<Ef> {
    type MappedSelf<MappedEv> = Http<MappedEv>;

    fn map_event<F, NewEvent>(&self, f: F) -> Self::MappedSelf<NewEvent>
    where
        F: Fn(NewEvent) -> Ef + Send + Sync + Copy + 'static,
        Ef: 'static,
        NewEvent: 'static,
    {
        Http::new(self.context.map_event(f))
    }
}
