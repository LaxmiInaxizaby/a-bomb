#![feature(
    concat_idents,
    proc_macro_hygiene
)]
#![allow(
    non_snake_case,
    unused_macros,
    unused
)]
#![allow(warnings)]
#![deny(
    deprecated
)]
use std::collections::HashMap;
use smash::hash40;
use csk::*;
use param_config::*;
use smash::lib::lua_const::*;


mod abomb;

pub static mut MARKED_COLORS: [bool; 256] = [false; 256];


extern "C" fn mods_mounted(_ev: arcropolis_api::Event) {
    let lowest_color: i32 = 32;
    let color_num: i32 = 2;
    let marked_slots: Vec<i32> = (32..=33).collect();

    unsafe {
        for slot in &marked_slots {
            MARKED_COLORS[*slot as usize] = true;
        }
    }

    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_accel_mul"), 0, 0.168));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_accel_add"), 0, 0.105));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_speed_max"), 0, 1.555));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_slow_speed_mul"), 0, 0.2));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_middle_ratio"), 0, 0.5));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_fast_ratio"), 0, 0.75));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("ground_brake"), 0, 0.125));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("dash_speed"), 0, 2.4));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("run_accel_mul"), 0, 0.156));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("run_accel_add"), 0, 0.044));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("run_speed_max"), 0, 3.0));
    update_int_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_squat_frame"), 0, 3));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_speed_x"), 0, 0.95));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_speed_x_mul"), 0, 0.8));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_speed_x_max"), 0, 1.6));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_aerial_speed_x_mul"), 0, 1.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_initial_y"), 0, 20.7));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_y"), 0, 39.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("mini_jump_y"), 0, 11.1));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_aerial_y"), 0, 30.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_accel_x_mul"), 0, 0.08));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_accel_x_add"), 0, 0.05));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_speed_x_stable"), 0, 1.7));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_brake_x"), 0, 0.035));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_accel_y"), 0, 0.051));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_speed_y_stable"), 0, 1.03));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_brake_y"), 0, 0.035));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("dive_speed_y"), 0, 1.5));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("weight"), 0, 690.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_ground_speed_brake"), 0, 0.029));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("landing_attack_air_frame_n"), 0, 329.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("landing_attack_air_frame_f"), 0, 329.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("landing_attack_air_frame_b"), 0, 329.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("landing_attack_air_frame_hi"), 0, 329.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("landing_attack_air_frame_lw"), 0, 329.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("scale"), 0, 1.6));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("cliff_jump_x_speed"), 0, 0.9));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("cliff_jump_y"), 0, 37.0));
	update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("cliff_jump_y"), 0, 37.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("shield_radius"), 0, 16.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("shield_radius"), 0, 16.0));



    add_narration_characall_entry("vc_narration_characall_abomb");
    add_chara_db_entry_info(CharacterDatabaseEntry{
        ui_chara_id: hash40("ui_chara_abomb"),
        ui_series_id: Hash40Type::Overwrite(smash::hash40("ui_series_abomb")),
        clone_from_ui_chara_id: Some(hash40("ui_chara_purin")),
        name_id: StringType::Overwrite(CStrCSK::new("abomb")),
        fighter_kind: Hash40Type::Overwrite(hash40("fighter_kind_purin")),
        fighter_kind_corps: Hash40Type::Overwrite(hash40("fighter_kind_purin")),
        fighter_type: Hash40Type::Overwrite(0x1353795179),
        alt_chara_id: Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
        shop_item_tag: Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
        disp_order: SignedByteType::Optional(Some(2)),
        chara_count: SignedByteType::Overwrite(1),
        can_select: BoolType::Overwrite(true),
        is_usable_soundtest: BoolType::Overwrite(false),
        is_called_pokemon: BoolType::Overwrite(false),
        is_dlc:BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        color_num: UnsignedByteType::Overwrite(color_num as u8),
        extra_index_maps: UnsignedByteMap::Overwrite(HashMap::from([
            (hash40("color_start_index"), UnsignedByteType::Overwrite(lowest_color as u8)),
            (hash40("original_ui_chara_id"), UnsignedByteType::Overwrite(hash40("ui_chara_purin") as u8))
        ])),
        extra_hash_maps: Hash40Map::Overwrite(HashMap::from([
            (hash40("characall_label_c00"), Hash40Type::Overwrite(hash40("vc_narration_characall_abomb"))),
            (hash40("original_ui_chara_hash"), Hash40Type::Overwrite(hash40("ui_chara_abomb")))
        ])),
        ..Default::default()
    });

	add_series_db_entry_info(SeriesDatabaseEntry {
        ui_series_id: smash::hash40("ui_series_abomb"),
		name_id: StringType::Overwrite(CStrCSK::new("abomb")), 
        disp_order: SignedByteType::Overwrite(0), 
        disp_order_sound: SignedByteType::Overwrite(0), 
        save_no: SignedByteType::Overwrite(0), 
        shown_as_series_in_directory: BoolType::Overwrite(false),
        is_dlc: BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        dlc_chara_id: Hash40Type::Overwrite(0),
        is_use_amiibo_bg: BoolType::Overwrite(false),
        ..std::default::Default::default()
    });
	
    add_chara_layout_db_entry_info(CharacterLayoutDatabaseEntry {
        ui_layout_id: hash40("ui_chara_abomb_00"),
        clone_from_ui_layout_id: Some(hash40("ui_chara_purin_00")),
        ui_chara_id: Hash40Type::Overwrite(hash40("ui_chara_abomb")),
        ..Default::default()
    });
	
	let lowest_color: i32 = 24;
    let color_num: i32 = 8;
    let marked_slots: Vec<i32> = (24..=31).collect();

    unsafe {
        for slot in &marked_slots {
            MARKED_COLORS[*slot as usize] = true;
        }
    }

    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_accel_mul"), 0, 0.168));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_accel_add"), 0, 0.105));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_speed_max"), 0, 1.555));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_slow_speed_mul"), 0, 0.2));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_middle_ratio"), 0, 0.5));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("walk_fast_ratio"), 0, 0.75));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("ground_brake"), 0, 0.125));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("dash_speed"), 0, 2.4));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("run_accel_mul"), 0, 0.156));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("run_accel_add"), 0, 0.044));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("run_speed_max"), 0, 3.0));
    update_int_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_squat_frame"), 0, 3));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_speed_x"), 0, 0.95));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_speed_x_mul"), 0, 0.8));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_speed_x_max"), 0, 1.6));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_aerial_speed_x_mul"), 0, 1.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_initial_y"), 0, 20.7));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_y"), 0, 39.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("mini_jump_y"), 0, 11.1));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("jump_aerial_y"), 0, 30.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_accel_x_mul"), 0, 0.08));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_accel_x_add"), 0, 0.05));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_speed_x_stable"), 0, 1.7));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_brake_x"), 0, 0.035));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_accel_y"), 0, 0.051));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_speed_y_stable"), 0, 1.03));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_brake_y"), 0, 0.035));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("dive_speed_y"), 0, 1.5));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("weight"), 0, 109.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("air_ground_speed_brake"), 0, 0.029));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("landing_attack_air_frame_n"), 0, 999.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("landing_attack_air_frame_f"), 0, 999.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("landing_attack_air_frame_b"), 0, 999.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("landing_attack_air_frame_hi"), 0, 999.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("landing_attack_air_frame_lw"), 0, 999.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("scale"), 0, 1.3));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("cliff_jump_x_speed"), 0, 0.9));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("cliff_jump_y"), 0, 37.0));
	update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("cliff_jump_y"), 0, 37.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("shield_radius"), 0, 16.0));
    update_float_2(*FIGHTER_KIND_PURIN, marked_slots.clone(), (hash40("shield_radius"), 0, 16.0));



    add_narration_characall_entry("vc_narration_characall_bomb");
    add_chara_db_entry_info(CharacterDatabaseEntry{
        ui_chara_id: hash40("ui_chara_bomb"),
        ui_series_id: Hash40Type::Overwrite(smash::hash40("ui_series_bomb")),
        clone_from_ui_chara_id: Some(hash40("ui_chara_purin")),
        name_id: StringType::Overwrite(CStrCSK::new("bomb")),
        fighter_kind: Hash40Type::Overwrite(hash40("fighter_kind_purin")),
        fighter_kind_corps: Hash40Type::Overwrite(hash40("fighter_kind_purin")),
        fighter_type: Hash40Type::Overwrite(0x1353795179),
        alt_chara_id: Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
        shop_item_tag: Hash40Type::Overwrite(0x2302D482A /* Hash40 of -1 */), 
        disp_order: SignedByteType::Optional(Some(2)),
        chara_count: SignedByteType::Overwrite(1),
        can_select: BoolType::Overwrite(true),
        is_usable_soundtest: BoolType::Overwrite(false),
        is_called_pokemon: BoolType::Overwrite(false),
        is_dlc:BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        color_num: UnsignedByteType::Overwrite(color_num as u8),
        extra_index_maps: UnsignedByteMap::Overwrite(HashMap::from([
            (hash40("color_start_index"), UnsignedByteType::Overwrite(lowest_color as u8)),
            (hash40("original_ui_chara_id"), UnsignedByteType::Overwrite(hash40("ui_chara_purin") as u8))
        ])),
        extra_hash_maps: Hash40Map::Overwrite(HashMap::from([
            (hash40("characall_label_c00"), Hash40Type::Overwrite(hash40("vc_narration_characall_bomb"))),
            (hash40("original_ui_chara_hash"), Hash40Type::Overwrite(hash40("ui_chara_bomb")))
        ])),
        ..Default::default()
    });

	add_series_db_entry_info(SeriesDatabaseEntry {
        ui_series_id: smash::hash40("ui_series_bomb"),
		name_id: StringType::Overwrite(CStrCSK::new("bomb")), 
        disp_order: SignedByteType::Overwrite(0), 
        disp_order_sound: SignedByteType::Overwrite(0), 
        save_no: SignedByteType::Overwrite(0), 
        shown_as_series_in_directory: BoolType::Overwrite(false),
        is_dlc: BoolType::Overwrite(false),
        is_patch: BoolType::Overwrite(false),
        dlc_chara_id: Hash40Type::Overwrite(0),
        is_use_amiibo_bg: BoolType::Overwrite(false),
        ..std::default::Default::default()
    });
	
    add_chara_layout_db_entry_info(CharacterLayoutDatabaseEntry {
        ui_layout_id: hash40("ui_chara_bomb_00"),
        clone_from_ui_layout_id: Some(hash40("ui_chara_purin_00")),
        ui_chara_id: Hash40Type::Overwrite(hash40("ui_chara_bomb")),
        //eye_0_flash_count: ::UnsignedByteType::Optional(Some(2)), 
        //eye_0_flash0_pos_x: FloatType::Optional(Some(-117.0)), 
        //eye_0_flash0_pos_y: FloatType::Optional(Some(-15.0)), 
        //eye_0_flash1_pos_x: FloatType::Optional(Some(180.0)), 
        //eye_0_flash1_pos_y: FloatType::Optional(Some(146.0)), 
        //eye_0_flash2_pos_x: FloatType::Optional(Some(0.0)), 
        //eye_0_flash2_pos_y: FloatType::Optional(Some(0.0)), 
        //eye_0_flash3_pos_x: FloatType::Optional(Some(0.0)), 
        //eye_0_flash3_pos_y: FloatType::Optional(Some(0.0)), 
        //eye_0_flash4_pos_x: FloatType::Optional(Some(0.0)), 
        //eye_0_flash4_pos_y: FloatType::Optional(Some(0.0)), 
        //eye_1_flash_count: ::UnsignedByteType::Optional(Some(2)), 
        //eye_1_flash0_pos_x: FloatType::Optional(Some(-108.0)), 
        //eye_1_flash0_pos_y: FloatType::Optional(Some(74.0)), 
        //eye_1_flash1_pos_x: FloatType::Optional(Some(110.0)), 
        //eye_1_flash1_pos_y: FloatType::Optional(Some(192.0)), 
        //eye_1_flash2_pos_x: FloatType::Optional(Some(0.0)), 
        //eye_1_flash2_pos_y: FloatType::Optional(Some(0.0)), 
        //eye_1_flash3_pos_x: FloatType::Optional(Some(0.0)), 
        //eye_1_flash3_pos_y: FloatType::Optional(Some(0.0)), 
        //eye_1_flash4_pos_x: FloatType::Optional(Some(0.0)), 
        //eye_1_flash4_pos_y: FloatType::Optional(Some(0.0)), 
        //eye_2_flash_count: ::UnsignedByteType::Optional(Some(2)), 
        //eye_2_flash0_pos_x: FloatType::Optional(Some(-50.0)), 
        //eye_2_flash0_pos_y: FloatType::Optional(Some(-12.0)), 
        //eye_2_flash1_pos_x: FloatType::Optional(Some(80.0)), 
        //eye_2_flash1_pos_y: FloatType::Optional(Some(59.0)), 
        //eye_2_flash2_pos_x: FloatType::Optional(Some(0.0)), 
        //eye_2_flash2_pos_y: FloatType::Optional(Some(0.0)), 
        //eye_2_flash3_pos_x: FloatType::Optional(Some(0.0)), 
        //eye_2_flash3_pos_y: FloatType::Optional(Some(0.0)), 
        //eye_2_flash4_pos_x: FloatType::Optional(Some(0.0)), 
        //eye_2_flash4_pos_y: FloatType::Optional(Some(0.0)), 
        //eye_flash_info_pos_x: FloatType::Optional(Some(39.0)), 
        //eye_flash_info_pos_y: FloatType::Optional(Some(9.0)), 
        //spirits_eye_visible: ::BoolType::Optional(Some(true)), 
        //chara_1_offset_x: FloatType::Optional(Some(-3.0)), 
        //chara_1_offset_y: FloatType::Optional(Some(-55.0)), 
        //chara_1_scale: FloatType::Optional(Some(1.11)), 
        //chara_1_1_offset_x: FloatType::Optional(Some(3.0)), 
        //chara_1_1_offset_y: FloatType::Optional(Some(-56.0)), 
        //chara_1_1_scale: FloatType::Optional(Some(1.44)), 
        //chara_1_2_offset_x: FloatType::Optional(Some(0.0)), 
        //chara_1_2_offset_y: FloatType::Optional(Some(0.0)), 
        //chara_1_2_scale: FloatType::Optional(Some(1.0)), 
        //chara_1_3_offset_x: FloatType::Optional(Some(3.0)), 
        //chara_1_3_offset_y: FloatType::Optional(Some(-37.0)), 
        //chara_1_3_scale: FloatType::Optional(Some(1.44)), 
        //chara_1_4_offset_x: FloatType::Optional(Some(3.0)), 
        //chara_1_4_offset_y: FloatType::Optional(Some(-37.0)), 
        //chara_1_4_scale: FloatType::Optional(Some(1.44)), 
        //chara_1_5_offset_x: FloatType::Optional(Some(0.0)), 
        //chara_1_5_offset_y: FloatType::Optional(Some(0.0)), 
        //chara_1_5_scale: FloatType::Optional(Some(1.0)), 
        //chara_3_0_offset_x: FloatType::Optional(Some(331.0)), 
        //chara_3_0_offset_y: FloatType::Optional(Some(-285.0)), 
        //chara_3_0_scale: FloatType::Optional(Some(1.0)), 
        //chara_3_1_offset_x: FloatType::Optional(Some(352.0)), 
        //chara_3_1_offset_y: FloatType::Optional(Some(-252.0)), 
        //chara_3_1_scale: FloatType::Optional(Some(0.96)), 
        //chara_3_2_offset_x: FloatType::Optional(Some(80.0)), 
        //chara_3_2_offset_y: FloatType::Optional(Some(-100.0)), 
        //chara_3_2_scale: FloatType::Optional(Some(0.76)), 
        //chara_3_3_offset_x: FloatType::Optional(Some(550.0)), 
        //chara_3_3_offset_y: FloatType::Optional(Some(-150.0)), 
        //chara_3_3_scale: FloatType::Optional(Some(0.84)), 
        //chara_3_4_offset_x: FloatType::Optional(Some(405.0)), 
        //chara_3_4_offset_y: FloatType::Optional(Some(-177.0)), 
        //chara_3_4_scale: FloatType::Optional(Some(0.87)), 
        //chara_3_5_offset_x: FloatType::Optional(Some(354.0)), 
        //chara_3_5_offset_y: FloatType::Optional(Some(-245.0)), 
        //chara_3_5_scale: FloatType::Optional(Some(0.92)), 
        //chara_3_6_offset_x: FloatType::Optional(Some(0.0)), 
        //chara_3_6_offset_y: FloatType::Optional(Some(0.0)), 
        //chara_3_6_scale: FloatType::Optional(Some(1.0)), 
        //chara_3_7_offset_x: FloatType::Optional(Some(331.0)), 
        //chara_3_7_offset_y: FloatType::Optional(Some(-285.0)),
        //chara_3_7_scale: FloatType::Optional(Some(1.0)), 
        //chara_5_offset_x: FloatType::Optional(Some(0.0)), 
        //chara_5_offset_y: FloatType::Optional(Some(0.0)), 
        //chara_5_scale: FloatType::Optional(Some(1.0)), 
        //chara_select_icon_list_offset_x: FloatType::Optional(Some(0.0)), 
        //chara_select_icon_list_offset_y: FloatType::Optional(Some(0.0)), 
        //chara_select_icon_list_scale: FloatType::Optional(Some(1.0)), 
        //chara_7_0_offset_x: FloatType::Optional(Some(-4.0)), 
        //chara_7_0_offset_y: FloatType::Optional(Some(3.0)), 
        //chara_7_0_scale: FloatType::Optional(Some(0.95)), 
        //chara_7_1_offset_x: FloatType::Optional(Some(-4.0)), 
        //chara_7_1_offset_y: FloatType::Optional(Some(3.0)), 
        //chara_7_1_scale: FloatType::Optional(Some(0.95)), 
        ..Default::default()
    });
}



#[skyline::main(name = "smashline_test")]
pub fn main() {
    unsafe {
        extern "C" {
            fn arcrop_register_event_callback(
                ty: arcropolis_api::Event,
                callback: arcropolis_api::EventCallbackFn,
            );
        }
        arcrop_register_event_callback(arcropolis_api::Event::ModFilesystemMounted, mods_mounted);
    }
    abomb::install();
}