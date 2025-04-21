use crate::kreide::client::*;
use crate::kreide::gamecore::*;
use crate::kreide::native_types::*;
use std::{ffi::c_void, sync::LazyLock};
pub mod rpg {
    use crate::kreide::client::*;
    use crate::kreide::gamecore::*;
    use crate::kreide::native_types::*;
    use std::{ffi::c_void, sync::LazyLock};
    pub mod client {
        use crate::kreide::client::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::native_types::*;
        use std::{ffi::c_void, sync::LazyLock};
        pub static AvatarData_get_AvatarName: LazyLock<
            fn(*const AvatarData) -> *const NativeString,
        > = lazy_initialize_address!(0x88aca90);
        pub static UIGameEntityUtils_GetAvatarID: LazyLock<fn(*const GameEntity) -> u32> =
            lazy_initialize_address!(0x93ddcb0);
        pub static AvatarModule_GetAvatar: LazyLock<fn(*const c_void, u32) -> *const AvatarData> =
            lazy_initialize_address!(0x8f10f20);
        pub static TextmapStatic_GetText: LazyLock<
            fn(*const TextID, *const NativeArray<NativeObject>) -> *const NativeString,
        > = lazy_initialize_address!(0x942cae0);
    }
    pub mod gamecore {
        use crate::kreide::client::*;
        use crate::kreide::gamecore::*;
        use crate::kreide::native_types::*;
        use std::{ffi::c_void, sync::LazyLock};
        pub static CharacterConfig_GetSkillIndexByTriggerKey: LazyLock<
            fn(*const CharacterConfig, *const NativeString) -> i32,
        > = lazy_initialize_address!(0x59dee60);
        pub static EntityManager__GetEntitySummoner: LazyLock<
            fn(*const EntityManager, *const GameEntity) -> *const GameEntity,
        > = lazy_initialize_address!(0x8cfb090);
        pub static AbilityStatic_GetActualOwner: LazyLock<
            fn(*const GameEntity) -> *const GameEntity,
        > = lazy_initialize_address!(0x97cd040);
        pub static ServantSkillRowData_get_SkillName: LazyLock<
            fn(*const ServantSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x86db4f0);
        pub static ServantSkillRowData_get_AttackType: LazyLock<
            fn(*const ServantSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x86db420);
        pub static AvatarSkillRowData_get_SkillName: LazyLock<
            fn(*const AvatarSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x89986c0);
        pub static AvatarSkillRowData_get_AttackType: LazyLock<
            fn(*const AvatarSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x8998390);
        pub static BattleEventSkillRowData_get_SkillName: LazyLock<
            fn(*const BattleEventSkillRowData) -> *const TextID,
        > = lazy_initialize_address!(0x8818b60);
        pub static BattleEventSkillRowData_get_AttackType: LazyLock<
            fn(*const BattleEventSkillRowData) -> AttackType,
        > = lazy_initialize_address!(0x8818ae0);
        pub static SkillCharacterComponent_GetSkillData: LazyLock<
            fn(*const SkillCharacterComponent, i32, i32) -> *const SkillData,
        > = lazy_initialize_address!(0x954b750);
        pub static GamePlayStatic_GetEntityManager: LazyLock<fn() -> *const EntityManager> =
            lazy_initialize_address!(0x89b54b0);
        pub static TurnBasedAbilityComponent_GetAbilityMappedSkill: LazyLock<
            fn(*const TurnBasedAbilityComponent, *const NativeString) -> *const NativeString,
        > = lazy_initialize_address!(0x993bcf0);
    }
}
