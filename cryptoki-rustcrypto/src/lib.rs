// Copyright 2023 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use cryptoki::{
    error::Result,
    mechanism::Mechanism,
    object::{Attribute, AttributeType, ObjectHandle},
    session::Session,
};

pub mod ecdsa;
pub mod rsa;
pub mod x509;

pub trait SessionLike {
    fn create_object(&self, template: &[Attribute]) -> Result<ObjectHandle>;
    fn find_objects(&self, template: &[Attribute]) -> Result<Vec<ObjectHandle>>;
    fn get_attributes(
        &self,
        object: ObjectHandle,
        attributes: &[AttributeType],
    ) -> Result<Vec<Attribute>>;
    fn sign(&self, mechanism: &Mechanism, key: ObjectHandle, data: &[u8]) -> Result<Vec<u8>>;
}

impl SessionLike for Session {
    fn create_object(&self, template: &[Attribute]) -> Result<ObjectHandle> {
        Session::create_object(self, template)
    }
    fn find_objects(&self, template: &[Attribute]) -> Result<Vec<ObjectHandle>> {
        Session::find_objects(self, template)
    }
    fn get_attributes(
        &self,
        object: ObjectHandle,
        attributes: &[AttributeType],
    ) -> Result<Vec<Attribute>> {
        Session::get_attributes(self, object, attributes)
    }
    fn sign(&self, mechanism: &Mechanism, key: ObjectHandle, data: &[u8]) -> Result<Vec<u8>> {
        Session::sign(self, mechanism, key, data)
    }
}

impl<'s> SessionLike for &'s Session {
    fn create_object(&self, template: &[Attribute]) -> Result<ObjectHandle> {
        Session::create_object(self, template)
    }
    fn find_objects(&self, template: &[Attribute]) -> Result<Vec<ObjectHandle>> {
        Session::find_objects(self, template)
    }
    fn get_attributes(
        &self,
        object: ObjectHandle,
        attributes: &[AttributeType],
    ) -> Result<Vec<Attribute>> {
        Session::get_attributes(self, object, attributes)
    }
    fn sign(&self, mechanism: &Mechanism, key: ObjectHandle, data: &[u8]) -> Result<Vec<u8>> {
        Session::sign(self, mechanism, key, data)
    }
}
