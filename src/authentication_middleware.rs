use {
    crate::{entities::prelude::ApiKey, error::ToErr, AppData},
    actix_session::Session,
    actix_web::{
        body::MessageBody,
        dev::{
            forward_ready, Extensions, Payload, Service, ServiceRequest, ServiceResponse, Transform,
        },
        error::{ErrorBadRequest, ErrorForbidden, ErrorUnauthorized},
        http::header::HeaderValue,
        Error, FromRequest, HttpMessage, HttpRequest,
    },
    base64::{prelude::BASE64_STANDARD, Engine},
    sea_orm::EntityTrait,
    serde::{Deserialize, Serialize},
    serde_inline_default::serde_inline_default,
    std::{
        borrow::Cow,
        cell::{Ref, RefCell},
        future::{ready, Future, Ready},
        pin::Pin,
        rc::Rc,
    },
};

pub struct AuthenticationMiddleware;

impl<S, B> Transform<S, ServiceRequest> for AuthenticationMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = InnerAuthenticationMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(InnerAuthenticationMiddleware {
            service: Rc::new(service),
        }))
    }
}

pub struct InnerAuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for InnerAuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = self.service.clone();

        Box::pin(async move {
            async fn get_authentication(
                header: &HeaderValue,
            ) -> Result<InnerAuthentication, Error> {
                let header = header.as_bytes();
                let bytes = if header.starts_with(b"Basic ") {
                    Cow::Owned(
                        BASE64_STANDARD
                            .decode(&header[6..])
                            .map_err(|_| ErrorBadRequest("invalid authorization header"))?,
                    )
                } else {
                    Cow::Borrowed(header)
                };
                let pos = bytes
                    .iter()
                    .position(|&b| b == b':')
                    .ok_or(ErrorBadRequest("invalid authorization header"))?;

                let mut id: u32 = 0;
                for &byte in &bytes[..pos] {
                    if byte.is_ascii_digit() {
                        id *= 10;
                        id += (byte - b'0') as u32;
                    } else {
                        return Err(ErrorBadRequest("invalid authorization header"));
                    }
                }
                let data = std::str::from_utf8(&bytes[pos + 1..])
                    .map_err(|_| ErrorBadRequest("invalid authorization header"))?;

                let key = ApiKey::find_by_id(id)
                    .one(&AppData::get().conn)
                    .await
                    .to_err()?
                    .ok_or(ErrorForbidden("invalid api key"))?;

                key.verify(data).await?;

                Ok(InnerAuthentication {
                    id: key.owner,
                    name: None,
                    is_session: false,
                })
            }

            let session = Session::from_request(req.request(), &mut Payload::None).await?;
            let auth = if let Some(auth) = session.get::<InnerAuthentication>("user")? {
                Some(auth)
            } else if let Some(header) = req.headers().get("Authorization") {
                Some(get_authentication(header).await?)
            } else {
                None
            };

            let authentication = Authentication::set_authentication(&req, auth);

            let res = service.call(req).await?;

            if let (true, auth) = &*authentication.borrow() {
                if let Some(auth) = auth {
                    session.insert("user", auth)?;
                } else {
                    session.remove("user");
                }
            }

            Ok(res)
        })
    }
}

type AuthenticationType = Rc<RefCell<(bool, Option<InnerAuthentication>)>>;
pub struct Authentication(AuthenticationType);

impl Authentication {
    fn get_authentication(extensions: &mut Extensions) -> Self {
        match extensions.get::<AuthenticationType>() {
            Some(a_impl) => Authentication(a_impl.clone()),
            None => {
                let inner = Rc::new(RefCell::new((false, None)));
                extensions.insert(inner.clone());
                Authentication(inner)
            }
        }
    }

    fn set_authentication(req: &ServiceRequest, value: Option<InnerAuthentication>) -> Self {
        let _self = Self::get_authentication(&mut req.extensions_mut());
        *_self.0.borrow_mut() = (false, value);
        _self
    }

    fn borrow(&self) -> Ref<'_, (bool, Option<InnerAuthentication>)> {
        self.0.borrow()
    }

    #[allow(dead_code)]
    pub fn is_some(&self) -> bool {
        self.0.borrow().1.is_some()
    }

    #[allow(dead_code)]
    pub fn is_none(&self) -> bool {
        self.0.borrow().1.is_none()
    }

    #[allow(dead_code)]
    pub fn take(&self) -> Result<InnerAuthentication, Error> {
        self.0
            .borrow_mut()
            .1
            .take()
            .ok_or(ErrorUnauthorized("Not logged in"))
    }

    #[allow(dead_code)]
    pub fn take_session(&self) -> Result<InnerAuthentication, Error> {
        let auth = self.take()?;
        if !auth.is_session {
            return Err(ErrorForbidden("session required"));
        }
        Ok(auth)
    }

    #[allow(dead_code)]
    pub fn id(&self) -> Result<u32, Error> {
        if let Some(auth) = &self.0.borrow().1 {
            Ok(auth.id)
        } else {
            Err(ErrorUnauthorized("Not logged in"))
        }
    }

    #[allow(dead_code)]
    pub fn name(&self) -> Result<Option<String>, Error> {
        if let Some(auth) = &self.0.borrow().1 {
            Ok(auth.name.clone())
        } else {
            Err(ErrorUnauthorized("Not logged in"))
        }
    }

    #[allow(dead_code)]
    pub fn set(&self, id: u32, name: String) {
        *self.0.borrow_mut() = (
            true,
            Some(InnerAuthentication {
                id,
                name: Some(name),
                is_session: false,
            }),
        );
    }

    #[allow(dead_code)]
    pub fn unset(&self) {
        *self.0.borrow_mut() = (true, None);
    }
}

impl FromRequest for Authentication {
    type Error = Error;
    type Future = Ready<Result<Authentication, Error>>;

    #[inline]
    fn from_request(req: &HttpRequest, _: &mut Payload) -> Self::Future {
        ready(Ok(Authentication::get_authentication(
            &mut req.extensions_mut(),
        )))
    }
}

#[serde_inline_default]
#[derive(Serialize, Deserialize)]
pub struct InnerAuthentication {
    pub id: u32,
    pub name: Option<String>,
    #[serde_inline_default(true)]
    #[serde(skip)]
    is_session: bool,
}
