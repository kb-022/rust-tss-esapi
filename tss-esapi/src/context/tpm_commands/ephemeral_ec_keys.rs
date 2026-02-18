// Copyright 2021 Contributors to the Parsec project.
// SPDX-License-Identifier: Apache-2.0

use std::ptr;
use log::error;

use crate::{Context, ReturnCode};
use crate::handles::KeyHandle;
use crate::structures::{EccParameter, EccPoint, SensitiveData};
use crate::tss2_esys::{Esys_Commit};

impl Context {
    // Missing function: EC_Ephemeral
    pub fn commit(
        &mut self,
        sign_handle: KeyHandle,
        p1: EccPoint,
        s2: Option<SensitiveData>,
        y2: Option<EccParameter>,
    ) -> Result<(EccPoint, EccPoint, EccPoint, u16)>{

        let mut k_ptr = ptr::null_mut();
        let mut l_ptr = ptr::null_mut();
        let mut e_ptr = ptr::null_mut();
        let mut counter:u16 = 0;

        ReturnCode::ensure_success(
            unsafe{
                Esys_Commit(
                    self.mut_context(),
                    sign_handle.into(),
                    self.required_session_1()?,
                    self.optional_session_2(),
                    self.optional_session_3(),
                    &p1.into(),
                    &s2.unwrap_or_default().into(),
                    &y2.unwrap_or_default().into(),
                    &mut k_ptr,
                    &mut l_ptr,
                    &mut e_ptr,
                    &mut counter
                )
            },
            |ret| {
                error!("Error when commiting: {:#010X}", ret);
            },
        )?;
        Ok((
            EccPoint::try_from(Context::ffi_data_to_owned(k_ptr)?)?,
            EccPoint::try_from(Context::ffi_data_to_owned(l_ptr)?)?,
            EccPoint::try_from(Context::ffi_data_to_owned(e_ptr)?)?,
            counter
            ))
    }
}
