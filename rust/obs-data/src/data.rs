//! Safe core of `libobs/obs-data.c`.
//!
//! The method bodies are `todo!()` until the port commit. Tier 1 calls them
//! and panics.

use std::path::Path;

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Type {
    Null = 0,
    String = 1,
    Number = 2,
    Boolean = 3,
    Object = 4,
    Array = 5,
}

#[repr(i32)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NumType {
    Invalid = 0,
    Int = 1,
    Double = 2,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Vec4 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Quat {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fps {
    pub numerator: u32,
    pub denominator: u32,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FpsRead {
    pub ok: bool,
    pub fps: Option<Fps>,
    pub option: Option<String>,
}

#[derive(Clone)]
pub struct Item;

impl Item {
    pub fn name(&self) -> String {
        todo!("obs_data_item_get_name")
    }
    pub fn ty(&self) -> Type {
        todo!("obs_data_item_gettype")
    }
    pub fn numtype(&self) -> NumType {
        todo!("obs_data_item_numtype")
    }
    pub fn has_user_value(&self) -> bool {
        todo!("obs_data_item_has_user_value")
    }
    pub fn get_string(&self) -> String {
        todo!("obs_data_item_get_string")
    }
    pub fn get_autoselect_int(&self) -> i64 {
        todo!("obs_data_item_get_autoselect_int")
    }
    pub fn next(self) -> Option<Item> {
        todo!("obs_data_item_next")
    }
    pub fn remove(&self) {
        todo!("obs_data_item_remove")
    }
}

#[derive(Clone)]
pub struct Data;

impl Data {
    pub fn new() -> Self {
        todo!("obs_data_create")
    }
    pub fn from_json(text: &str) -> Option<Self> {
        let _ = text;
        todo!("obs_data_create_from_json")
    }
    pub fn from_json_file(path: &Path) -> Option<Self> {
        let _ = path;
        todo!("obs_data_create_from_json_file")
    }
    pub fn from_json_file_safe(path: &Path, backup_ext: &str) -> Option<Self> {
        let _ = (path, backup_ext);
        todo!("obs_data_create_from_json_file_safe")
    }
    pub fn defaults_from(src: Option<&Data>) -> Data {
        let _ = src;
        todo!("obs_data_get_defaults")
    }
    pub fn json(&self) -> String {
        todo!("obs_data_get_json")
    }
    pub fn json_with_defaults(&self) -> String {
        todo!("obs_data_get_json_with_defaults")
    }
    pub fn json_pretty(&self) -> String {
        todo!("obs_data_get_json_pretty")
    }
    pub fn last_json(&self) -> Option<String> {
        todo!("obs_data_get_last_json")
    }
    pub fn save_json(&self, path: &Path) -> bool {
        let _ = path;
        todo!("obs_data_save_json")
    }
    pub fn save_json_safe(&self, path: &Path, temp_ext: &str, backup_ext: &str) -> bool {
        let _ = (path, temp_ext, backup_ext);
        todo!("obs_data_save_json_safe")
    }
    pub fn set_string(&self, name: &str, val: Option<&str>) {
        let _ = (name, val);
        todo!("obs_data_set_string")
    }
    pub fn set_int(&self, name: &str, val: i64) {
        let _ = (name, val);
        todo!("obs_data_set_int")
    }
    pub fn set_double(&self, name: &str, val: f64) {
        let _ = (name, val);
        todo!("obs_data_set_double")
    }
    pub fn set_bool(&self, name: &str, val: bool) {
        let _ = (name, val);
        todo!("obs_data_set_bool")
    }
    pub fn set_obj(&self, name: &str, obj: Option<&Data>) {
        let _ = (name, obj);
        todo!("obs_data_set_obj")
    }
    pub fn set_array(&self, name: &str, array: Option<&Array>) {
        let _ = (name, array);
        todo!("obs_data_set_array")
    }
    pub fn set_default_string(&self, name: &str, val: Option<&str>) {
        let _ = (name, val);
        todo!("obs_data_set_default_string")
    }
    pub fn set_default_int(&self, name: &str, val: i64) {
        let _ = (name, val);
        todo!("obs_data_set_default_int")
    }
    pub fn set_autoselect_string(&self, name: &str, val: Option<&str>) {
        let _ = (name, val);
        todo!("obs_data_set_autoselect_string")
    }
    pub fn set_autoselect_int(&self, name: &str, val: i64) {
        let _ = (name, val);
        todo!("obs_data_set_autoselect_int")
    }
    pub fn get_string(&self, name: &str) -> String {
        let _ = name;
        todo!("obs_data_get_string")
    }
    pub fn get_int(&self, name: &str) -> i64 {
        let _ = name;
        todo!("obs_data_get_int")
    }
    pub fn get_double(&self, name: &str) -> f64 {
        let _ = name;
        todo!("obs_data_get_double")
    }
    pub fn get_bool(&self, name: &str) -> bool {
        let _ = name;
        todo!("obs_data_get_bool")
    }
    pub fn get_obj(&self, name: &str) -> Option<Data> {
        let _ = name;
        todo!("obs_data_get_obj")
    }
    pub fn get_array(&self, name: &str) -> Option<Array> {
        let _ = name;
        todo!("obs_data_get_array")
    }
    pub fn get_default_string(&self, name: &str) -> String {
        let _ = name;
        todo!("obs_data_get_default_string")
    }
    pub fn get_default_int(&self, name: &str) -> i64 {
        let _ = name;
        todo!("obs_data_get_default_int")
    }
    pub fn get_autoselect_string(&self, name: &str) -> String {
        let _ = name;
        todo!("obs_data_get_autoselect_string")
    }
    pub fn has_user_value(&self, name: &str) -> bool {
        let _ = name;
        todo!("obs_data_has_user_value")
    }
    pub fn has_default_value(&self, name: &str) -> bool {
        let _ = name;
        todo!("obs_data_has_default_value")
    }
    pub fn has_autoselect_value(&self, name: &str) -> bool {
        let _ = name;
        todo!("obs_data_has_autoselect_value")
    }
    pub fn unset_user_value(&self, name: &str) {
        let _ = name;
        todo!("obs_data_unset_user_value")
    }
    pub fn apply(&self, src: &Data) {
        let _ = src;
        todo!("obs_data_apply")
    }
    pub fn erase(&self, name: &str) {
        let _ = name;
        todo!("obs_data_erase")
    }
    pub fn clear(&self) {
        todo!("obs_data_clear")
    }
    pub fn first(&self) -> Option<Item> {
        todo!("obs_data_first")
    }
    pub fn item(&self, name: &str) -> Option<Item> {
        let _ = name;
        todo!("obs_data_item_byname")
    }
    pub fn set_vec2(&self, name: &str, val: Vec2) {
        let _ = (name, val);
        todo!("obs_data_set_vec2")
    }
    pub fn get_vec2(&self, name: &str) -> Option<Vec2> {
        let _ = name;
        todo!("obs_data_get_vec2")
    }
    pub fn set_vec3(&self, name: &str, val: Vec3) {
        let _ = (name, val);
        todo!("obs_data_set_vec3")
    }
    pub fn get_vec3(&self, name: &str) -> Option<Vec3> {
        let _ = name;
        todo!("obs_data_get_vec3")
    }
    pub fn set_fps(&self, name: &str, fps: Fps, option: Option<&str>) {
        let _ = (name, fps, option);
        todo!("obs_data_set_frames_per_second")
    }
    pub fn get_fps(&self, name: &str, read_option: bool) -> FpsRead {
        let _ = (name, read_option);
        todo!("obs_data_get_frames_per_second")
    }
}

#[derive(Clone)]
pub struct Array;

impl Array {
    pub fn new() -> Self {
        todo!("obs_data_array_create")
    }
    pub fn count(&self) -> usize {
        todo!("obs_data_array_count")
    }
    pub fn count_of(array: Option<&Array>) -> usize {
        let _ = array;
        todo!("obs_data_array_count")
    }
    pub fn item(&self, idx: usize) -> Option<Data> {
        let _ = idx;
        todo!("obs_data_array_item")
    }
    pub fn push(&self, obj: Option<&Data>) -> usize {
        let _ = obj;
        todo!("obs_data_array_push_back")
    }
    pub fn push_of(array: Option<&Array>, obj: Option<&Data>) -> usize {
        let _ = (array, obj);
        todo!("obs_data_array_push_back")
    }
    pub fn insert(&self, idx: usize, obj: Option<&Data>) {
        let _ = (idx, obj);
        todo!("obs_data_array_insert")
    }
    pub fn push_array(&self, other: &Array) {
        let _ = other;
        todo!("obs_data_array_push_back_array")
    }
    pub fn erase(&self, idx: usize) {
        let _ = idx;
        todo!("obs_data_array_erase")
    }
}
