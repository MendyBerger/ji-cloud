use web_sys::Url;
use wasm_bindgen::prelude::*;
use shared::domain::{
    image::{ImageId, ImageSearchQuery}, 
    jig::{JigId, ModuleId, ModuleKind}, 
    search::CreateSearchKeyResponse, 
    session::CreateSessionOAuthResponse
};
use crate::firebase::FirebaseUserInfo;
use serde::{Serialize, Deserialize};
use std::str::FromStr;
use uuid::Uuid;

pub type Id = String;

#[derive(Debug, Clone)]
pub enum Route {
    NotFound,
    NoAuth,
    User(UserRoute),
    Admin(AdminRoute),
    Jig(JigRoute),
    Legacy(LegacyRoute),
    Module(ModuleRoute),
	Dev(DevRoute),
}

#[derive(Debug, Clone)]
pub enum UserRoute {
    Profile(ProfileSection),
    RegisterOauth(OauthData),
    LoginOauth(OauthData),
    Login,
    Register,
    ContinueRegistration,
    SendEmailConfirmation,
    RegisterComplete,
}

#[derive(Debug, Clone)]
pub enum ProfileSection {
    Landing,
    ChangeEmail
}

#[derive(Debug, Clone)]
pub enum AdminRoute {
    Landing,
    Categories,
    Locale,
    ImageSearch(Option<ImageSearchQuery>),
    ImageAdd,
    ImageMeta(ImageId, bool), //flag is for if it's a new image
}

#[derive(Debug, Clone)]
pub enum LegacyRoute {
    Play(Id, Option<Id>) 
}

#[derive(Debug, Clone)]
pub enum JigRoute {
    Gallery,
    Edit(JigId, Option<ModuleId>),
    Play(JigId, Option<ModuleId>) 
}

#[derive(Debug, Clone)]
pub enum JigPlayMode {
    Producer,
    Audience 
}

#[derive(Debug, Clone)]
pub enum ModuleRoute {
    Edit(ModuleKind, Id, Id),
    Play(ModuleKind, Id, Id),
}

#[derive(Debug, Clone)]
pub enum DevRoute {
    Showcase(Id, String),
    Scratch(Id, String),
}

//Just for serializing across local routes
#[derive(Serialize, Deserialize)]
struct JsonQuery {
    pub data: String  //json-encoded data as-needed
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum OauthData {
    Google(OauthCode)
}
pub type OauthCode = String;

impl Route {
    pub fn redirect(self) {
        let location = web_sys::window().unwrap_throw().location();
        let s:String = self.into();
        location.set_href(&s).unwrap_throw();
    }

    pub fn replace_state(self) {
        let history = web_sys::window().unwrap_throw().history().unwrap_throw();
        let url:String = self.into();
        history.replace_state_with_url(&JsValue::NULL, "", Some(&url));
    }

	pub fn to_string(&self) -> String {
		self.into()
	}
	
