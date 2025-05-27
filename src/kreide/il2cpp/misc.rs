use retour::static_detour;

use crate::kreide::types::{MiHoYo_SDK_Win_MiHoYoSDKDll, UnityEngine_Application};

static_detour! {
    static ON_login_will_enter_game: fn(usize, usize);
}

fn on_login_will_enter_game(json_string: usize, callback: usize) {
    ON_login_will_enter_game.call(json_string, callback);
    UnityEngine_Application::set_target_framerate(360).unwrap();
    println!("fps set to 360")
}

pub fn unlock_fps() {
    unsafe {
        ON_login_will_enter_game
            .initialize(
                std::mem::transmute::<usize, fn(usize, usize)>(
                    MiHoYo_SDK_Win_MiHoYoSDKDll::get_class()
                        .unwrap()
                        .find_method_by_name("login_will_enter_game")
                        .unwrap()
                        .va(),
                ),
                on_login_will_enter_game,
            )
            .unwrap();

        ON_login_will_enter_game.enable().unwrap();
    }
}
