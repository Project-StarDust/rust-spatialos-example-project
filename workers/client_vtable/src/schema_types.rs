use spatialos_sdk::sys_exports::schema::ComponentData;
use spatialos_sdk::sys_exports::schema::ComponentUpdate;
use spatialos_sdk::sys_exports::worker::ComponentId;
use spatialos_sdk::{Component, Type};
use std::os::raw::c_void;

#[allow(dead_code)]
#[derive(SpatialComponent)]
#[id(1001)]
pub struct ClientData {
    #[field_id(1)]
    input_state: f32,
}

pub struct Position;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PositionData {
    pub coords: CoordinatesData,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PositionUpdate {
    pub coords: *mut CoordinatesUpdate,
}

pub struct Coordinates;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CoordinatesData {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct CoordinatesUpdate {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Type for Coordinates {
    type Data = CoordinatesData;

    type Update = CoordinatesUpdate;

    fn type_data_deserialize(
        _: *mut c_void,
        source: &mut spatialos_sdk::sys_exports::schema::Object,
    ) -> Self::Data {
        let x = source.get_double(1);
        let y = source.get_double(2);
        let z = source.get_double(3);
        Self::Data { x, y, z }
    }

    fn type_data_serialize(
        _: *mut c_void,
        data: &mut Self::Data,
        target: &mut spatialos_sdk::sys_exports::schema::Object,
    ) {
        target.add_double(1, data.x);
        target.add_double(2, data.y);
        target.add_double(3, data.z);
    }

    fn type_update_deserialize(
        _: *mut c_void,
        source: &mut spatialos_sdk::sys_exports::schema::Object,
    ) -> Self::Update {
        let x = source.get_double(1);
        let y = source.get_double(2);
        let z = source.get_double(3);
        Self::Update { x, y, z }
    }

    fn type_update_serialize(
        user_data: *mut c_void,
        data: &mut Self::Update,
        target: &mut spatialos_sdk::sys_exports::schema::Object,
    ) {
        target.add_double(1, data.x);
        target.add_double(2, data.y);
        target.add_double(3, data.z);
    }

    fn type_update_free(user_data: *mut c_void, data: Self::Update) {}

    fn type_update_copy(user_data: *mut c_void, data: &Self::Update) -> Self::Update {
        data.clone()
    }
}

impl Component for Position {
    type Update = PositionUpdate;
    type Data = PositionData;

    const ID: ComponentId = 54;

    fn component_data_deserialize(
        _: ComponentId,
        user_data: *mut c_void,
        mut source: ComponentData,
    ) -> Self::Data {
        let mut fields = source.get_fields();
        let coords = Coordinates::type_data_deserialize(user_data, &mut fields.get_object(1));
        Self::Data { coords }
    }
    fn component_data_serialize(
        _: ComponentId,
        user_data: *mut c_void,
        data: &mut Self::Data,
    ) -> ComponentData {
        let mut component_data = ComponentData::new();
        let mut fields = component_data.get_fields();
        Coordinates::type_data_serialize(user_data, &mut data.coords, &mut fields.add_object(1));
        component_data
    }

    fn component_update_deserialize(
        _: ComponentId,
        user_data: *mut c_void,
        mut source: ComponentUpdate,
    ) -> Self::Update {
        let mut fields = source.get_fields();
        let coords = if fields.get_object_count(1) == 1 {
            Box::into_raw(Box::new(Coordinates::type_update_deserialize(
                user_data,
                &mut fields.get_object(1),
            )))
        } else {
            std::ptr::null_mut()
        };
        Self::Update { coords }
    }

    fn component_update_serialize(
        _: ComponentId,
        user_data: *mut c_void,
        data: &mut Self::Update,
    ) -> ComponentUpdate {
        let mut new_update = ComponentUpdate::new();
        let mut fields = new_update.get_fields();
        if !data.coords.is_null() {
            let mut coords = unsafe { Box::from_raw(data.coords) };
            Coordinates::type_update_serialize(user_data, &mut *coords, &mut fields.add_object(1));
            Box::into_raw(coords);
        }
        new_update
    }

    fn component_update_free(_: ComponentId, user_data: *mut c_void, data: Self::Update) {
        if !data.coords.is_null() {
            let coords = unsafe { Box::from_raw(data.coords) };
            Coordinates::type_update_free(user_data, *coords);
        }
    }

    fn component_update_copy(
        _: ComponentId,
        user_data: *mut c_void,
        data: &Self::Update,
    ) -> Self::Update {
        let mut new_data = data.clone();
        new_data.coords = if !data.coords.is_null() {
            let coords = unsafe { Box::from_raw(data.coords) };
            let new_coords = Coordinates::type_update_copy(user_data, &*coords);
            Box::into_raw(coords);
            Box::into_raw(Box::new(new_coords))
        } else {
            std::ptr::null_mut()
        };
        new_data
    }
}