    pub fn from_url(url:&str) -> Self {
        let url = Url::new(&url).unwrap_throw();
        let paths = url.pathname();
        let paths = paths.split("/").into_iter().skip(1).collect::<Vec<_>>();
        let paths = paths.as_slice();
        let params = url.search_params();
        let json_query = params.get("data");

        match paths {
			["dev", "showcase", id] => {
                let page = params.get("page").unwrap_or_default();
                Self::Dev(DevRoute::Showcase(id.to_string(), page))
            }
			["dev", "scratch", id] => {
                let page = params.get("page").unwrap_or_default();
                Self::Dev(DevRoute::Scratch(id.to_string(), page))
            }
            ["user", "profile"] => Self::User(UserRoute::Profile(ProfileSection::Landing)),
            ["user", "profile", "change-email"] => Self::User(UserRoute::Profile(ProfileSection::ChangeEmail)),
            ["user", "login"] => Self::User(UserRoute::Login),
            ["user", "register"] => Self::User(UserRoute::Register),

            ["user", "register-oauth"] => {
                if let Some(code) = params.get("code") {
                    let data = OauthData::Google(code);
                    Self::User(UserRoute::RegisterOauth(data))
                } else {
                    Self::NoAuth
                }
            }
            ["user", "login-oauth"] => {
                if let Some(code) = params.get("code") {
                    let data = OauthData::Google(code);
                    Self::User(UserRoute::LoginOauth(data))
                } else {
                    Self::NoAuth
                }
            }
            ["user", "continue-registration"] => Self::User(UserRoute::ContinueRegistration),
            ["user", "send-email-confirmation"] => Self::User(UserRoute::SendEmailConfirmation),
            ["user", "register-complete"] => Self::User(UserRoute::RegisterComplete),
            ["admin", "locale"] => Self::Admin(AdminRoute::Locale),
            ["admin", "categories"] => Self::Admin(AdminRoute::Categories),
            ["admin", "image-search"] => {
                if let Some(search) = json_query {
                    let search:ImageSearchQuery = serde_json::from_str(&search).unwrap_throw();
                    Self::Admin(AdminRoute::ImageSearch(Some(search)))
                } else {
                    Self::Admin(AdminRoute::ImageSearch(None))
                }
            },
            ["admin", "image-add"] => Self::Admin(AdminRoute::ImageAdd),
            ["admin", "image-meta", id, flag] => {
                let id = ImageId(Uuid::from_str(id).unwrap_throw());
                Self::Admin(AdminRoute::ImageMeta(id, bool::from_str(flag).unwrap_throw()))
            },
            ["admin"] => Self::Admin(AdminRoute::Landing),
            ["jig", "gallery"] => Self::Jig(JigRoute::Gallery),
            ["jig", "edit", "debug"] => Self::Jig(JigRoute::Edit(
                    JigId(Uuid::from_u128(0)),
                    None
            )),
            ["jig", "edit", jig_id] => Self::Jig(JigRoute::Edit(
                    JigId(Uuid::from_str(jig_id).unwrap_throw()),
                    None
            )),
            ["jig", "edit", jig_id, module_id] => Self::Jig(JigRoute::Edit(
                    JigId(Uuid::from_str(jig_id).unwrap_throw()),
                    Some(ModuleId(Uuid::from_str(module_id).unwrap_throw()))
            )),
            ["jig", "play", jig_id] => Self::Jig(JigRoute::Play(
                    JigId(Uuid::from_str(jig_id).unwrap_throw()),
                    None
            )),
            ["jig", "play", jig_id, module_id] => Self::Jig(JigRoute::Play(
                    JigId(Uuid::from_str(jig_id).unwrap_throw()),
                    Some(ModuleId(Uuid::from_str(module_id).unwrap_throw()))
            )),
                    
            ["legacy", "play", jig_id] => Self::Legacy(LegacyRoute::Play(jig_id.to_string(), None)),
            ["legacy", "play", jig_id, module_id] => Self::Legacy(LegacyRoute::Play(jig_id.to_string(), Some(module_id.to_string()))),
            ["module", kind, "edit", jig_id, module_id] => Self::Module(ModuleRoute::Edit(ModuleKind::from_str(kind).expect_throw("unknown module kind!"), jig_id.to_string(), module_id.to_string())),
            ["module", kind, "play", jig_id, module_id] => Self::Module(ModuleRoute::Play(ModuleKind::from_str(kind).expect_throw("unknown module kind!"), jig_id.to_string(), module_id.to_string())),
            ["no-auth"] => Self::NoAuth,

            _ => Self::NotFound
        }
    }
}

impl From<Route> for String {
	fn from(route:Route) -> Self {
		(&route).into()
	}
}

impl From<&Route> for String {
    fn from(route:&Route) -> Self {
        match route {
            Route::NoAuth => "/no-auth".to_string(),
			Route::Dev(route) => {
                match route {
                    DevRoute::Showcase(id, page) => format!("/dev/showcase/{}?page={}", id, page),
                    DevRoute::Scratch(id, page) => format!("/dev/scratch/{}?page={}", id, page)
				}
			},
            Route::User(route) => {
                match route {
                    UserRoute::Profile(ProfileSection::Landing) => "/user/profile".to_string(),
                    UserRoute::Profile(ProfileSection::ChangeEmail) => "/user/profile/change-email".to_string(),
                    UserRoute::ContinueRegistration => "/user/continue-registration".to_string(),
                    UserRoute::Login => "/user/login".to_string(),
                    UserRoute::Register => "/user/register".to_string(),
                    UserRoute::RegisterOauth(_) => "/user/register-oauth".to_string(),
                    UserRoute::LoginOauth(_) => "/user/login-oauth".to_string(),
                    UserRoute::SendEmailConfirmation => "/user/send-email-confirmation".to_string(),
                    UserRoute::RegisterComplete => "/user/register-complete".to_string(),
                }
            },
            Route::Admin(route) => {
                match route {
                    AdminRoute::Landing => "/admin".to_string(),
                    AdminRoute::Locale => "/admin/locale".to_string(),
                    AdminRoute::Categories => "/admin/categories".to_string(),
                    AdminRoute::ImageSearch(search) => {
                        match search {
                            None => "/admin/image-search".to_string(),
                            Some(search) => {
                                let data = serde_json::to_string(&search).unwrap_throw();
                                let query = JsonQuery { data };
                                let query = serde_qs::to_string(&query).unwrap_throw();
                                format!("/admin/image-search?{}", query)
                            }
                        }
                    }
                    AdminRoute::ImageAdd => "/admin/image-add".to_string(),
                    AdminRoute::ImageMeta(id, is_new) => format!("/admin/image-meta/{}/{}", id.0.to_string(), is_new),
                }
            },
            Route::Jig(route) => {
                match route {
                    JigRoute::Gallery => "/jig/gallery".to_string(),
                    JigRoute::Edit(jig_id, module_id) => {
                        if let Some(module_id) = module_id {
                            format!("/jig/edit/{}/{}", jig_id.0.to_string(), module_id.0.to_string())
                        } else {
                            format!("/jig/edit/{}", jig_id.0.to_string())
                        }
                    }
                    JigRoute::Play(jig_id, module_id) => {
                        if let Some(module_id) = module_id {
                            format!("/jig/play/{}/{}", jig_id.0.to_string(), module_id.0.to_string())
                        } else {
                            format!("/jig/play/{}", jig_id.0.to_string())
                        }
                    }
                }
            },
            Route::Legacy(route) => {
                match route {
                    LegacyRoute::Play(jig_id, module_id) => {
                        if let Some(module_id) = module_id {
                            format!("/legacy/play/{}/{}", jig_id, module_id)
                        } else {
                            format!("/legacy/play/{}", jig_id)
                        }
                    }
                }
            },
            Route::Module(route) => {
                match route {
                    ModuleRoute::Edit(kind, jig_id, module_id) => format!("/module/{}/edit/{}/{}", kind.as_str(), jig_id, module_id),
                    ModuleRoute::Play(kind, jig_id, module_id) => format!("/module/{}/play/{}/{}", kind.as_str(), jig_id, module_id),
                }
            },
            Route::NotFound => "/404".to_string(),
        }
    }
}

