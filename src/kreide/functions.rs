use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub mod rpg {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
	pub mod client {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void,u32) -> *const AvatarData> = lazy_initialize_address!(0x8f12080);
pub static TextmapStatic_GetText: LazyLock<fn(*const TextID,*const NativeArray<NativeObject>) -> *const NativeString> = lazy_initialize_address!(0x942ed70);
pub static GlobalVars_cctor: LazyLock<fn() -> *const NativeObject> = lazy_initialize_address!(0x8611be0);
pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> = lazy_initialize_address!(0x93dff50);
pub static AvatarData_get_AvatarName: LazyLock<fn(*const AvatarData) -> *const NativeString> = lazy_initialize_address!(0x88f5a80);
	}
	pub mod gamecore {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> = lazy_initialize_address!(0x89b6c30);
pub static BattleEventSkillRowData_get_SkillName: LazyLock<fn(*const BattleEventSkillRowData) -> *const TextID> = lazy_initialize_address!(0x881a3c0);
pub static BattleEventSkillRowData_get_AttackType: LazyLock<fn(*const BattleEventSkillRowData) -> AttackType> = lazy_initialize_address!(0x881a340);
pub static SkillCharacterComponent_GetSkillData: LazyLock<fn(*const SkillCharacterComponent,i32,i32) -> *const SkillData> = lazy_initialize_address!(0x954d870);
pub static AbilityStatic_GetActualOwner: LazyLock<fn(*const GameEntity) -> *const GameEntity> = lazy_initialize_address!(0x97ceee0);
pub static TurnBasedAbilityComponent_GetAbilityMappedSkill: LazyLock<fn(*const TurnBasedAbilityComponent,*const NativeString) -> *const NativeString> = lazy_initialize_address!(0x993db60);
pub static AvatarSkillRowData_get_SkillName: LazyLock<fn(*const AvatarSkillRowData) -> *const TextID> = lazy_initialize_address!(0x8999e40);
pub static AvatarSkillRowData_get_AttackType: LazyLock<fn(*const AvatarSkillRowData) -> AttackType> = lazy_initialize_address!(0x8999b10);
pub static CharacterConfig_GetSkillIndexByTriggerKey: LazyLock<fn(*const CharacterConfig,*const NativeString) -> i32> = lazy_initialize_address!(0x59dfe60);
pub static ServantSkillRowData_get_SkillName: LazyLock<fn(*const ServantSkillRowData) -> *const TextID> = lazy_initialize_address!(0x86dc100);
pub static ServantSkillRowData_get_AttackType: LazyLock<fn(*const ServantSkillRowData) -> AttackType> = lazy_initialize_address!(0x86dc030);
pub static EntityManager__GetEntitySummoner: LazyLock<fn(*const EntityManager,*const GameEntity) -> *const GameEntity> = lazy_initialize_address!(0x8cfc400);
	}
}
pub mod unityengine {
use std::{ffi::c_void, sync::LazyLock};
use crate::kreide::native_types::*;
use crate::kreide::gamecore::*;
use crate::kreide::client::*;
pub static Application_set_targetFrameRate: LazyLock<fn(i32)> = lazy_initialize_address!(0x738c660);
}
