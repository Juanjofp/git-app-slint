use std::{rc::Rc, sync::Arc};

use file_downloader::UreqDownloader as Downloader;
use git_info::GitInfoAnonymousUReq as GitInfo;
use slint::{ComponentHandle, Image, Model, SharedString, VecModel, Weak};

use super::{AppWindow, UIUser};

pub struct UCFindUser;

impl UCFindUser {
    pub fn register(ui: Weak<AppWindow>, downloader: Arc<Downloader>, git_info: Arc<GitInfo>) {
        let ui = ui.unwrap();

        let users_model = Rc::new(VecModel::from(vec![]));

        ui.set_users(users_model.clone().into());

        ui.on_find_user({
            let git_info_clone = Arc::clone(&git_info);
            let downloader_clone = Arc::clone(&downloader);
            let ui_clone = ui.as_weak();

            move |user_name: SharedString| {
                ui_clone.unwrap().set_is_loading(true);

                let ui_for_thread = ui_clone.clone();
                let git_info_clone = Arc::clone(&git_info_clone);
                let downloader_clone = Arc::clone(&downloader_clone);
                std::thread::spawn(move || {
                    let user = git_info_clone.user(user_name.as_str());

                    if let Ok(user) = user {
                        let avatar_url = downloader_clone.download(&user.user.avatar).unwrap();

                        let avatar_url = avatar_url.file;

                        let name = user.name.map_or("Jhon Doe".to_string(), |name| name);

                        let nickname = user.user.login.clone();

                        slint::invoke_from_event_loop(move || {
                            let user = UIUser {
                                nickname: SharedString::from(nickname),
                                name: SharedString::from(name),
                                avatar_url: Image::load_from_path(&avatar_url).unwrap(),

                                repo_count: SharedString::from(
                                    user.repo_info.public_repos.to_string(),
                                ),
                                follower_count: SharedString::from(
                                    user.repo_info.followers.to_string(),
                                ),
                                following_count: SharedString::from(
                                    user.repo_info.following.to_string(),
                                ),
                                collaborator_count: SharedString::from(
                                    user.repo_info.collaborators.to_string(),
                                ),
                                gist_count: SharedString::from(
                                    user.repo_info.public_gists.to_string(),
                                ),
                            };

                            let ui = ui_for_thread.unwrap();

                            ui.get_users()
                                .as_any()
                                .downcast_ref::<VecModel<UIUser>>()
                                .expect("Should be VecModel<UIUser>")
                                .push(user);

                            ui.set_is_loading(false);
                        })
                        .unwrap();
                    } else {
                        slint::invoke_from_event_loop(move || {
                            let ui = ui_for_thread.unwrap();

                            ui.set_is_loading(false);
                        })
                        .unwrap();
                    }
                });
            }
        });
    }
}
