#[cfg(feature = "cbor")]
pub mod cbor;
#[cfg(feature = "json")]
pub mod json;
#[cfg(feature = "manual")]
pub mod manual;
#[cfg(feature = "url_json")]
pub mod url_json;

use crate::response::Res;
pub use crate::{error::ServerFnError, request::Req};
use async_trait::async_trait;
pub trait Encoding {
    const REQUEST_CONTENT_TYPE: &'static str;
    const RESPONSE_CONTENT_TYPE: &'static str;
}

#[async_trait]
pub trait Codec<
    RequestBody,
    ResponseBody,
    Request,
    Response,
    Enc,
    IntoReq,
    IntoResp,
    IntoReqBody,
    IntoRespBody,
> where
    Enc: Encoding,
    Request: Req<RequestBody> + Send,
    Response: Res<ResponseBody> + Send,
    RequestBody: Sync,
    ResponseBody: Sync,
    IntoReqBody: Sync,
    IntoRespBody: Sync,
    IntoReq: Req<IntoReqBody> + Send,
    IntoResp: Res<IntoRespBody> + Send,
    Self: Sized,
{
    async fn from_req(req: Request) -> Result<Self, ServerFnError>;
    async fn into_req(self) -> Result<IntoReq, ServerFnError>;

    async fn from_res(res: Response) -> Result<Self, ServerFnError>;
    async fn into_res(self) -> Result<IntoResp, ServerFnError>;
}
