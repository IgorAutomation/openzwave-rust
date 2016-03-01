use std::fmt;
use ffi::manager as extern_manager;
use ffi::utils::rust_string_creator;
use std::ffi::CString;

#[derive(Clone, Copy)]
pub struct Node {
    home_id: u32,
    node_id: u8
}

// implements simple node getters
macro_rules! node_getters {
    ( $($impl_name: ident as $name: ident -> $t: ty),+ ) => {
        $(pub fn $name(&self) -> $t {
            let manager_ptr = unsafe { extern_manager::get() };
            unsafe {
                extern_manager::$impl_name(manager_ptr, self.home_id, self.node_id)
            }
        })*
    }
}

macro_rules! node_string_getters {
    ( $($impl_name: ident as $name: ident),+ ) => {
        $(pub fn $name(&self) -> String {
            let manager_ptr = unsafe { extern_manager::get() };
            let result = unsafe {
                CString::from_raw(
                    extern_manager::$impl_name(manager_ptr, self.home_id, self.node_id, rust_string_creator)
                )
            };
            result.into_string().unwrap()
        })*
    }
}

impl Node {
    pub fn from_id(home_id: u32, node_id: u8) -> Node {
        Node { home_id: home_id, node_id: node_id }
    }

    node_getters! {
        is_node_listening_device as is_listening_device -> bool,
        is_node_frequent_listening_device as is_frequent_listening_device -> bool,
        is_node_beaming_device as is_beaming_device -> bool,
        is_node_routing_device as is_routing_device -> bool,
        is_node_security_device as is_security_device -> bool,
        get_node_max_baud_rate as get_max_baud_rate -> u32,
        get_node_version as get_version -> u8,
        get_node_security as get_security -> u8,
        is_node_zwave_plus as is_zwave_plus -> bool,
        get_node_basic as get_basic -> u8,
        get_node_generic as get_generic -> u8,
        get_node_specific as get_specific -> u8
    }

    node_string_getters! {
        get_node_type as get_type,
        get_node_manufacturer_name as get_manufacturer_name,
        get_node_product_name as get_product_name,
        get_node_name as get_name,
        get_node_location as get_location,
        get_node_manufacturer_id as get_manufacturer_id,
        get_node_product_type as get_product_type,
        get_node_product_id as get_product_id,
        get_node_query_stage as get_query_stage,
        get_node_device_type_string as get_device_type_string,
        get_node_role_string as get_role_string,
        get_node_plus_type_string as get_plus_type_string
    }
}

impl fmt::Debug for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Node {{ home_id: {:?}, node_id: {:?}, \
                is_listening_device: {:?}, is_frequent_listening_device: {:?}, \
                is_beaming_device: {:?}, is_routing_device: {:?}, is_security_device: {:?}, \
                max_baud_rate: {:?}, version: {:?}, security: {:?}, is_zwave_plus: {:?}, \
                basic: {:?}, generic: {:?}, specific: {:?}, \
                type: {:?}, manufacturer_name: {:?}, product_name: {:?}, \
                name: {:?}, location: {:?}, manufacturer_id: {:?}, product_type: {:?} \
                product_id: {:?}, query_stage: {:?}, device_type_string: {:?}, \
                role_string: {:?}, plus_type_string: {:?} }}",
               self.home_id,
               self.node_id,
               self.is_listening_device(),
               self.is_frequent_listening_device(),
               self.is_beaming_device(),
               self.is_routing_device(),
               self.is_security_device(),
               self.get_max_baud_rate(),
               self.get_version(),
               self.get_security(),
               self.is_zwave_plus(),
               self.get_basic(),
               self.get_generic(),
               self.get_specific(),
               self.get_type(),
               self.get_manufacturer_name(),
               self.get_product_name(),
               self.get_name(),
               self.get_location(),
               self.get_manufacturer_id(),
               self.get_product_type(),
               self.get_product_id(),
               self.get_query_stage(),
               self.get_device_type_string(),
               self.get_role_string(),
               self.get_plus_type_string()
        )
    }
}
