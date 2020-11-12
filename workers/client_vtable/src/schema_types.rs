use spatialos_sdk::sys_exports::schema::ComponentData;
use spatialos_sdk::sys_exports::schema::ComponentUpdate;
use spatialos_sdk::sys_exports::worker::ComponentId;
use spatialos_sdk::Component;
use std::os::raw::c_void;

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Position {
    pub coords: Coordinates,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct PositionUpdate {
    pub coords: *mut Coordinates,
}

#[repr(C)]
#[derive(Debug, Clone)]
pub struct Coordinates {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Component for Position {
    type Update = PositionUpdate;

    const ID: ComponentId = 54;

    fn component_data_deserialize(
        _: ComponentId,
        _: *mut c_void,
        mut source: ComponentData,
    ) -> Self {
        let mut fields = source.get_fields();
        let mut object = fields.get_object(1);
        let x = object.get_double(1);
        let y = object.get_double(2);
        let z = object.get_double(3);
        Position {
            coords: Coordinates { x, y, z },
        }
    }
    fn component_data_serialize(
        _: ComponentId,
        _: *mut c_void,
        data: &mut Position,
    ) -> ComponentData {
        let mut component_data = ComponentData::new();
        let mut fields = component_data.get_fields();
        let mut coords = fields.add_object(1);
        coords.add_double(1, data.coords.x);
        coords.add_double(2, data.coords.y);
        coords.add_double(3, data.coords.z);
        component_data
    }

    fn component_update_deserialize(
        _: ComponentId,
        _: *mut c_void,
        mut source: ComponentUpdate,
    ) -> Self::Update {
        let mut fields = source.get_fields();
        PositionUpdate {
            coords: if fields.get_object_count(1) == 1 {
                let mut coords = fields.get_object(1);
                let x = coords.get_double(1);
                let y = coords.get_double(2);
                let z = coords.get_double(3);
                let boxed_coords = Box::new(Coordinates { x, y, z });
                Box::into_raw(boxed_coords)
            } else {
                std::ptr::null_mut()
            },
        }
    }

    fn component_update_serialize(
        _: ComponentId,
        _: *mut c_void,
        data: &mut Self::Update,
    ) -> ComponentUpdate {
        let mut new_update = ComponentUpdate::new();
        let mut fields = new_update.get_fields();
        if !data.coords.is_null() {
            let coords = unsafe { Box::from_raw(data.coords) };
            let mut object = fields.add_object(1);
            object.add_double(1, coords.x);
            object.add_double(2, coords.y);
            object.add_double(3, coords.z);
            Box::into_raw(coords);
        }
        new_update
    }

    fn component_update_free(_: ComponentId, _: *mut c_void, data: Self::Update) {
        if !data.coords.is_null() {
            unsafe {
                Box::from_raw(data.coords);
            }
        }
    }
    fn component_update_copy(_: ComponentId, _: *mut c_void, data: &Self::Update) -> Self::Update {
        let mut new_data = data.clone();
        if !data.coords.is_null() {
            let coords = unsafe { Box::from_raw(data.coords) };
            new_data.coords = Box::into_raw(coords.clone());
            Box::into_raw(coords);
        }
        new_data
    }
}
