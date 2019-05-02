use libc;
extern "C" {
    /* Allocate a new pointer list */
    #[no_mangle]
    fn clist_new() -> *mut clist;
    /* Destroys a list. Data pointed by data pointers is NOT freed. */
    #[no_mangle]
    fn clist_free(_: *mut clist);
    /* Inserts this data pointer after the element pointed by the iterator */
    #[no_mangle]
    fn clist_insert_after(_: *mut clist, _: *mut clistiter, _: *mut libc::c_void) -> libc::c_int;
    /* Deletes the element pointed by the iterator.
    Returns an iterator to the next element. */
    #[no_mangle]
    fn clist_delete(_: *mut clist, _: *mut clistiter) -> *mut clistiter;
    #[no_mangle]
    fn clist_foreach(lst: *mut clist, func: clist_func, data: *mut libc::c_void);
    #[no_mangle]
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    #[no_mangle]
    fn mailmime_composite_type_new(
        ct_type: libc::c_int,
        ct_token: *mut libc::c_char,
    ) -> *mut mailmime_composite_type;
    #[no_mangle]
    fn mailmime_composite_type_free(ct: *mut mailmime_composite_type);
    #[no_mangle]
    fn mailmime_content_new(
        ct_type: *mut mailmime_type,
        ct_subtype: *mut libc::c_char,
        ct_parameters: *mut clist,
    ) -> *mut mailmime_content;
    #[no_mangle]
    fn mailmime_content_free(content: *mut mailmime_content);
    #[no_mangle]
    fn mailmime_discrete_type_new(
        dt_type: libc::c_int,
        dt_extension: *mut libc::c_char,
    ) -> *mut mailmime_discrete_type;
    #[no_mangle]
    fn mailmime_discrete_type_free(discrete_type: *mut mailmime_discrete_type);
    #[no_mangle]
    fn mailmime_encoding_free(encoding: *mut mailmime_mechanism);
    #[no_mangle]
    fn mailmime_mechanism_new(
        enc_type: libc::c_int,
        enc_token: *mut libc::c_char,
    ) -> *mut mailmime_mechanism;
    #[no_mangle]
    fn mailmime_mechanism_free(mechanism: *mut mailmime_mechanism);
    #[no_mangle]
    fn mailmime_parameter_new(
        pa_name: *mut libc::c_char,
        pa_value: *mut libc::c_char,
    ) -> *mut mailmime_parameter;
    #[no_mangle]
    fn mailmime_parameter_free(parameter: *mut mailmime_parameter);
    #[no_mangle]
    fn mailmime_type_new(
        tp_type: libc::c_int,
        tp_discrete_type: *mut mailmime_discrete_type,
        tp_composite_type: *mut mailmime_composite_type,
    ) -> *mut mailmime_type;
    #[no_mangle]
    fn mailmime_type_free(type_0: *mut mailmime_type);
    /*
    void mailmime_x_token_free(gchar * x_token);
    */
    #[no_mangle]
    fn mailmime_field_new(
        fld_type: libc::c_int,
        fld_content: *mut mailmime_content,
        fld_encoding: *mut mailmime_mechanism,
        fld_id: *mut libc::c_char,
        fld_description: *mut libc::c_char,
        fld_version: uint32_t,
        fld_disposition: *mut mailmime_disposition,
        fld_language: *mut mailmime_language,
        fld_location: *mut libc::c_char,
    ) -> *mut mailmime_field;
    #[no_mangle]
    fn mailmime_field_free(field: *mut mailmime_field);
    #[no_mangle]
    fn mailmime_fields_new(fld_list: *mut clist) -> *mut mailmime_fields;
    #[no_mangle]
    fn mailmime_fields_free(fields: *mut mailmime_fields);
    #[no_mangle]
    fn mailmime_data_new(
        dt_type: libc::c_int,
        dt_encoding: libc::c_int,
        dt_encoded: libc::c_int,
        dt_data: *const libc::c_char,
        dt_length: size_t,
        dt_filename: *mut libc::c_char,
    ) -> *mut mailmime_data;
    #[no_mangle]
    fn mailmime_new(
        mm_type: libc::c_int,
        mm_mime_start: *const libc::c_char,
        mm_length: size_t,
        mm_mime_fields: *mut mailmime_fields,
        mm_content_type: *mut mailmime_content,
        mm_body: *mut mailmime_data,
        mm_preamble: *mut mailmime_data,
        mm_epilogue: *mut mailmime_data,
        mm_mp_list: *mut clist,
        mm_fields: *mut mailimf_fields,
        mm_msg_mime: *mut mailmime,
    ) -> *mut mailmime;
    #[no_mangle]
    fn mailmime_free(mime: *mut mailmime);
    #[no_mangle]
    fn mailmime_disposition_new(
        dsp_type: *mut mailmime_disposition_type,
        dsp_parms: *mut clist,
    ) -> *mut mailmime_disposition;
    #[no_mangle]
    fn mailmime_disposition_free(dsp: *mut mailmime_disposition);
    #[no_mangle]
    fn mailmime_disposition_type_new(
        dt_type: libc::c_int,
        dt_extension: *mut libc::c_char,
    ) -> *mut mailmime_disposition_type;
    #[no_mangle]
    fn mailmime_disposition_type_free(dsp_type: *mut mailmime_disposition_type);
    #[no_mangle]
    fn mailmime_disposition_parm_new(
        pa_type: libc::c_int,
        pa_filename: *mut libc::c_char,
        pa_creation_date: *mut libc::c_char,
        pa_modification_date: *mut libc::c_char,
        pa_read_date: *mut libc::c_char,
        pa_size: size_t,
        pa_parameter: *mut mailmime_parameter,
    ) -> *mut mailmime_disposition_parm;
    #[no_mangle]
    fn mailmime_disposition_parm_free(dsp_parm: *mut mailmime_disposition_parm);
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn getpid() -> pid_t;
    #[no_mangle]
    fn gethostname(_: *mut libc::c_char, _: size_t) -> libc::c_int;
    #[no_mangle]
    fn random() -> libc::c_long;
    #[no_mangle]
    fn time(_: *mut time_t) -> time_t;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    /*
     * libEtPan! -- a mail stuff library
     *
     * Copyright (C) 2001, 2005 - DINH Viet Hoa
     * All rights reserved.
     *
     * Redistribution and use in source and binary forms, with or without
     * modification, are permitted provided that the following conditions
     * are met:
     * 1. Redistributions of source code must retain the above copyright
     *    notice, this list of conditions and the following disclaimer.
     * 2. Redistributions in binary form must reproduce the above copyright
     *    notice, this list of conditions and the following disclaimer in the
     *    documentation and/or other materials provided with the distribution.
     * 3. Neither the name of the libEtPan! project nor the names of its
     *    contributors may be used to endorse or promote products derived
     *    from this software without specific prior written permission.
     *
     * THIS SOFTWARE IS PROVIDED BY THE AUTHORS AND CONTRIBUTORS ``AS IS'' AND
     * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
     * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
     * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS OR CONTRIBUTORS BE LIABLE
     * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
     * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
     * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
     * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
     * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
     * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
     * SUCH DAMAGE.
     */
    /*
     * $Id: mailmime.h,v 1.18 2011/01/06 00:09:52 hoa Exp $
     */
    #[no_mangle]
    fn mailmime_content_parse(
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
        result: *mut *mut mailmime_content,
    ) -> libc::c_int;
    #[no_mangle]
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    #[no_mangle]
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
}
pub type __int32_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type __darwin_time_t = libc::c_long;
pub type __darwin_pid_t = __int32_t;
pub type pid_t = __darwin_pid_t;
pub type size_t = __darwin_size_t;
pub type time_t = __darwin_time_t;
pub type uint32_t = libc::c_uint;
/*
 * libEtPan! -- a mail stuff library
 *
 * clist - Implements simple generic double-linked pointer lists
 *
 * Copyright (c) 1999-2005, Ga�l Roualland <gael.roualland@iname.com>
 * interface changes - 2005 - DINH Viet Hoa
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the libEtPan! project nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHORS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/*
 * $Id: clist.h,v 1.13 2011/05/09 21:49:46 hoa Exp $
 */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clistcell_s {
    pub data: *mut libc::c_void,
    pub previous: *mut clistcell_s,
    pub next: *mut clistcell_s,
}
pub type clistcell = clistcell_s;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct clist_s {
    pub first: *mut clistcell,
    pub last: *mut clistcell,
    pub count: libc::c_int,
}
pub type clist = clist_s;
pub type clistiter = clistcell;
pub type clist_func =
    Option<unsafe extern "C" fn(_: *mut libc::c_void, _: *mut libc::c_void) -> ()>;
