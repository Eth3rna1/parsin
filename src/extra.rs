/*
    Extra functions that don't necessarily fit in with the rest
    of some function in a module, belong here
*/
use crate::errors::Error;
use crate::errors::ErrorKind;

use std::cmp::PartialEq;

pub(crate) fn find_and_remove<T: PartialEq>(
    _collection: &mut Vec<T>,
    item: &T,
) -> Result<T, Error> {
    let index = {
        let mut bind: usize = 0;
        let mut exists = false;
        for (i, ele) in _collection.iter().enumerate() {
            if ele == item {
                exists = true;
                bind = i;
            }
        }
        if !exists {
            return Err(Error::new(
                ErrorKind::Other,
                "Element does not exist within the collection given.".to_string(),
            ));
        }
        bind
    };
    Ok(_collection.remove(index))
}
