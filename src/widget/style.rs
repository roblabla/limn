use graphics::types::Color;
use linked_hash_map::LinkedHashMap;
use widget::{Property, PropSet};
use std::collections::BTreeSet;
use std::hash::Hash;

#[derive(Clone, Debug)]
pub enum Value<T> where T: Clone {
    Single(T),
    Selector((LinkedHashMap<PropSet, T>, T)),
}

impl<T> Value<T> where T: Clone {
    pub fn from_props(&self, props: &PropSet) -> T {
        match *self {
            Value::Selector::<T>((ref sel, ref def)) => {
                if sel.contains_key(&props) {
                    return sel.get(&props).unwrap().clone()
                } else {
                    for (style_props, style_val) in sel.iter() {
                        // props matches all in style props
                        if style_props.is_subset(&props) {
                            return style_val.clone();
                        }
                    }
                }
            }, _ => ()
        }
        self.default()
    }
    pub fn default(&self) -> T {
        match *self {
            Value::Single::<T>(ref val) => {
                val.clone()
            }
            Value::Selector::<T>((ref sel, ref def)) => {
                def.clone()
            }
        }
    }
}