/*
  mailimf_fields is a list of header fields

  - fld_list is a list of header fields
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_fields {
    pub fld_list: *mut clist,
}
/* these are the possible returned error codes */
pub type unnamed = libc::c_uint;
pub const MAILIMF_ERROR_FILE: unnamed = 4;
pub const MAILIMF_ERROR_INVAL: unnamed = 3;
pub const MAILIMF_ERROR_MEMORY: unnamed = 2;
pub const MAILIMF_ERROR_PARSE: unnamed = 1;
pub const MAILIMF_NO_ERROR: unnamed = 0;
/*
 * libEtPan! -- a mail stuff library
 *
 * Copyright (C) 2001, 2005 - DINH Viet Hoa
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the libEtPan! project nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHORS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/*
 * $Id: mailmime_types.h,v 1.33 2011/01/06 00:09:52 hoa Exp $
 */
pub type unnamed_0 = libc::c_uint;
pub const MAILMIME_COMPOSITE_TYPE_EXTENSION: unnamed_0 = 3;
pub const MAILMIME_COMPOSITE_TYPE_MULTIPART: unnamed_0 = 2;
pub const MAILMIME_COMPOSITE_TYPE_MESSAGE: unnamed_0 = 1;
pub const MAILMIME_COMPOSITE_TYPE_ERROR: unnamed_0 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_composite_type {
    pub ct_type: libc::c_int,
    pub ct_token: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_content {
    pub ct_type: *mut mailmime_type,
    pub ct_subtype: *mut libc::c_char,
    pub ct_parameters: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_type {
    pub tp_type: libc::c_int,
    pub tp_data: unnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_1 {
    pub tp_discrete_type: *mut mailmime_discrete_type,
    pub tp_composite_type: *mut mailmime_composite_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_discrete_type {
    pub dt_type: libc::c_int,
    pub dt_extension: *mut libc::c_char,
}
pub type unnamed_2 = libc::c_uint;
pub const MAILMIME_DISCRETE_TYPE_EXTENSION: unnamed_2 = 6;
pub const MAILMIME_DISCRETE_TYPE_APPLICATION: unnamed_2 = 5;
pub const MAILMIME_DISCRETE_TYPE_VIDEO: unnamed_2 = 4;
pub const MAILMIME_DISCRETE_TYPE_AUDIO: unnamed_2 = 3;
pub const MAILMIME_DISCRETE_TYPE_IMAGE: unnamed_2 = 2;
pub const MAILMIME_DISCRETE_TYPE_TEXT: unnamed_2 = 1;
pub const MAILMIME_DISCRETE_TYPE_ERROR: unnamed_2 = 0;
pub type unnamed_3 = libc::c_uint;
pub const MAILMIME_FIELD_LOCATION: unnamed_3 = 8;
pub const MAILMIME_FIELD_LANGUAGE: unnamed_3 = 7;
pub const MAILMIME_FIELD_DISPOSITION: unnamed_3 = 6;
pub const MAILMIME_FIELD_VERSION: unnamed_3 = 5;
pub const MAILMIME_FIELD_DESCRIPTION: unnamed_3 = 4;
pub const MAILMIME_FIELD_ID: unnamed_3 = 3;
pub const MAILMIME_FIELD_TRANSFER_ENCODING: unnamed_3 = 2;
pub const MAILMIME_FIELD_TYPE: unnamed_3 = 1;
pub const MAILMIME_FIELD_NONE: unnamed_3 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_field {
    pub fld_type: libc::c_int,
    pub fld_data: unnamed_4,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_4 {
    pub fld_content: *mut mailmime_content,
    pub fld_encoding: *mut mailmime_mechanism,
    pub fld_id: *mut libc::c_char,
    pub fld_description: *mut libc::c_char,
    pub fld_version: uint32_t,
    pub fld_disposition: *mut mailmime_disposition,
    pub fld_language: *mut mailmime_language,
    pub fld_location: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_language {
    pub lg_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_disposition {
    pub dsp_type: *mut mailmime_disposition_type,
    pub dsp_parms: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_disposition_type {
    pub dsp_type: libc::c_int,
    pub dsp_extension: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_mechanism {
    pub enc_type: libc::c_int,
    pub enc_token: *mut libc::c_char,
}
pub type unnamed_5 = libc::c_uint;
pub const MAILMIME_MECHANISM_TOKEN: unnamed_5 = 6;
pub const MAILMIME_MECHANISM_BASE64: unnamed_5 = 5;
pub const MAILMIME_MECHANISM_QUOTED_PRINTABLE: unnamed_5 = 4;
pub const MAILMIME_MECHANISM_BINARY: unnamed_5 = 3;
pub const MAILMIME_MECHANISM_8BIT: unnamed_5 = 2;
pub const MAILMIME_MECHANISM_7BIT: unnamed_5 = 1;
pub const MAILMIME_MECHANISM_ERROR: unnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_fields {
    pub fld_list: *mut clist,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_parameter {
    pub pa_name: *mut libc::c_char,
    pub pa_value: *mut libc::c_char,
}
pub type unnamed_6 = libc::c_uint;
pub const MAILMIME_TYPE_COMPOSITE_TYPE: unnamed_6 = 2;
pub const MAILMIME_TYPE_DISCRETE_TYPE: unnamed_6 = 1;
pub const MAILMIME_TYPE_ERROR: unnamed_6 = 0;
pub type unnamed_7 = libc::c_uint;
pub const MAILMIME_DATA_FILE: unnamed_7 = 1;
pub const MAILMIME_DATA_TEXT: unnamed_7 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_data {
    pub dt_type: libc::c_int,
    pub dt_encoding: libc::c_int,
    pub dt_encoded: libc::c_int,
    pub dt_data: unnamed_8,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_8 {
    pub dt_text: unnamed_9,
    pub dt_filename: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_9 {
    pub dt_data: *const libc::c_char,
    pub dt_length: size_t,
}
pub type unnamed_10 = libc::c_uint;
pub const MAILMIME_MESSAGE: unnamed_10 = 3;
pub const MAILMIME_MULTIPLE: unnamed_10 = 2;
pub const MAILMIME_SINGLE: unnamed_10 = 1;
pub const MAILMIME_NONE: unnamed_10 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime {
    pub mm_parent_type: libc::c_int,
    pub mm_parent: *mut mailmime,
    pub mm_multipart_pos: *mut clistiter,
    pub mm_type: libc::c_int,
    pub mm_mime_start: *const libc::c_char,
    pub mm_length: size_t,
    pub mm_mime_fields: *mut mailmime_fields,
    pub mm_content_type: *mut mailmime_content,
    pub mm_body: *mut mailmime_data,
    pub mm_data: unnamed_11,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_11 {
    pub mm_single: *mut mailmime_data,
    pub mm_multipart: unnamed_13,
    pub mm_message: unnamed_12,
}
/* message */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_12 {
    pub mm_fields: *mut mailimf_fields,
    pub mm_msg_mime: *mut mailmime,
}
/* multi-part */
#[derive(Copy, Clone)]
#[repr(C)]
pub struct unnamed_13 {
    pub mm_preamble: *mut mailmime_data,
    pub mm_epilogue: *mut mailmime_data,
    pub mm_mp_list: *mut clist,
}
pub type unnamed_14 = libc::c_uint;
pub const MAILMIME_DISPOSITION_PARM_PARAMETER: unnamed_14 = 5;
pub const MAILMIME_DISPOSITION_PARM_SIZE: unnamed_14 = 4;
pub const MAILMIME_DISPOSITION_PARM_READ_DATE: unnamed_14 = 3;
pub const MAILMIME_DISPOSITION_PARM_MODIFICATION_DATE: unnamed_14 = 2;
pub const MAILMIME_DISPOSITION_PARM_CREATION_DATE: unnamed_14 = 1;
pub const MAILMIME_DISPOSITION_PARM_FILENAME: unnamed_14 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_disposition_parm {
    pub pa_type: libc::c_int,
    pub pa_data: unnamed_15,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_15 {
    pub pa_filename: *mut libc::c_char,
    pub pa_creation_date: *mut libc::c_char,
    pub pa_modification_date: *mut libc::c_char,
    pub pa_read_date: *mut libc::c_char,
    pub pa_size: size_t,
    pub pa_parameter: *mut mailmime_parameter,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_single_fields {
    pub fld_content: *mut mailmime_content,
    pub fld_content_charset: *mut libc::c_char,
    pub fld_content_boundary: *mut libc::c_char,
    pub fld_content_name: *mut libc::c_char,
    pub fld_encoding: *mut mailmime_mechanism,
    pub fld_id: *mut libc::c_char,
    pub fld_description: *mut libc::c_char,
    pub fld_version: uint32_t,
    pub fld_disposition: *mut mailmime_disposition,
    pub fld_disposition_filename: *mut libc::c_char,
    pub fld_disposition_creation_date: *mut libc::c_char,
    pub fld_disposition_modification_date: *mut libc::c_char,
    pub fld_disposition_read_date: *mut libc::c_char,
    pub fld_disposition_size: size_t,
    pub fld_language: *mut mailmime_language,
    pub fld_location: *mut libc::c_char,
}
/*
 * libEtPan! -- a mail stuff library
 *
 * Copyright (C) 2001, 2005 - DINH Viet Hoa
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions
 * are met:
 * 1. Redistributions of source code must retain the above copyright
 *    notice, this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright
 *    notice, this list of conditions and the following disclaimer in the
 *    documentation and/or other materials provided with the distribution.
 * 3. Neither the name of the libEtPan! project nor the names of its
 *    contributors may be used to endorse or promote products derived
 *    from this software without specific prior written permission.
 *
 * THIS SOFTWARE IS PROVIDED BY THE AUTHORS AND CONTRIBUTORS ``AS IS'' AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED.  IN NO EVENT SHALL THE AUTHORS OR CONTRIBUTORS BE LIABLE
 * FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL
 * DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS
 * OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION)
 * HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT
 * LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY
 * OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF
 * SUCH DAMAGE.
 */
/*
 * $Id: mailmime_types_helper.h,v 1.17 2008/01/14 17:13:53 hoa Exp $
 */
#[no_mangle]
pub unsafe extern "C" fn mailmime_transfer_encoding_get(
    mut fields: *mut mailmime_fields,
) -> libc::c_int {
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    cur = (*(*fields).fld_list).first;
    while !cur.is_null() {
        let mut field: *mut mailmime_field = 0 as *mut mailmime_field;
        field = (if !cur.is_null() {
            (*cur).data
        } else {
            0 as *mut libc::c_void
        }) as *mut mailmime_field;
        if (*field).fld_type == MAILMIME_FIELD_TRANSFER_ENCODING as libc::c_int {
            return (*(*field).fld_data.fld_encoding).enc_type;
        }
        cur = if !cur.is_null() {
            (*cur).next
        } else {
            0 as *mut clistcell_s
        }
    }
    return MAILMIME_MECHANISM_8BIT as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_disposition_new_filename(
    mut type_0: libc::c_int,
    mut filename: *mut libc::c_char,
) -> *mut mailmime_disposition {
    return mailmime_disposition_new_with_data(
        type_0,
        filename,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        -1i32 as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_disposition_new_with_data(
    mut type_0: libc::c_int,
    mut filename: *mut libc::c_char,
    mut creation_date: *mut libc::c_char,
    mut modification_date: *mut libc::c_char,
    mut read_date: *mut libc::c_char,
    mut size: size_t,
) -> *mut mailmime_disposition {
    let mut current_block: u64;
    let mut dsp_type: *mut mailmime_disposition_type = 0 as *mut mailmime_disposition_type;
    let mut list: *mut clist = 0 as *mut clist;
    let mut r: libc::c_int = 0;
    let mut parm: *mut mailmime_disposition_parm = 0 as *mut mailmime_disposition_parm;
    let mut dsp: *mut mailmime_disposition = 0 as *mut mailmime_disposition;
    dsp_type = mailmime_disposition_type_new(type_0, 0 as *mut libc::c_char);
    if !dsp_type.is_null() {
        list = clist_new();
        if !list.is_null() {
            if !filename.is_null() {
                parm = mailmime_disposition_parm_new(
                    MAILMIME_DISPOSITION_PARM_FILENAME as libc::c_int,
                    filename,
                    0 as *mut libc::c_char,
                    0 as *mut libc::c_char,
                    0 as *mut libc::c_char,
                    0i32 as size_t,
                    0 as *mut mailmime_parameter,
                );
                if parm.is_null() {
                    current_block = 13210718484351940574;
                } else {
                    r = clist_insert_after(list, (*list).last, parm as *mut libc::c_void);
                    if r < 0i32 {
                        mailmime_disposition_parm_free(parm);
                        current_block = 13210718484351940574;
                    } else {
                        current_block = 4166486009154926805;
                    }
                }
            } else {
                current_block = 4166486009154926805;
            }
            match current_block {
                4166486009154926805 => {
                    if !creation_date.is_null() {
                        parm = mailmime_disposition_parm_new(
                            MAILMIME_DISPOSITION_PARM_CREATION_DATE as libc::c_int,
                            0 as *mut libc::c_char,
                            creation_date,
                            0 as *mut libc::c_char,
                            0 as *mut libc::c_char,
                            0i32 as size_t,
                            0 as *mut mailmime_parameter,
                        );
                        if parm.is_null() {
                            current_block = 13210718484351940574;
                        } else {
                            r = clist_insert_after(list, (*list).last, parm as *mut libc::c_void);
                            if r < 0i32 {
                                mailmime_disposition_parm_free(parm);
                                current_block = 13210718484351940574;
                            } else {
                                current_block = 12147880666119273379;
                            }
                        }
                    } else {
                        current_block = 12147880666119273379;
                    }
                    match current_block {
                        13210718484351940574 => {}
                        _ => {
                            if !modification_date.is_null() {
                                parm = mailmime_disposition_parm_new(
                                    MAILMIME_DISPOSITION_PARM_MODIFICATION_DATE as libc::c_int,
                                    0 as *mut libc::c_char,
                                    0 as *mut libc::c_char,
                                    modification_date,
                                    0 as *mut libc::c_char,
                                    0i32 as size_t,
                                    0 as *mut mailmime_parameter,
                                );
                                if parm.is_null() {
                                    current_block = 13210718484351940574;
                                } else {
                                    r = clist_insert_after(
                                        list,
                                        (*list).last,
                                        parm as *mut libc::c_void,
                                    );
                                    if r < 0i32 {
                                        mailmime_disposition_parm_free(parm);
                                        current_block = 13210718484351940574;
                                    } else {
                                        current_block = 13550086250199790493;
                                    }
                                }
                            } else {
                                current_block = 13550086250199790493;
                            }
                            match current_block {
                                13210718484351940574 => {}
                                _ => {
                                    if !read_date.is_null() {
                                        parm = mailmime_disposition_parm_new(
                                            MAILMIME_DISPOSITION_PARM_READ_DATE as libc::c_int,
                                            0 as *mut libc::c_char,
                                            0 as *mut libc::c_char,
                                            0 as *mut libc::c_char,
                                            read_date,
                                            0i32 as size_t,
                                            0 as *mut mailmime_parameter,
                                        );
                                        if parm.is_null() {
                                            current_block = 13210718484351940574;
                                        } else {
                                            r = clist_insert_after(
                                                list,
                                                (*list).last,
                                                parm as *mut libc::c_void,
                                            );
                                            if r < 0i32 {
                                                mailmime_disposition_parm_free(parm);
                                                current_block = 13210718484351940574;
                                            } else {
                                                current_block = 9520865839495247062;
                                            }
                                        }
                                    } else {
                                        current_block = 9520865839495247062;
                                    }
                                    match current_block {
                                        13210718484351940574 => {}
                                        _ => {
                                            if size != -1i32 as size_t {
                                                parm = mailmime_disposition_parm_new(
                                                    MAILMIME_DISPOSITION_PARM_SIZE as libc::c_int,
                                                    0 as *mut libc::c_char,
                                                    0 as *mut libc::c_char,
                                                    0 as *mut libc::c_char,
                                                    0 as *mut libc::c_char,
                                                    size,
                                                    0 as *mut mailmime_parameter,
                                                );
                                                if parm.is_null() {
                                                    current_block = 13210718484351940574;
                                                } else {
                                                    r = clist_insert_after(
                                                        list,
                                                        (*list).last,
                                                        parm as *mut libc::c_void,
                                                    );
                                                    if r < 0i32 {
                                                        mailmime_disposition_parm_free(parm);
                                                        current_block = 13210718484351940574;
                                                    } else {
                                                        current_block = 12199444798915819164;
                                                    }
                                                }
                                            } else {
                                                current_block = 12199444798915819164;
                                            }
                                            match current_block {
                                                13210718484351940574 => {}
                                                _ => {
                                                    dsp = mailmime_disposition_new(dsp_type, list);
                                                    return dsp;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
            clist_foreach(
                list,
                ::std::mem::transmute::<
                    Option<unsafe extern "C" fn(_: *mut mailmime_disposition_parm) -> ()>,
                    clist_func,
                >(Some(mailmime_disposition_parm_free)),
                0 as *mut libc::c_void,
            );
            clist_free(list);
        }
        mailmime_disposition_type_free(dsp_type);
    }
    return 0 as *mut mailmime_disposition;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_fields_new_empty() -> *mut mailmime_fields {
    let mut list: *mut clist = 0 as *mut clist;
    let mut fields: *mut mailmime_fields = 0 as *mut mailmime_fields;
    list = clist_new();
    if !list.is_null() {
        fields = mailmime_fields_new(list);
        if fields.is_null() {
            clist_free(list);
        } else {
            return fields;
        }
    }
    return 0 as *mut mailmime_fields;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_fields_add(
    mut fields: *mut mailmime_fields,
    mut field: *mut mailmime_field,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = clist_insert_after(
        (*fields).fld_list,
        (*(*fields).fld_list).last,
        field as *mut libc::c_void,
    );
    if r < 0i32 {
        return MAILIMF_ERROR_MEMORY as libc::c_int;
    }
    return MAILIMF_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_fields_new_with_data(
    mut encoding: *mut mailmime_mechanism,
    mut id: *mut libc::c_char,
    mut description: *mut libc::c_char,
    mut disposition: *mut mailmime_disposition,
    mut language: *mut mailmime_language,
) -> *mut mailmime_fields {
    let mut current_block: u64;
    let mut field: *mut mailmime_field = 0 as *mut mailmime_field;
    let mut fields: *mut mailmime_fields = 0 as *mut mailmime_fields;
    let mut r: libc::c_int = 0;
    fields = mailmime_fields_new_empty();
    if !fields.is_null() {
        if !encoding.is_null() {
            field = mailmime_field_new(
                MAILMIME_FIELD_TRANSFER_ENCODING as libc::c_int,
                0 as *mut mailmime_content,
                encoding,
                0 as *mut libc::c_char,
                0 as *mut libc::c_char,
                0i32 as uint32_t,
                0 as *mut mailmime_disposition,
                0 as *mut mailmime_language,
                0 as *mut libc::c_char,
            );
            if field.is_null() {
                current_block = 5039974454013832799;
            } else {
                r = mailmime_fields_add(fields, field);
                if r != MAILIMF_NO_ERROR as libc::c_int {
                    mailmime_field_detach(field);
                    mailmime_field_free(field);
                    current_block = 5039974454013832799;
                } else {
                    current_block = 7746791466490516765;
                }
            }
        } else {
            current_block = 7746791466490516765;
        }
        match current_block {
            7746791466490516765 => {
                if !id.is_null() {
                    field = mailmime_field_new(
                        MAILMIME_FIELD_ID as libc::c_int,
                        0 as *mut mailmime_content,
                        0 as *mut mailmime_mechanism,
                        id,
                        0 as *mut libc::c_char,
                        0i32 as uint32_t,
                        0 as *mut mailmime_disposition,
                        0 as *mut mailmime_language,
                        0 as *mut libc::c_char,
                    );
                    if field.is_null() {
                        current_block = 5039974454013832799;
                    } else {
                        r = mailmime_fields_add(fields, field);
                        if r != MAILIMF_NO_ERROR as libc::c_int {
                            mailmime_field_detach(field);
                            mailmime_field_free(field);
                            current_block = 5039974454013832799;
                        } else {
                            current_block = 13242334135786603907;
                        }
                    }
                } else {
                    current_block = 13242334135786603907;
                }
                match current_block {
                    5039974454013832799 => {}
                    _ => {
                        if !description.is_null() {
                            field = mailmime_field_new(
                                MAILMIME_FIELD_DESCRIPTION as libc::c_int,
                                0 as *mut mailmime_content,
                                0 as *mut mailmime_mechanism,
                                0 as *mut libc::c_char,
                                description,
                                0i32 as uint32_t,
                                0 as *mut mailmime_disposition,
                                0 as *mut mailmime_language,
                                0 as *mut libc::c_char,
                            );
                            if field.is_null() {
                                current_block = 5039974454013832799;
                            } else {
                                r = mailmime_fields_add(fields, field);
                                if r != MAILIMF_NO_ERROR as libc::c_int {
                                    mailmime_field_detach(field);
                                    mailmime_field_free(field);
                                    current_block = 5039974454013832799;
                                } else {
                                    current_block = 15125582407903384992;
                                }
                            }
                        } else {
                            current_block = 15125582407903384992;
                        }
                        match current_block {
                            5039974454013832799 => {}
                            _ => {
                                if !disposition.is_null() {
                                    field = mailmime_field_new(
                                        MAILMIME_FIELD_DISPOSITION as libc::c_int,
                                        0 as *mut mailmime_content,
                                        0 as *mut mailmime_mechanism,
                                        0 as *mut libc::c_char,
                                        0 as *mut libc::c_char,
                                        0i32 as uint32_t,
                                        disposition,
                                        0 as *mut mailmime_language,
                                        0 as *mut libc::c_char,
                                    );
                                    if field.is_null() {
                                        current_block = 5039974454013832799;
                                    } else {
                                        r = mailmime_fields_add(fields, field);
                                        if r != MAILIMF_NO_ERROR as libc::c_int {
                                            mailmime_field_detach(field);
                                            mailmime_field_free(field);
                                            current_block = 5039974454013832799;
                                        } else {
                                            current_block = 9520865839495247062;
                                        }
                                    }
                                } else {
                                    current_block = 9520865839495247062;
                                }
                                match current_block {
                                    5039974454013832799 => {}
                                    _ => {
                                        if !language.is_null() {
                                            field = mailmime_field_new(
                                                MAILMIME_FIELD_DISPOSITION as libc::c_int,
                                                0 as *mut mailmime_content,
                                                0 as *mut mailmime_mechanism,
                                                0 as *mut libc::c_char,
                                                0 as *mut libc::c_char,
                                                0i32 as uint32_t,
                                                0 as *mut mailmime_disposition,
                                                language,
                                                0 as *mut libc::c_char,
                                            );
                                            if field.is_null() {
                                                current_block = 5039974454013832799;
                                            } else {
                                                r = mailmime_fields_add(fields, field);
                                                if r != MAILIMF_NO_ERROR as libc::c_int {
                                                    mailmime_field_detach(field);
                                                    mailmime_field_free(field);
                                                    current_block = 5039974454013832799;
                                                } else {
                                                    current_block = 15512526488502093901;
                                                }
                                            }
                                        } else {
                                            current_block = 15512526488502093901;
                                        }
                                        match current_block {
                                            5039974454013832799 => {}
                                            _ => return fields,
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        clist_foreach(
            (*fields).fld_list,
            ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut mailmime_field) -> ()>,
                clist_func,
            >(Some(mailmime_field_detach)),
            0 as *mut libc::c_void,
        );
        mailmime_fields_free(fields);
    }
    return 0 as *mut mailmime_fields;
}
unsafe extern "C" fn mailmime_field_detach(mut field: *mut mailmime_field) {
    match (*field).fld_type {
        1 => (*field).fld_data.fld_content = 0 as *mut mailmime_content,
        2 => (*field).fld_data.fld_encoding = 0 as *mut mailmime_mechanism,
        3 => (*field).fld_data.fld_id = 0 as *mut libc::c_char,
        4 => (*field).fld_data.fld_description = 0 as *mut libc::c_char,
        6 => (*field).fld_data.fld_disposition = 0 as *mut mailmime_disposition,
        7 => (*field).fld_data.fld_language = 0 as *mut mailmime_language,
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_fields_new_with_version(
    mut encoding: *mut mailmime_mechanism,
    mut id: *mut libc::c_char,
    mut description: *mut libc::c_char,
    mut disposition: *mut mailmime_disposition,
    mut language: *mut mailmime_language,
) -> *mut mailmime_fields {
    let mut field: *mut mailmime_field = 0 as *mut mailmime_field;
    let mut fields: *mut mailmime_fields = 0 as *mut mailmime_fields;
    let mut r: libc::c_int = 0;
    fields = mailmime_fields_new_with_data(encoding, id, description, disposition, language);
    if !fields.is_null() {
        field = mailmime_field_new(
            MAILMIME_FIELD_VERSION as libc::c_int,
            0 as *mut mailmime_content,
            0 as *mut mailmime_mechanism,
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            (1i32 << 16i32) as uint32_t,
            0 as *mut mailmime_disposition,
            0 as *mut mailmime_language,
            0 as *mut libc::c_char,
        );
        if !field.is_null() {
            r = mailmime_fields_add(fields, field);
            if r != MAILIMF_NO_ERROR as libc::c_int {
                mailmime_field_detach(field);
                mailmime_field_free(field);
            } else {
                return fields;
            }
        }
        clist_foreach(
            (*fields).fld_list,
            ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut mailmime_field) -> ()>,
                clist_func,
            >(Some(mailmime_field_detach)),
            0 as *mut libc::c_void,
        );
        mailmime_fields_free(fields);
    }
    return 0 as *mut mailmime_fields;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_get_content_message() -> *mut mailmime_content {
    let mut list: *mut clist = 0 as *mut clist;
    let mut composite_type: *mut mailmime_composite_type = 0 as *mut mailmime_composite_type;
    let mut mime_type: *mut mailmime_type = 0 as *mut mailmime_type;
    let mut content: *mut mailmime_content = 0 as *mut mailmime_content;
    let mut subtype: *mut libc::c_char = 0 as *mut libc::c_char;
    composite_type = mailmime_composite_type_new(
        MAILMIME_COMPOSITE_TYPE_MESSAGE as libc::c_int,
        0 as *mut libc::c_char,
    );
    if !composite_type.is_null() {
        mime_type = mailmime_type_new(
            MAILMIME_TYPE_COMPOSITE_TYPE as libc::c_int,
            0 as *mut mailmime_discrete_type,
            composite_type,
        );
        if !mime_type.is_null() {
            composite_type = 0 as *mut mailmime_composite_type;
            list = clist_new();
            if !list.is_null() {
                subtype = strdup(b"rfc822\x00" as *const u8 as *const libc::c_char);
                if !subtype.is_null() {
                    content = mailmime_content_new(mime_type, subtype, list);
                    if content.is_null() {
                        free(subtype as *mut libc::c_void);
                    } else {
                        return content;
                    }
                }
                clist_free(list);
            }
            mailmime_type_free(mime_type);
        }
        if !composite_type.is_null() {
            mailmime_composite_type_free(composite_type);
        }
    }
    return 0 as *mut mailmime_content;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_get_content_text() -> *mut mailmime_content {
    let mut list: *mut clist = 0 as *mut clist;
    let mut discrete_type: *mut mailmime_discrete_type = 0 as *mut mailmime_discrete_type;
    let mut mime_type: *mut mailmime_type = 0 as *mut mailmime_type;
    let mut content: *mut mailmime_content = 0 as *mut mailmime_content;
    let mut subtype: *mut libc::c_char = 0 as *mut libc::c_char;
    discrete_type = mailmime_discrete_type_new(
        MAILMIME_DISCRETE_TYPE_TEXT as libc::c_int,
        0 as *mut libc::c_char,
    );
    if !discrete_type.is_null() {
        mime_type = mailmime_type_new(
            MAILMIME_TYPE_DISCRETE_TYPE as libc::c_int,
            discrete_type,
            0 as *mut mailmime_composite_type,
        );
        if !mime_type.is_null() {
            discrete_type = 0 as *mut mailmime_discrete_type;
            list = clist_new();
            if !list.is_null() {
                subtype = strdup(b"plain\x00" as *const u8 as *const libc::c_char);
                if !subtype.is_null() {
                    content = mailmime_content_new(mime_type, subtype, list);
                    if content.is_null() {
                        free(subtype as *mut libc::c_void);
                    } else {
                        return content;
                    }
                }
                clist_free(list);
            }
            mailmime_type_free(mime_type);
        }
        if !discrete_type.is_null() {
            mailmime_discrete_type_free(discrete_type);
        }
    }
    return 0 as *mut mailmime_content;
}
/* struct mailmime_content * mailmime_get_content(char * mime_type); */
#[no_mangle]
pub unsafe extern "C" fn mailmime_data_new_data(
    mut encoding: libc::c_int,
    mut encoded: libc::c_int,
    mut data: *const libc::c_char,
    mut length: size_t,
) -> *mut mailmime_data {
    return mailmime_data_new(
        MAILMIME_DATA_TEXT as libc::c_int,
        encoding,
        encoded,
        data,
        length,
        0 as *mut libc::c_char,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_data_new_file(
    mut encoding: libc::c_int,
    mut encoded: libc::c_int,
    mut filename: *mut libc::c_char,
) -> *mut mailmime_data {
    return mailmime_data_new(
        MAILMIME_DATA_FILE as libc::c_int,
        encoding,
        encoded,
        0 as *const libc::c_char,
        0i32 as size_t,
        filename,
    );
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_new_message_data(mut msg_mime: *mut mailmime) -> *mut mailmime {
    let mut content: *mut mailmime_content = 0 as *mut mailmime_content;
    let mut build_info: *mut mailmime = 0 as *mut mailmime;
    let mut mime_fields: *mut mailmime_fields = 0 as *mut mailmime_fields;
    content = mailmime_get_content_message();
    if !content.is_null() {
        mime_fields = mailmime_fields_new_with_version(
            0 as *mut mailmime_mechanism,
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            0 as *mut mailmime_disposition,
            0 as *mut mailmime_language,
        );
        if !mime_fields.is_null() {
            build_info = mailmime_new(
                MAILMIME_MESSAGE as libc::c_int,
                0 as *const libc::c_char,
                0i32 as size_t,
                mime_fields,
                content,
                0 as *mut mailmime_data,
                0 as *mut mailmime_data,
                0 as *mut mailmime_data,
                0 as *mut clist,
                0 as *mut mailimf_fields,
                msg_mime,
            );
            if build_info.is_null() {
                mailmime_fields_free(mime_fields);
            } else {
                return build_info;
            }
        }
        mailmime_content_free(content);
    }
    return 0 as *mut mailmime;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_new_empty(
    mut content: *mut mailmime_content,
    mut mime_fields: *mut mailmime_fields,
) -> *mut mailmime {
    let mut current_block: u64;
    let mut build_info: *mut mailmime = 0 as *mut mailmime;
    let mut list: *mut clist = 0 as *mut clist;
    let mut r: libc::c_int = 0;
    let mut mime_type: libc::c_int = 0;
    list = 0 as *mut clist;
    match (*(*content).ct_type).tp_type {
        1 => {
            mime_type = MAILMIME_SINGLE as libc::c_int;
            current_block = 12349973810996921269;
        }
        2 => match (*(*(*content).ct_type).tp_data.tp_composite_type).ct_type {
            2 => {
                current_block = 5822726848290245908;
                match current_block {
                    565197971715936940 => {
                        if strcasecmp(
                            (*content).ct_subtype,
                            b"rfc822\x00" as *const u8 as *const libc::c_char,
                        ) == 0i32
                        {
                            mime_type = MAILMIME_MESSAGE as libc::c_int
                        } else {
                            mime_type = MAILMIME_SINGLE as libc::c_int
                        }
                    }
                    _ => mime_type = MAILMIME_MULTIPLE as libc::c_int,
                }
                current_block = 12349973810996921269;
            }
            1 => {
                current_block = 565197971715936940;
                match current_block {
                    565197971715936940 => {
                        if strcasecmp(
                            (*content).ct_subtype,
                            b"rfc822\x00" as *const u8 as *const libc::c_char,
                        ) == 0i32
                        {
                            mime_type = MAILMIME_MESSAGE as libc::c_int
                        } else {
                            mime_type = MAILMIME_SINGLE as libc::c_int
                        }
                    }
                    _ => mime_type = MAILMIME_MULTIPLE as libc::c_int,
                }
                current_block = 12349973810996921269;
            }
            _ => {
                current_block = 13576996419214490990;
            }
        },
        _ => {
            current_block = 13576996419214490990;
        }
    }
    match current_block {
        12349973810996921269 => {
            if mime_type == MAILMIME_MULTIPLE as libc::c_int {
                let mut attr_name: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut attr_value: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut param: *mut mailmime_parameter = 0 as *mut mailmime_parameter;
                let mut parameters: *mut clist = 0 as *mut clist;
                let mut boundary: *mut libc::c_char = 0 as *mut libc::c_char;
                list = clist_new();
                if list.is_null() {
                    current_block = 13576996419214490990;
                } else {
                    attr_name = strdup(b"boundary\x00" as *const u8 as *const libc::c_char);
                    if attr_name.is_null() {
                        current_block = 13142422523813476356;
                    } else {
                        boundary = mailmime_generate_boundary();
                        attr_value = boundary;
                        if attr_name.is_null() {
                            free(attr_name as *mut libc::c_void);
                            current_block = 13142422523813476356;
                        } else {
                            param = mailmime_parameter_new(attr_name, attr_value);
                            if param.is_null() {
                                free(attr_value as *mut libc::c_void);
                                free(attr_name as *mut libc::c_void);
                                current_block = 13142422523813476356;
                            } else {
                                if (*content).ct_parameters.is_null() {
                                    parameters = clist_new();
                                    if parameters.is_null() {
                                        mailmime_parameter_free(param);
                                        current_block = 13142422523813476356;
                                    } else {
                                        current_block = 1836292691772056875;
                                    }
                                } else {
                                    parameters = (*content).ct_parameters;
                                    current_block = 1836292691772056875;
                                }
                                match current_block {
                                    13142422523813476356 => {}
                                    _ => {
                                        r = clist_insert_after(
                                            parameters,
                                            (*parameters).last,
                                            param as *mut libc::c_void,
                                        );
                                        if r != 0i32 {
                                            clist_free(parameters);
                                            mailmime_parameter_free(param);
                                            current_block = 13142422523813476356;
                                        } else {
                                            if (*content).ct_parameters.is_null() {
                                                (*content).ct_parameters = parameters
                                            }
                                            current_block = 2543120759711851213;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    match current_block {
                        2543120759711851213 => {}
                        _ => {
                            clist_free(list);
                            current_block = 13576996419214490990;
                        }
                    }
                }
            } else {
                current_block = 2543120759711851213;
            }
            match current_block {
                13576996419214490990 => {}
                _ => {
                    build_info = mailmime_new(
                        mime_type,
                        0 as *const libc::c_char,
                        0i32 as size_t,
                        mime_fields,
                        content,
                        0 as *mut mailmime_data,
                        0 as *mut mailmime_data,
                        0 as *mut mailmime_data,
                        list,
                        0 as *mut mailimf_fields,
                        0 as *mut mailmime,
                    );
                    if build_info.is_null() {
                        clist_free(list);
                        return 0 as *mut mailmime;
                    }
                    return build_info;
                }
            }
        }
        _ => {}
    }
    return 0 as *mut mailmime;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_generate_boundary() -> *mut libc::c_char {
    let mut id: [libc::c_char; 512] = [0; 512];
    let mut now: time_t = 0;
    let mut name: [libc::c_char; 512] = [0; 512];
    let mut value: libc::c_long = 0;
    now = time(0 as *mut time_t);
    value = random();
    gethostname(name.as_mut_ptr(), 512i32 as size_t);
    snprintf(
        id.as_mut_ptr(),
        512i32 as libc::c_ulong,
        b"%llx_%lx_%x\x00" as *const u8 as *const libc::c_char,
        now as libc::c_longlong,
        value,
        getpid(),
    );
    return strdup(id.as_mut_ptr());
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_new_with_content(
    mut content_type: *const libc::c_char,
    mut mime_fields: *mut mailmime_fields,
    mut result: *mut *mut mailmime,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    let mut cur_token: size_t = 0;
    let mut content: *mut mailmime_content = 0 as *mut mailmime_content;
    let mut build_info: *mut mailmime = 0 as *mut mailmime;
    let mut res: libc::c_int = 0;
    cur_token = 0i32 as size_t;
    r = mailmime_content_parse(
        content_type,
        strlen(content_type),
        &mut cur_token,
        &mut content,
    );
    if r != MAILIMF_NO_ERROR as libc::c_int {
        res = r
    } else {
        build_info = mailmime_new_empty(content, mime_fields);
        if build_info.is_null() {
            res = MAILIMF_ERROR_MEMORY as libc::c_int;
            mailmime_content_free(content);
        } else {
            *result = build_info;
            return MAILIMF_NO_ERROR as libc::c_int;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_set_preamble_file(
    mut build_info: *mut mailmime,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut data: *mut mailmime_data = 0 as *mut mailmime_data;
    data = mailmime_data_new(
        MAILMIME_DATA_FILE as libc::c_int,
        MAILMIME_MECHANISM_8BIT as libc::c_int,
        0i32,
        0 as *const libc::c_char,
        0i32 as size_t,
        filename,
    );
    if data.is_null() {
        return MAILIMF_ERROR_MEMORY as libc::c_int;
    }
    (*build_info).mm_data.mm_multipart.mm_preamble = data;
    return MAILIMF_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_set_epilogue_file(
    mut build_info: *mut mailmime,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut data: *mut mailmime_data = 0 as *mut mailmime_data;
    data = mailmime_data_new(
        MAILMIME_DATA_FILE as libc::c_int,
        MAILMIME_MECHANISM_8BIT as libc::c_int,
        0i32,
        0 as *const libc::c_char,
        0i32 as size_t,
        filename,
    );
    if data.is_null() {
        return MAILIMF_ERROR_MEMORY as libc::c_int;
    }
    (*build_info).mm_data.mm_multipart.mm_epilogue = data;
    return MAILIMF_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_set_preamble_text(
    mut build_info: *mut mailmime,
    mut data_str: *mut libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut data: *mut mailmime_data = 0 as *mut mailmime_data;
    data = mailmime_data_new(
        MAILMIME_DATA_TEXT as libc::c_int,
        MAILMIME_MECHANISM_8BIT as libc::c_int,
        0i32,
        data_str,
        length,
        0 as *mut libc::c_char,
    );
    if data.is_null() {
        return MAILIMF_ERROR_MEMORY as libc::c_int;
    }
    (*build_info).mm_data.mm_multipart.mm_preamble = data;
    return MAILIMF_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_set_epilogue_text(
    mut build_info: *mut mailmime,
    mut data_str: *mut libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut data: *mut mailmime_data = 0 as *mut mailmime_data;
    data = mailmime_data_new(
        MAILMIME_DATA_TEXT as libc::c_int,
        MAILMIME_MECHANISM_8BIT as libc::c_int,
        0i32,
        data_str,
        length,
        0 as *mut libc::c_char,
    );
    if data.is_null() {
        return MAILIMF_ERROR_MEMORY as libc::c_int;
    }
    (*build_info).mm_data.mm_multipart.mm_epilogue = data;
    return MAILIMF_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_set_body_file(
    mut build_info: *mut mailmime,
    mut filename: *mut libc::c_char,
) -> libc::c_int {
    let mut encoding: libc::c_int = 0;
    let mut data: *mut mailmime_data = 0 as *mut mailmime_data;
    encoding = mailmime_transfer_encoding_get((*build_info).mm_mime_fields);
    data = mailmime_data_new(
        MAILMIME_DATA_FILE as libc::c_int,
        encoding,
        0i32,
        0 as *const libc::c_char,
        0i32 as size_t,
        filename,
    );
    if data.is_null() {
        return MAILIMF_ERROR_MEMORY as libc::c_int;
    }
    (*build_info).mm_data.mm_single = data;
    return MAILIMF_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_set_body_text(
    mut build_info: *mut mailmime,
    mut data_str: *mut libc::c_char,
    mut length: size_t,
) -> libc::c_int {
    let mut encoding: libc::c_int = 0;
    let mut data: *mut mailmime_data = 0 as *mut mailmime_data;
    encoding = mailmime_transfer_encoding_get((*build_info).mm_mime_fields);
    data = mailmime_data_new(
        MAILMIME_DATA_TEXT as libc::c_int,
        encoding,
        0i32,
        data_str,
        length,
        0 as *mut libc::c_char,
    );
    if data.is_null() {
        return MAILIMF_ERROR_MEMORY as libc::c_int;
    }
    (*build_info).mm_data.mm_single = data;
    return MAILIMF_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_add_part(
    mut build_info: *mut mailmime,
    mut part: *mut mailmime,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    if (*build_info).mm_type == MAILMIME_MESSAGE as libc::c_int {
        (*build_info).mm_data.mm_message.mm_msg_mime = part;
        (*part).mm_parent_type = MAILMIME_MESSAGE as libc::c_int;
        (*part).mm_parent = build_info
    } else if (*build_info).mm_type == MAILMIME_MULTIPLE as libc::c_int {
        r = clist_insert_after(
            (*build_info).mm_data.mm_multipart.mm_mp_list,
            (*(*build_info).mm_data.mm_multipart.mm_mp_list).last,
            part as *mut libc::c_void,
        );
        if r != 0i32 {
            return MAILIMF_ERROR_MEMORY as libc::c_int;
        }
        (*part).mm_parent_type = MAILMIME_MULTIPLE as libc::c_int;
        (*part).mm_parent = build_info;
        (*part).mm_multipart_pos = (*(*build_info).mm_data.mm_multipart.mm_mp_list).last
    } else {
        return MAILIMF_ERROR_INVAL as libc::c_int;
    }
    return MAILIMF_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_remove_part(mut mime: *mut mailmime) {
    let mut parent: *mut mailmime = 0 as *mut mailmime;
    parent = (*mime).mm_parent;
    if parent.is_null() {
        return;
    }
    match (*mime).mm_parent_type {
        3 => {
            (*mime).mm_parent = 0 as *mut mailmime;
            (*parent).mm_data.mm_message.mm_msg_mime = 0 as *mut mailmime
        }
        2 => {
            (*mime).mm_parent = 0 as *mut mailmime;
            clist_delete(
                (*parent).mm_data.mm_multipart.mm_mp_list,
                (*mime).mm_multipart_pos,
            );
        }
        _ => {}
    };
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_set_imf_fields(
    mut build_info: *mut mailmime,
    mut mm_fields: *mut mailimf_fields,
) {
    (*build_info).mm_data.mm_message.mm_fields = mm_fields;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_single_fields_init(
    mut single_fields: *mut mailmime_single_fields,
    mut fld_fields: *mut mailmime_fields,
    mut fld_content: *mut mailmime_content,
) {
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    memset(
        single_fields as *mut libc::c_void,
        0i32,
        ::std::mem::size_of::<mailmime_single_fields>() as libc::c_ulong,
    );
    if !fld_content.is_null() {
        mailmime_content_single_fields_init(single_fields, fld_content);
    }
    if fld_fields.is_null() {
        return;
    }
    cur = (*(*fld_fields).fld_list).first;
    while !cur.is_null() {
        let mut field: *mut mailmime_field = 0 as *mut mailmime_field;
        field = (if !cur.is_null() {
            (*cur).data
        } else {
            0 as *mut libc::c_void
        }) as *mut mailmime_field;
        match (*field).fld_type {
            1 => {
                mailmime_content_single_fields_init(single_fields, (*field).fld_data.fld_content);
            }
            2 => (*single_fields).fld_encoding = (*field).fld_data.fld_encoding,
            3 => (*single_fields).fld_id = (*field).fld_data.fld_id,
            4 => (*single_fields).fld_description = (*field).fld_data.fld_description,
            5 => (*single_fields).fld_version = (*field).fld_data.fld_version,
            6 => {
                mailmime_disposition_single_fields_init(
                    single_fields,
                    (*field).fld_data.fld_disposition,
                );
            }
            7 => (*single_fields).fld_language = (*field).fld_data.fld_language,
            8 => (*single_fields).fld_location = (*field).fld_data.fld_location,
            _ => {}
        }
        cur = if !cur.is_null() {
            (*cur).next
        } else {
            0 as *mut clistcell_s
        }
    }
}
unsafe extern "C" fn mailmime_disposition_single_fields_init(
    mut single_fields: *mut mailmime_single_fields,
    mut fld_disposition: *mut mailmime_disposition,
) {
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    (*single_fields).fld_disposition = fld_disposition;
    cur = (*(*fld_disposition).dsp_parms).first;
    while !cur.is_null() {
        let mut param: *mut mailmime_disposition_parm = 0 as *mut mailmime_disposition_parm;
        param = (if !cur.is_null() {
            (*cur).data
        } else {
            0 as *mut libc::c_void
        }) as *mut mailmime_disposition_parm;
        match (*param).pa_type {
            0 => (*single_fields).fld_disposition_filename = (*param).pa_data.pa_filename,
            1 => (*single_fields).fld_disposition_creation_date = (*param).pa_data.pa_creation_date,
            2 => {
                (*single_fields).fld_disposition_modification_date =
                    (*param).pa_data.pa_modification_date
            }
            3 => (*single_fields).fld_disposition_read_date = (*param).pa_data.pa_read_date,
            4 => (*single_fields).fld_disposition_size = (*param).pa_data.pa_size,
            _ => {}
        }
        cur = if !cur.is_null() {
            (*cur).next
        } else {
            0 as *mut clistcell_s
        }
    }
}
unsafe extern "C" fn mailmime_content_single_fields_init(
    mut single_fields: *mut mailmime_single_fields,
    mut fld_content: *mut mailmime_content,
) {
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    (*single_fields).fld_content = fld_content;
    cur = (*(*fld_content).ct_parameters).first;
    while !cur.is_null() {
        let mut param: *mut mailmime_parameter = 0 as *mut mailmime_parameter;
        param = (if !cur.is_null() {
            (*cur).data
        } else {
            0 as *mut libc::c_void
        }) as *mut mailmime_parameter;
        if strcasecmp(
            (*param).pa_name,
            b"boundary\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*single_fields).fld_content_boundary = (*param).pa_value
        }
        if strcasecmp(
            (*param).pa_name,
            b"charset\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*single_fields).fld_content_charset = (*param).pa_value
        }
        if strcasecmp(
            (*param).pa_name,
            b"name\x00" as *const u8 as *const libc::c_char,
        ) == 0i32
        {
            (*single_fields).fld_content_name = (*param).pa_value
        }
        cur = if !cur.is_null() {
            (*cur).next
        } else {
            0 as *mut clistcell_s
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_single_fields_new(
    mut fld_fields: *mut mailmime_fields,
    mut fld_content: *mut mailmime_content,
) -> *mut mailmime_single_fields {
    let mut single_fields: *mut mailmime_single_fields = 0 as *mut mailmime_single_fields;
    single_fields = malloc(::std::mem::size_of::<mailmime_single_fields>() as libc::c_ulong)
        as *mut mailmime_single_fields;
    if single_fields.is_null() {
        return 0 as *mut mailmime_single_fields;
    } else {
        mailmime_single_fields_init(single_fields, fld_fields, fld_content);
        return single_fields;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_single_fields_free(
    mut single_fields: *mut mailmime_single_fields,
) {
    free(single_fields as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_smart_add_part(
    mut mime: *mut mailmime,
    mut mime_sub: *mut mailmime,
) -> libc::c_int {
    let mut saved_sub: *mut mailmime = 0 as *mut mailmime;
    let mut mp: *mut mailmime = 0 as *mut mailmime;
    let mut res: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    match (*mime).mm_type {
        1 => res = MAILIMF_ERROR_INVAL as libc::c_int,
        2 => {
            r = mailmime_add_part(mime, mime_sub);
            if r != MAILIMF_NO_ERROR as libc::c_int {
                res = MAILIMF_ERROR_MEMORY as libc::c_int
            } else {
                return MAILIMF_NO_ERROR as libc::c_int;
            }
        }
        _ => {
            /* MAILMIME_MESSAGE */
            if (*mime).mm_data.mm_message.mm_msg_mime.is_null() {
                r = mailmime_add_part(mime, mime_sub);
                if r != MAILIMF_NO_ERROR as libc::c_int {
                    res = MAILIMF_ERROR_MEMORY as libc::c_int
                } else {
                    return MAILIMF_NO_ERROR as libc::c_int;
                }
            } else {
                if (*(*mime).mm_data.mm_message.mm_msg_mime).mm_type
                    == MAILMIME_MULTIPLE as libc::c_int
                {
                    return mailmime_add_part((*mime).mm_data.mm_message.mm_msg_mime, mime_sub);
                }
                saved_sub = (*mime).mm_data.mm_message.mm_msg_mime;
                mp = mailmime_multiple_new(
                    b"multipart/mixed\x00" as *const u8 as *const libc::c_char,
                );
                if mp.is_null() {
                    res = MAILIMF_ERROR_MEMORY as libc::c_int
                } else {
                    mailmime_remove_part(saved_sub);
                    r = mailmime_add_part(mime, mp);
                    if r != MAILIMF_NO_ERROR as libc::c_int {
                        res = MAILIMF_ERROR_MEMORY as libc::c_int;
                        mailmime_free(mp);
                    } else {
                        r = mailmime_add_part(mp, saved_sub);
                        if r != MAILIMF_NO_ERROR as libc::c_int {
                            res = MAILIMF_ERROR_MEMORY as libc::c_int
                        } else {
                            r = mailmime_add_part(mp, mime_sub);
                            if r != MAILIMF_NO_ERROR as libc::c_int {
                                res = MAILIMF_ERROR_MEMORY as libc::c_int
                            } else {
                                return MAILIMF_NO_ERROR as libc::c_int;
                            }
                        }
                    }
                    mailmime_free(saved_sub);
                }
            }
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_multiple_new(mut type_0: *const libc::c_char) -> *mut mailmime {
    let mut mime_fields: *mut mailmime_fields = 0 as *mut mailmime_fields;
    let mut content: *mut mailmime_content = 0 as *mut mailmime_content;
    let mut mp: *mut mailmime = 0 as *mut mailmime;
    mime_fields = mailmime_fields_new_empty();
    if !mime_fields.is_null() {
        content = mailmime_content_new_with_str(type_0);
        if !content.is_null() {
            mp = mailmime_new_empty(content, mime_fields);
            if mp.is_null() {
                mailmime_content_free(content);
            } else {
                return mp;
            }
        }
        mailmime_fields_free(mime_fields);
    }
    return 0 as *mut mailmime;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_content_new_with_str(
    mut str: *const libc::c_char,
) -> *mut mailmime_content {
    let mut r: libc::c_int = 0;
    let mut cur_token: size_t = 0;
    let mut content: *mut mailmime_content = 0 as *mut mailmime_content;
    cur_token = 0i32 as size_t;
    r = mailmime_content_parse(str, strlen(str), &mut cur_token, &mut content);
    if r != MAILIMF_NO_ERROR as libc::c_int {
        return 0 as *mut mailmime_content;
    }
    return content;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_smart_remove_part(mut mime: *mut mailmime) -> libc::c_int {
    let mut parent: *mut mailmime = 0 as *mut mailmime;
    let mut res: libc::c_int = 0;
    parent = (*mime).mm_parent;
    if parent.is_null() {
        res = MAILIMF_ERROR_INVAL as libc::c_int
    } else {
        match (*mime).mm_type {
            3 => {
                if !(*mime).mm_data.mm_message.mm_msg_mime.is_null() {
                    res = MAILIMF_ERROR_INVAL as libc::c_int
                } else {
                    mailmime_remove_part(mime);
                    mailmime_free(mime);
                    return MAILIMF_NO_ERROR as libc::c_int;
                }
            }
            2 => {
                if !((*(*mime).mm_data.mm_multipart.mm_mp_list).first
                    == (*(*mime).mm_data.mm_multipart.mm_mp_list).last
                    && (*(*mime).mm_data.mm_multipart.mm_mp_list).last.is_null())
                {
                    res = MAILIMF_ERROR_INVAL as libc::c_int
                } else {
                    mailmime_remove_part(mime);
                    mailmime_free(mime);
                    return MAILIMF_NO_ERROR as libc::c_int;
                }
            }
            1 => {
                mailmime_remove_part(mime);
                mailmime_free(mime);
                return MAILIMF_NO_ERROR as libc::c_int;
            }
            _ => return MAILIMF_ERROR_INVAL as libc::c_int,
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_fields_new_encoding(
    mut type_0: libc::c_int,
) -> *mut mailmime_fields {
    let mut encoding: *mut mailmime_mechanism = 0 as *mut mailmime_mechanism;
    let mut mime_fields: *mut mailmime_fields = 0 as *mut mailmime_fields;
    encoding = mailmime_mechanism_new(type_0, 0 as *mut libc::c_char);
    if !encoding.is_null() {
        mime_fields = mailmime_fields_new_with_data(
            encoding,
            0 as *mut libc::c_char,
            0 as *mut libc::c_char,
            0 as *mut mailmime_disposition,
            0 as *mut mailmime_language,
        );
        if mime_fields.is_null() {
            mailmime_mechanism_free(encoding);
        } else {
            return mime_fields;
        }
    }
    return 0 as *mut mailmime_fields;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_fields_new_filename(
    mut dsp_type: libc::c_int,
    mut filename: *mut libc::c_char,
    mut encoding_type: libc::c_int,
) -> *mut mailmime_fields {
    let mut dsp: *mut mailmime_disposition = 0 as *mut mailmime_disposition;
    let mut encoding: *mut mailmime_mechanism = 0 as *mut mailmime_mechanism;
    let mut mime_fields: *mut mailmime_fields = 0 as *mut mailmime_fields;
    dsp = mailmime_disposition_new_with_data(
        dsp_type,
        filename,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        0 as *mut libc::c_char,
        -1i32 as size_t,
    );
    if !dsp.is_null() {
        encoding = mailmime_mechanism_new(encoding_type, 0 as *mut libc::c_char);
        if !encoding.is_null() {
            mime_fields = mailmime_fields_new_with_data(
                encoding,
                0 as *mut libc::c_char,
                0 as *mut libc::c_char,
                dsp,
                0 as *mut mailmime_language,
            );
            if mime_fields.is_null() {
                mailmime_encoding_free(encoding);
            } else {
                return mime_fields;
            }
        }
        mailmime_disposition_free(dsp);
    }
    return 0 as *mut mailmime_fields;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_param_new_with_data(
    mut name: *mut libc::c_char,
    mut value: *mut libc::c_char,
) -> *mut mailmime_parameter {
    let mut param_name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut param_value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut param: *mut mailmime_parameter = 0 as *mut mailmime_parameter;
    param_name = strdup(name);
    if !param_name.is_null() {
        param_value = strdup(value);
        if !param_value.is_null() {
            param = mailmime_parameter_new(param_name, param_value);
            if param.is_null() {
                free(param_value as *mut libc::c_void);
            } else {
                return param;
            }
        }
        free(param_name as *mut libc::c_void);
    }
    return 0 as *mut mailmime_parameter;
}
