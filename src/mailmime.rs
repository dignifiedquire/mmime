use libc;
extern "C" {
    #[no_mangle]
    fn __toupper(_: __darwin_ct_rune_t) -> __darwin_ct_rune_t;

    /* Allocate a new pointer list */
    #[no_mangle]
    fn clist_new() -> *mut clist;
    /* Destroys a list. Data pointed by data pointers is NOT freed. */
    #[no_mangle]
    fn clist_free(_: *mut clist);
    /* Inserts this data pointer after the element pointed by the iterator */
    #[no_mangle]
    fn clist_insert_after(_: *mut clist, _: *mut clistiter, _: *mut libc::c_void) -> libc::c_int;
    #[no_mangle]
    fn clist_foreach(lst: *mut clist, func: clist_func, data: *mut libc::c_void);
    /* internal use */
    #[no_mangle]
    fn mailimf_atom_free(atom: *mut libc::c_char);
    #[no_mangle]
    fn mailimf_cfws_parse(
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimf_unstrict_char_parse(
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
        token: libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimf_custom_string_parse(
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
        result: *mut *mut libc::c_char,
        is_custom_char: Option<unsafe extern "C" fn(_: libc::c_char) -> libc::c_int>,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimf_token_case_insensitive_len_parse(
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
        token: *mut libc::c_char,
        token_length: size_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimf_quoted_string_parse(
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
        result: *mut *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimf_number_parse(
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
        result: *mut uint32_t,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimf_msg_id_parse(
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
        result: *mut *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailimf_atom_parse(
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
        result: *mut *mut libc::c_char,
    ) -> libc::c_int;
    #[no_mangle]
    fn mailmime_attribute_free(attribute: *mut libc::c_char);
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
    fn mailmime_description_free(description: *mut libc::c_char);
    #[no_mangle]
    fn mailmime_location_free(location: *mut libc::c_char);
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
    fn mailmime_extension_token_free(extension: *mut libc::c_char);
    #[no_mangle]
    fn mailmime_id_free(id: *mut libc::c_char);
    #[no_mangle]
    fn mailmime_mechanism_new(
        enc_type: libc::c_int,
        enc_token: *mut libc::c_char,
    ) -> *mut mailmime_mechanism;
    #[no_mangle]
    fn mailmime_parameter_new(
        pa_name: *mut libc::c_char,
        pa_value: *mut libc::c_char,
    ) -> *mut mailmime_parameter;
    #[no_mangle]
    fn mailmime_parameter_free(parameter: *mut mailmime_parameter);
    #[no_mangle]
    fn mailmime_subtype_free(subtype: *mut libc::c_char);
    #[no_mangle]
    fn mailmime_token_free(token: *mut libc::c_char);
    #[no_mangle]
    fn mailmime_type_new(
        tp_type: libc::c_int,
        tp_discrete_type: *mut mailmime_discrete_type,
        tp_composite_type: *mut mailmime_composite_type,
    ) -> *mut mailmime_type;
    #[no_mangle]
    fn mailmime_type_free(type_0: *mut mailmime_type);
    #[no_mangle]
    fn mailmime_value_free(value: *mut libc::c_char);
    #[no_mangle]
    fn mailmime_language_new(lg_list: *mut clist) -> *mut mailmime_language;
    #[no_mangle]
    fn mailmime_language_free(lang: *mut mailmime_language);
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
     * $Id: mailmime_decode.h,v 1.14 2008/02/20 22:15:52 hoa Exp $
     */
    #[no_mangle]
    fn mailmime_encoded_phrase_parse(
        default_fromcode: *const libc::c_char,
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
        tocode: *const libc::c_char,
        result: *mut *mut libc::c_char,
    ) -> libc::c_int;
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
     * $Id: mailmime_disposition.h,v 1.11 2008/02/20 22:15:52 hoa Exp $
     */
    #[no_mangle]
    fn mailmime_disposition_parse(
        message: *const libc::c_char,
        length: size_t,
        indx: *mut size_t,
        result: *mut *mut mailmime_disposition,
    ) -> libc::c_int;
    #[no_mangle]
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    #[no_mangle]
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    #[no_mangle]
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    #[no_mangle]
    fn free(_: *mut libc::c_void);
    #[no_mangle]
    fn strncasecmp(_: *const libc::c_char, _: *const libc::c_char, _: libc::c_ulong)
        -> libc::c_int;
}
pub type __darwin_ct_rune_t = libc::c_int;
pub type __darwin_size_t = libc::c_ulong;
pub type size_t = __darwin_size_t;
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
 * $Id: mailimf_types.h,v 1.34 2006/05/22 13:39:42 hoa Exp $
 */
/*
  IMPORTANT NOTE:

  All allocation functions will take as argument allocated data
  and will store these data in the structure they will allocate.
  Data should be persistant during all the use of the structure
  and will be freed by the free function of the structure

  allocation functions will return NULL on failure
*/
/*
  mailimf_date_time is a date

  - day is the day of month (1 to 31)

  - month (1 to 12)

  - year (4 digits)

  - hour (0 to 23)

  - min (0 to 59)

  - sec (0 to 59)

  - zone (this is the decimal value that we can read, for example:
    for "-0200", the value is -200)
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_date_time {
    pub dt_day: libc::c_int,
    pub dt_month: libc::c_int,
    pub dt_year: libc::c_int,
    pub dt_hour: libc::c_int,
    pub dt_min: libc::c_int,
    pub dt_sec: libc::c_int,
    pub dt_zone: libc::c_int,
}
/*
  mailimf_mailbox_list is a list of mailboxes

  - list is a list of mailboxes
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_mailbox_list {
    pub mb_list: *mut clist,
}
/*
  mailimf_mailbox is a mailbox

  - display_name is the name that will be displayed for this mailbox,
    for example 'name' in '"name" <mailbox@domain>,
    should be allocated with malloc()

  - addr_spec is the mailbox, for example 'mailbox@domain'
    in '"name" <mailbox@domain>, should be allocated with malloc()
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_mailbox {
    pub mb_display_name: *mut libc::c_char,
    pub mb_addr_spec: *mut libc::c_char,
}
/*
  mailimf_address_list is a list of addresses

  - list is a list of addresses
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_address_list {
    pub ad_list: *mut clist,
}
/*
  mailimf_fields is a list of header fields

  - fld_list is a list of header fields
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_fields {
    pub fld_list: *mut clist,
}
/* this is a type of field */
pub type unnamed = libc::c_uint;
/* other field */
pub const MAILIMF_FIELD_OPTIONAL_FIELD: unnamed = 22;
/* Keywords */
pub const MAILIMF_FIELD_KEYWORDS: unnamed = 21;
/* Comments */
pub const MAILIMF_FIELD_COMMENTS: unnamed = 20;
/* Subject */
pub const MAILIMF_FIELD_SUBJECT: unnamed = 19;
/* References */
pub const MAILIMF_FIELD_REFERENCES: unnamed = 18;
/* In-Reply-To */
pub const MAILIMF_FIELD_IN_REPLY_TO: unnamed = 17;
/* Message-ID */
pub const MAILIMF_FIELD_MESSAGE_ID: unnamed = 16;
/* Bcc */
pub const MAILIMF_FIELD_BCC: unnamed = 15;
/* Cc */
pub const MAILIMF_FIELD_CC: unnamed = 14;
/* To */
pub const MAILIMF_FIELD_TO: unnamed = 13;
/* Reply-To */
pub const MAILIMF_FIELD_REPLY_TO: unnamed = 12;
/* Sender */
pub const MAILIMF_FIELD_SENDER: unnamed = 11;
/* From */
pub const MAILIMF_FIELD_FROM: unnamed = 10;
/* Date */
pub const MAILIMF_FIELD_ORIG_DATE: unnamed = 9;
/* Resent-Message-ID */
pub const MAILIMF_FIELD_RESENT_MSG_ID: unnamed = 8;
/* Resent-Bcc */
pub const MAILIMF_FIELD_RESENT_BCC: unnamed = 7;
/* Resent-Cc */
pub const MAILIMF_FIELD_RESENT_CC: unnamed = 6;
/* Resent-To */
pub const MAILIMF_FIELD_RESENT_TO: unnamed = 5;
/* Resent-Sender */
pub const MAILIMF_FIELD_RESENT_SENDER: unnamed = 4;
/* Resent-From */
pub const MAILIMF_FIELD_RESENT_FROM: unnamed = 3;
/* Resent-Date */
pub const MAILIMF_FIELD_RESENT_DATE: unnamed = 2;
/* Return-Path */
pub const MAILIMF_FIELD_RETURN_PATH: unnamed = 1;
/* on parse error */
pub const MAILIMF_FIELD_NONE: unnamed = 0;
/*
  mailimf_field is a field

  - fld_type is the type of the field

  - fld_data.fld_return_path is the parsed content of the Return-Path
    field if type is MAILIMF_FIELD_RETURN_PATH

  - fld_data.fld_resent_date is the parsed content of the Resent-Date field
    if type is MAILIMF_FIELD_RESENT_DATE

  - fld_data.fld_resent_from is the parsed content of the Resent-From field

  - fld_data.fld_resent_sender is the parsed content of the Resent-Sender field

  - fld_data.fld_resent_to is the parsed content of the Resent-To field

  - fld_data.fld_resent_cc is the parsed content of the Resent-Cc field

  - fld_data.fld_resent_bcc is the parsed content of the Resent-Bcc field

  - fld_data.fld_resent_msg_id is the parsed content of the Resent-Message-ID
    field

  - fld_data.fld_orig_date is the parsed content of the Date field

  - fld_data.fld_from is the parsed content of the From field

  - fld_data.fld_sender is the parsed content of the Sender field

  - fld_data.fld_reply_to is the parsed content of the Reply-To field

  - fld_data.fld_to is the parsed content of the To field

  - fld_data.fld_cc is the parsed content of the Cc field

  - fld_data.fld_bcc is the parsed content of the Bcc field

  - fld_data.fld_message_id is the parsed content of the Message-ID field

  - fld_data.fld_in_reply_to is the parsed content of the In-Reply-To field

  - fld_data.fld_references is the parsed content of the References field

  - fld_data.fld_subject is the content of the Subject field

  - fld_data.fld_comments is the content of the Comments field

  - fld_data.fld_keywords is the parsed content of the Keywords field

  - fld_data.fld_optional_field is an other field and is not parsed
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_field {
    pub fld_type: libc::c_int,
    pub fld_data: unnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_0 {
    pub fld_return_path: *mut mailimf_return,
    pub fld_resent_date: *mut mailimf_orig_date,
    pub fld_resent_from: *mut mailimf_from,
    pub fld_resent_sender: *mut mailimf_sender,
    pub fld_resent_to: *mut mailimf_to,
    pub fld_resent_cc: *mut mailimf_cc,
    pub fld_resent_bcc: *mut mailimf_bcc,
    pub fld_resent_msg_id: *mut mailimf_message_id,
    pub fld_orig_date: *mut mailimf_orig_date,
    pub fld_from: *mut mailimf_from,
    pub fld_sender: *mut mailimf_sender,
    pub fld_reply_to: *mut mailimf_reply_to,
    pub fld_to: *mut mailimf_to,
    pub fld_cc: *mut mailimf_cc,
    pub fld_bcc: *mut mailimf_bcc,
    pub fld_message_id: *mut mailimf_message_id,
    pub fld_in_reply_to: *mut mailimf_in_reply_to,
    pub fld_references: *mut mailimf_references,
    pub fld_subject: *mut mailimf_subject,
    pub fld_comments: *mut mailimf_comments,
    pub fld_keywords: *mut mailimf_keywords,
    pub fld_optional_field: *mut mailimf_optional_field,
}
/*
  mailimf_optional_field is a non-parsed field

  - fld_name is the name of the field

  - fld_value is the value of the field
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_optional_field {
    pub fld_name: *mut libc::c_char,
    pub fld_value: *mut libc::c_char,
}
/*
  mailimf_keywords is the parsed Keywords field

  - kw_list is the list of keywords
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_keywords {
    pub kw_list: *mut clist,
}
/*
  mailimf_comments is the parsed Comments field

  - cm_value is the value of the field
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_comments {
    pub cm_value: *mut libc::c_char,
}
/*
  mailimf_subject is the parsed Subject field

  - sbj_value is the value of the field
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_subject {
    pub sbj_value: *mut libc::c_char,
}
/*
 mailimf_references is the parsed References field

 - msg_id_list is the list of message identifiers
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_references {
    pub mid_list: *mut clist,
}
/*
  mailimf_in_reply_to is the parsed In-Reply-To field

  - mid_list is the list of message identifers
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_in_reply_to {
    pub mid_list: *mut clist,
}
/*
  mailimf_message_id is the parsed Message-ID field

  - mid_value is the message identifier
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_message_id {
    pub mid_value: *mut libc::c_char,
}
/*
  mailimf_bcc is the parsed Bcc field

  - bcc_addr_list is the parsed addres list
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_bcc {
    pub bcc_addr_list: *mut mailimf_address_list,
}
/*
  mailimf_cc is the parsed Cc field

  - cc_addr_list is the parsed addres list
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_cc {
    pub cc_addr_list: *mut mailimf_address_list,
}
/*
  mailimf_to is the parsed To field

  - to_addr_list is the parsed address list
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_to {
    pub to_addr_list: *mut mailimf_address_list,
}
/*
 mailimf_reply_to is the parsed Reply-To field

 - rt_addr_list is the parsed address list
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_reply_to {
    pub rt_addr_list: *mut mailimf_address_list,
}
/*
  mailimf_sender is the parsed Sender field

  - snd_mb is the parsed mailbox
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_sender {
    pub snd_mb: *mut mailimf_mailbox,
}
/*
  mailimf_from is the parsed From field

  - mb_list is the parsed mailbox list
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_from {
    pub frm_mb_list: *mut mailimf_mailbox_list,
}
/*
  mailimf_orig_date is the parsed Date field

  - date_time is the parsed date
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_orig_date {
    pub dt_date_time: *mut mailimf_date_time,
}
/*
  mailimf_return is the parsed Return-Path field

  - ret_path is the parsed value of Return-Path
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_return {
    pub ret_path: *mut mailimf_path,
}
/*
  mailimf_path is the parsed value of Return-Path

  - pt_addr_spec is a mailbox
*/
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailimf_path {
    pub pt_addr_spec: *mut libc::c_char,
}
/* these are the possible returned error codes */
pub type unnamed_1 = libc::c_uint;
pub const MAILIMF_ERROR_FILE: unnamed_1 = 4;
pub const MAILIMF_ERROR_INVAL: unnamed_1 = 3;
pub const MAILIMF_ERROR_MEMORY: unnamed_1 = 2;
pub const MAILIMF_ERROR_PARSE: unnamed_1 = 1;
pub const MAILIMF_NO_ERROR: unnamed_1 = 0;
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
pub type unnamed_2 = libc::c_uint;
pub const MAILMIME_COMPOSITE_TYPE_EXTENSION: unnamed_2 = 3;
pub const MAILMIME_COMPOSITE_TYPE_MULTIPART: unnamed_2 = 2;
pub const MAILMIME_COMPOSITE_TYPE_MESSAGE: unnamed_2 = 1;
pub const MAILMIME_COMPOSITE_TYPE_ERROR: unnamed_2 = 0;
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
    pub tp_data: unnamed_3,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_3 {
    pub tp_discrete_type: *mut mailmime_discrete_type,
    pub tp_composite_type: *mut mailmime_composite_type,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_discrete_type {
    pub dt_type: libc::c_int,
    pub dt_extension: *mut libc::c_char,
}
pub type unnamed_4 = libc::c_uint;
pub const MAILMIME_DISCRETE_TYPE_EXTENSION: unnamed_4 = 6;
pub const MAILMIME_DISCRETE_TYPE_APPLICATION: unnamed_4 = 5;
pub const MAILMIME_DISCRETE_TYPE_VIDEO: unnamed_4 = 4;
pub const MAILMIME_DISCRETE_TYPE_AUDIO: unnamed_4 = 3;
pub const MAILMIME_DISCRETE_TYPE_IMAGE: unnamed_4 = 2;
pub const MAILMIME_DISCRETE_TYPE_TEXT: unnamed_4 = 1;
pub const MAILMIME_DISCRETE_TYPE_ERROR: unnamed_4 = 0;
pub type unnamed_5 = libc::c_uint;
pub const MAILMIME_FIELD_LOCATION: unnamed_5 = 8;
pub const MAILMIME_FIELD_LANGUAGE: unnamed_5 = 7;
pub const MAILMIME_FIELD_DISPOSITION: unnamed_5 = 6;
pub const MAILMIME_FIELD_VERSION: unnamed_5 = 5;
pub const MAILMIME_FIELD_DESCRIPTION: unnamed_5 = 4;
pub const MAILMIME_FIELD_ID: unnamed_5 = 3;
pub const MAILMIME_FIELD_TRANSFER_ENCODING: unnamed_5 = 2;
pub const MAILMIME_FIELD_TYPE: unnamed_5 = 1;
pub const MAILMIME_FIELD_NONE: unnamed_5 = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mailmime_field {
    pub fld_type: libc::c_int,
    pub fld_data: unnamed_6,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union unnamed_6 {
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
pub type unnamed_7 = libc::c_uint;
pub const MAILMIME_MECHANISM_TOKEN: unnamed_7 = 6;
pub const MAILMIME_MECHANISM_BASE64: unnamed_7 = 5;
pub const MAILMIME_MECHANISM_QUOTED_PRINTABLE: unnamed_7 = 4;
pub const MAILMIME_MECHANISM_BINARY: unnamed_7 = 3;
pub const MAILMIME_MECHANISM_8BIT: unnamed_7 = 2;
pub const MAILMIME_MECHANISM_7BIT: unnamed_7 = 1;
pub const MAILMIME_MECHANISM_ERROR: unnamed_7 = 0;
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
pub type unnamed_8 = libc::c_uint;
pub const MAILMIME_TYPE_COMPOSITE_TYPE: unnamed_8 = 2;
pub const MAILMIME_TYPE_DISCRETE_TYPE: unnamed_8 = 1;
pub const MAILMIME_TYPE_ERROR: unnamed_8 = 0;
pub const FIELD_STATE_L: unnamed_9 = 3;
pub const FIELD_STATE_D: unnamed_9 = 2;
pub const FIELD_STATE_T: unnamed_9 = 1;
pub const FIELD_STATE_START: unnamed_9 = 0;
/*
x  entity-headers := [ content CRLF ]
                    [ encoding CRLF ]
                    [ id CRLF ]
                    [ description CRLF ]
                    *( MIME-extension-field CRLF )
            */
pub type unnamed_9 = libc::c_uint;
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
pub unsafe extern "C" fn mailmime_content_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut mailmime_content,
) -> libc::c_int {
    let mut current_block: u64;
    let mut cur_token: size_t = 0;
    let mut type_0: *mut mailmime_type = 0 as *mut mailmime_type;
    let mut subtype: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut parameters_list: *mut clist = 0 as *mut clist;
    let mut content: *mut mailmime_content = 0 as *mut mailmime_content;
    let mut r: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    cur_token = *indx;
    mailimf_cfws_parse(message, length, &mut cur_token);
    type_0 = 0 as *mut mailmime_type;
    r = mailmime_type_parse(message, length, &mut cur_token, &mut type_0);
    if r != MAILIMF_NO_ERROR as libc::c_int {
        res = r
    } else {
        r = mailimf_unstrict_char_parse(
            message,
            length,
            &mut cur_token,
            '/' as i32 as libc::c_char,
        );
        match r {
            0 => {
                r = mailimf_cfws_parse(message, length, &mut cur_token);
                if r != MAILIMF_NO_ERROR as libc::c_int && r != MAILIMF_ERROR_PARSE as libc::c_int {
                    res = r;
                    current_block = 10242373397628622958;
                } else {
                    r = mailmime_subtype_parse(message, length, &mut cur_token, &mut subtype);
                    if r != MAILIMF_NO_ERROR as libc::c_int {
                        res = r;
                        current_block = 10242373397628622958;
                    } else {
                        current_block = 1109700713171191020;
                    }
                }
            }
            1 => {
                subtype = strdup(b"unknown\x00" as *const u8 as *const libc::c_char);
                current_block = 1109700713171191020;
            }
            _ => {
                res = r;
                current_block = 10242373397628622958;
            }
        }
        match current_block {
            1109700713171191020 => {
                parameters_list = clist_new();
                if parameters_list.is_null() {
                    res = MAILIMF_ERROR_MEMORY as libc::c_int
                } else {
                    loop {
                        let mut final_token: size_t = 0;
                        let mut parameter: *mut mailmime_parameter = 0 as *mut mailmime_parameter;
                        final_token = cur_token;
                        r = mailimf_unstrict_char_parse(
                            message,
                            length,
                            &mut cur_token,
                            ';' as i32 as libc::c_char,
                        );
                        if r != MAILIMF_NO_ERROR as libc::c_int {
                            cur_token = final_token;
                            current_block = 12497913735442871383;
                            break;
                        } else {
                            r = mailimf_cfws_parse(message, length, &mut cur_token);
                            if r != MAILIMF_NO_ERROR as libc::c_int
                                && r != MAILIMF_ERROR_PARSE as libc::c_int
                            {
                                res = r;
                                current_block = 6276274620003476740;
                                break;
                            } else {
                                r = mailmime_parameter_parse(
                                    message,
                                    length,
                                    &mut cur_token,
                                    &mut parameter,
                                );
                                if r == MAILIMF_NO_ERROR as libc::c_int {
                                    r = clist_insert_after(
                                        parameters_list,
                                        (*parameters_list).last,
                                        parameter as *mut libc::c_void,
                                    );
                                    if !(r < 0i32) {
                                        continue;
                                    }
                                    mailmime_parameter_free(parameter);
                                    res = MAILIMF_ERROR_MEMORY as libc::c_int;
                                    current_block = 5731074241326334034;
                                    break;
                                } else if r == MAILIMF_ERROR_PARSE as libc::c_int {
                                    cur_token = final_token;
                                    current_block = 12497913735442871383;
                                    break;
                                } else {
                                    res = r;
                                    current_block = 6276274620003476740;
                                    break;
                                }
                            }
                        }
                    }
                    match current_block {
                        6276274620003476740 => {}
                        _ => {
                            match current_block {
                                12497913735442871383 => {
                                    content =
                                        mailmime_content_new(type_0, subtype, parameters_list);
                                    if content.is_null() {
                                        res = MAILIMF_ERROR_MEMORY as libc::c_int
                                    } else {
                                        *result = content;
                                        *indx = cur_token;
                                        return MAILIMF_NO_ERROR as libc::c_int;
                                    }
                                }
                                _ => {}
                            }
                            clist_foreach(
                                parameters_list,
                                ::std::mem::transmute::<
                                    Option<unsafe extern "C" fn(_: *mut mailmime_parameter) -> ()>,
                                    clist_func,
                                >(Some(mailmime_parameter_free)),
                                0 as *mut libc::c_void,
                            );
                            clist_free(parameters_list);
                        }
                    }
                }
                mailmime_subtype_free(subtype);
            }
            _ => {}
        }
        mailmime_type_free(type_0);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_parameter_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut mailmime_parameter,
) -> libc::c_int {
    let mut attribute: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut parameter: *mut mailmime_parameter = 0 as *mut mailmime_parameter;
    let mut cur_token: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    cur_token = *indx;
    r = mailmime_attribute_parse(message, length, &mut cur_token, &mut attribute);
    if r != MAILIMF_NO_ERROR as libc::c_int {
        res = r
    } else {
        r = mailimf_unstrict_char_parse(
            message,
            length,
            &mut cur_token,
            '=' as i32 as libc::c_char,
        );
        if r != MAILIMF_NO_ERROR as libc::c_int {
            res = r
        } else {
            r = mailimf_cfws_parse(message, length, &mut cur_token);
            if r != MAILIMF_NO_ERROR as libc::c_int && r != MAILIMF_ERROR_PARSE as libc::c_int {
                res = r
            } else {
                r = mailmime_value_parse(message, length, &mut cur_token, &mut value);
                if r != MAILIMF_NO_ERROR as libc::c_int {
                    res = r
                } else {
                    parameter = mailmime_parameter_new(attribute, value);
                    if parameter.is_null() {
                        res = MAILIMF_ERROR_MEMORY as libc::c_int;
                        mailmime_value_free(value);
                    } else {
                        *result = parameter;
                        *indx = cur_token;
                        return MAILIMF_NO_ERROR as libc::c_int;
                    }
                }
            }
        }
        mailmime_attribute_free(attribute);
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_value_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut r: libc::c_int = 0;
    r = mailimf_atom_parse(message, length, indx, result);
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailimf_quoted_string_parse(message, length, indx, result)
    }
    if r != MAILIMF_NO_ERROR as libc::c_int {
        return r;
    }
    return MAILIMF_NO_ERROR as libc::c_int;
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
 * $Id: mailmime.c,v 1.29 2011/01/06 00:09:52 hoa Exp $
 */
/*
 RFC 2045
 RFC 2046
 RFC 2047
 RFC 2048
 RFC 2049
 RFC 2231
 RFC 2387
 RFC 2424
 RFC 2557

 RFC 2183 Content-Disposition

 RFC 1766  Language
*/
unsafe extern "C" fn mailmime_attribute_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut libc::c_char,
) -> libc::c_int {
    return mailmime_token_parse(message, length, indx, result);
}
unsafe extern "C" fn mailmime_token_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut token: *mut *mut libc::c_char,
) -> libc::c_int {
    return mailimf_custom_string_parse(message, length, indx, token, Some(is_token));
}
unsafe extern "C" fn is_token(mut ch: libc::c_char) -> libc::c_int {
    let mut uch: libc::c_uchar = ch as libc::c_uchar;
    if uch as libc::c_int > 0x7fi32 {
        return 0i32;
    }
    if uch as libc::c_int == ' ' as i32 {
        return 0i32;
    }
    if 0 != is_tspecials(ch) {
        return 0i32;
    }
    return 1i32;
}
unsafe extern "C" fn is_tspecials(mut ch: libc::c_char) -> libc::c_int {
    match ch as libc::c_int {
        40 | 41 | 60 | 62 | 64 | 44 | 59 | 58 | 92 | 34 | 47 | 91 | 93 | 63 | 61 => return 1i32,
        _ => return 0i32,
    };
}
unsafe extern "C" fn mailmime_subtype_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut libc::c_char,
) -> libc::c_int {
    return mailmime_extension_token_parse(message, length, indx, result);
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_extension_token_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut libc::c_char,
) -> libc::c_int {
    return mailmime_token_parse(message, length, indx, result);
}
unsafe extern "C" fn mailmime_type_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut mailmime_type,
) -> libc::c_int {
    let mut discrete_type: *mut mailmime_discrete_type = 0 as *mut mailmime_discrete_type;
    let mut composite_type: *mut mailmime_composite_type = 0 as *mut mailmime_composite_type;
    let mut cur_token: size_t = 0;
    let mut mime_type: *mut mailmime_type = 0 as *mut mailmime_type;
    let mut type_0: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    cur_token = *indx;
    discrete_type = 0 as *mut mailmime_discrete_type;
    composite_type = 0 as *mut mailmime_composite_type;
    type_0 = MAILMIME_TYPE_ERROR as libc::c_int;
    r = mailmime_composite_type_parse(message, length, &mut cur_token, &mut composite_type);
    if r == MAILIMF_NO_ERROR as libc::c_int {
        type_0 = MAILMIME_TYPE_COMPOSITE_TYPE as libc::c_int
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailmime_discrete_type_parse(message, length, &mut cur_token, &mut discrete_type);
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_TYPE_DISCRETE_TYPE as libc::c_int
        }
    }
    if r != MAILIMF_NO_ERROR as libc::c_int {
        res = r
    } else {
        mime_type = mailmime_type_new(type_0, discrete_type, composite_type);
        if mime_type.is_null() {
            res = r;
            if !discrete_type.is_null() {
                mailmime_discrete_type_free(discrete_type);
            }
            if !composite_type.is_null() {
                mailmime_composite_type_free(composite_type);
            }
        } else {
            *result = mime_type;
            *indx = cur_token;
            return MAILIMF_NO_ERROR as libc::c_int;
        }
    }
    return res;
}
unsafe extern "C" fn mailmime_discrete_type_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut mailmime_discrete_type,
) -> libc::c_int {
    let mut extension: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: libc::c_int = 0;
    let mut discrete_type: *mut mailmime_discrete_type = 0 as *mut mailmime_discrete_type;
    let mut cur_token: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    cur_token = *indx;
    extension = 0 as *mut libc::c_char;
    type_0 = MAILMIME_DISCRETE_TYPE_ERROR as libc::c_int;
    r = mailimf_token_case_insensitive_len_parse(
        message,
        length,
        &mut cur_token,
        b"text\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        strlen(b"text\x00" as *const u8 as *const libc::c_char),
    );
    if r == MAILIMF_NO_ERROR as libc::c_int {
        type_0 = MAILMIME_DISCRETE_TYPE_TEXT as libc::c_int
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailimf_token_case_insensitive_len_parse(
            message,
            length,
            &mut cur_token,
            b"image\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            strlen(b"image\x00" as *const u8 as *const libc::c_char),
        );
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_DISCRETE_TYPE_IMAGE as libc::c_int
        }
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailimf_token_case_insensitive_len_parse(
            message,
            length,
            &mut cur_token,
            b"audio\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            strlen(b"audio\x00" as *const u8 as *const libc::c_char),
        );
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_DISCRETE_TYPE_AUDIO as libc::c_int
        }
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailimf_token_case_insensitive_len_parse(
            message,
            length,
            &mut cur_token,
            b"video\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            strlen(b"video\x00" as *const u8 as *const libc::c_char),
        );
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_DISCRETE_TYPE_VIDEO as libc::c_int
        }
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailimf_token_case_insensitive_len_parse(
            message,
            length,
            &mut cur_token,
            b"application\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            strlen(b"application\x00" as *const u8 as *const libc::c_char),
        );
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_DISCRETE_TYPE_APPLICATION as libc::c_int
        }
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailmime_extension_token_parse(message, length, &mut cur_token, &mut extension);
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_DISCRETE_TYPE_EXTENSION as libc::c_int
        }
    }
    if r != MAILIMF_NO_ERROR as libc::c_int {
        res = r
    } else {
        discrete_type = mailmime_discrete_type_new(type_0, extension);
        if discrete_type.is_null() {
            res = MAILIMF_ERROR_MEMORY as libc::c_int;
            mailmime_extension_token_free(extension);
        } else {
            *result = discrete_type;
            *indx = cur_token;
            return MAILIMF_NO_ERROR as libc::c_int;
        }
    }
    return res;
}
unsafe extern "C" fn mailmime_composite_type_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut mailmime_composite_type,
) -> libc::c_int {
    let mut extension_token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: libc::c_int = 0;
    let mut ct: *mut mailmime_composite_type = 0 as *mut mailmime_composite_type;
    let mut cur_token: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    cur_token = *indx;
    extension_token = 0 as *mut libc::c_char;
    type_0 = MAILMIME_COMPOSITE_TYPE_ERROR as libc::c_int;
    r = mailimf_token_case_insensitive_len_parse(
        message,
        length,
        &mut cur_token,
        b"message\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        strlen(b"message\x00" as *const u8 as *const libc::c_char),
    );
    if r == MAILIMF_NO_ERROR as libc::c_int {
        type_0 = MAILMIME_COMPOSITE_TYPE_MESSAGE as libc::c_int
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailimf_token_case_insensitive_len_parse(
            message,
            length,
            &mut cur_token,
            b"multipart\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            strlen(b"multipart\x00" as *const u8 as *const libc::c_char),
        );
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_COMPOSITE_TYPE_MULTIPART as libc::c_int
        }
    }
    if r != MAILIMF_NO_ERROR as libc::c_int {
        res = r
    } else {
        ct = mailmime_composite_type_new(type_0, extension_token);
        if ct.is_null() {
            res = MAILIMF_ERROR_MEMORY as libc::c_int;
            if !extension_token.is_null() {
                mailmime_extension_token_free(extension_token);
            }
        } else {
            *result = ct;
            *indx = cur_token;
            return MAILIMF_NO_ERROR as libc::c_int;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_description_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut libc::c_char,
) -> libc::c_int {
    return mailimf_custom_string_parse(message, length, indx, result, Some(is_text));
}
unsafe extern "C" fn is_text(mut ch: libc::c_char) -> libc::c_int {
    let mut uch: libc::c_uchar = ch as libc::c_uchar;
    if (uch as libc::c_int) < 1i32 {
        return 0i32;
    }
    if uch as libc::c_int == 10i32 || uch as libc::c_int == 13i32 {
        return 0i32;
    }
    return 1i32;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_location_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut libc::c_char,
) -> libc::c_int {
    return mailimf_custom_string_parse(message, length, indx, result, Some(is_text));
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_encoding_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut mailmime_mechanism,
) -> libc::c_int {
    return mailmime_mechanism_parse(message, length, indx, result);
}
unsafe extern "C" fn mailmime_mechanism_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut mailmime_mechanism,
) -> libc::c_int {
    let mut token: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut type_0: libc::c_int = 0;
    let mut mechanism: *mut mailmime_mechanism = 0 as *mut mailmime_mechanism;
    let mut cur_token: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    cur_token = *indx;
    type_0 = MAILMIME_MECHANISM_ERROR as libc::c_int;
    token = 0 as *mut libc::c_char;
    r = mailimf_token_case_insensitive_len_parse(
        message,
        length,
        &mut cur_token,
        b"7bit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
        strlen(b"7bit\x00" as *const u8 as *const libc::c_char),
    );
    if r == MAILIMF_NO_ERROR as libc::c_int {
        type_0 = MAILMIME_MECHANISM_7BIT as libc::c_int
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailimf_token_case_insensitive_len_parse(
            message,
            length,
            &mut cur_token,
            b"8bit\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            strlen(b"8bit\x00" as *const u8 as *const libc::c_char),
        );
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_MECHANISM_8BIT as libc::c_int
        }
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailimf_token_case_insensitive_len_parse(
            message,
            length,
            &mut cur_token,
            b"binary\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            strlen(b"binary\x00" as *const u8 as *const libc::c_char),
        );
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_MECHANISM_BINARY as libc::c_int
        }
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailimf_token_case_insensitive_len_parse(
            message,
            length,
            &mut cur_token,
            b"quoted-printable\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            strlen(b"quoted-printable\x00" as *const u8 as *const libc::c_char),
        );
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_MECHANISM_QUOTED_PRINTABLE as libc::c_int
        }
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailimf_token_case_insensitive_len_parse(
            message,
            length,
            &mut cur_token,
            b"base64\x00" as *const u8 as *const libc::c_char as *mut libc::c_char,
            strlen(b"base64\x00" as *const u8 as *const libc::c_char),
        );
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_MECHANISM_BASE64 as libc::c_int
        }
    }
    if r == MAILIMF_ERROR_PARSE as libc::c_int {
        r = mailmime_token_parse(message, length, &mut cur_token, &mut token);
        if r == MAILIMF_NO_ERROR as libc::c_int {
            type_0 = MAILMIME_MECHANISM_TOKEN as libc::c_int
        }
    }
    if r != MAILIMF_NO_ERROR as libc::c_int {
        res = r
    } else {
        mechanism = mailmime_mechanism_new(type_0, token);
        if mechanism.is_null() {
            res = MAILIMF_ERROR_MEMORY as libc::c_int;
            if !token.is_null() {
                mailmime_token_free(token);
            }
        } else {
            *result = mechanism;
            *indx = cur_token;
            return MAILIMF_NO_ERROR as libc::c_int;
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_field_parse(
    mut field: *mut mailimf_optional_field,
    mut result: *mut *mut mailmime_field,
) -> libc::c_int {
    let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut value: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut guessed_type: libc::c_int = 0;
    let mut cur_token: size_t = 0;
    let mut content: *mut mailmime_content = 0 as *mut mailmime_content;
    let mut encoding: *mut mailmime_mechanism = 0 as *mut mailmime_mechanism;
    let mut id: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut description: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut version: uint32_t = 0;
    let mut mime_field: *mut mailmime_field = 0 as *mut mailmime_field;
    let mut language: *mut mailmime_language = 0 as *mut mailmime_language;
    let mut disposition: *mut mailmime_disposition = 0 as *mut mailmime_disposition;
    let mut location: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut res: libc::c_int = 0;
    let mut r: libc::c_int = 0;
    name = (*field).fld_name;
    value = (*field).fld_value;
    cur_token = 0i32 as size_t;
    content = 0 as *mut mailmime_content;
    encoding = 0 as *mut mailmime_mechanism;
    id = 0 as *mut libc::c_char;
    description = 0 as *mut libc::c_char;
    version = 0i32 as uint32_t;
    disposition = 0 as *mut mailmime_disposition;
    language = 0 as *mut mailmime_language;
    location = 0 as *mut libc::c_char;
    guessed_type = guess_field_type(name);
    match guessed_type {
        1 => {
            if strcasecmp(
                name,
                b"Content-Type\x00" as *const u8 as *const libc::c_char,
            ) != 0i32
            {
                return MAILIMF_ERROR_PARSE as libc::c_int;
            }
            let mut cur_token_0: size_t = 0i32 as size_t;
            let mut decoded_value: *mut libc::c_char = 0 as *mut libc::c_char;
            r = mailmime_encoded_phrase_parse(
                b"us-ascii\x00" as *const u8 as *const libc::c_char,
                value,
                strlen(value),
                &mut cur_token_0,
                b"utf-8\x00" as *const u8 as *const libc::c_char,
                &mut decoded_value,
            );
            if r != MAILIMF_NO_ERROR as libc::c_int {
                cur_token_0 = 0i32 as size_t;
                r = mailmime_content_parse(value, strlen(value), &mut cur_token_0, &mut content)
            } else {
                cur_token_0 = 0i32 as size_t;
                r = mailmime_content_parse(
                    decoded_value,
                    strlen(decoded_value),
                    &mut cur_token_0,
                    &mut content,
                );
                free(decoded_value as *mut libc::c_void);
            }
            if r != MAILIMF_NO_ERROR as libc::c_int {
                return r;
            }
        }
        2 => {
            if strcasecmp(
                name,
                b"Content-Transfer-Encoding\x00" as *const u8 as *const libc::c_char,
            ) != 0i32
            {
                return MAILIMF_ERROR_PARSE as libc::c_int;
            }
            r = mailmime_encoding_parse(value, strlen(value), &mut cur_token, &mut encoding);
            if r != MAILIMF_NO_ERROR as libc::c_int {
                return r;
            }
        }
        3 => {
            if strcasecmp(name, b"Content-ID\x00" as *const u8 as *const libc::c_char) != 0i32 {
                return MAILIMF_ERROR_PARSE as libc::c_int;
            }
            r = mailmime_id_parse(value, strlen(value), &mut cur_token, &mut id);
            if r != MAILIMF_NO_ERROR as libc::c_int {
                return r;
            }
        }
        4 => {
            if strcasecmp(
                name,
                b"Content-Description\x00" as *const u8 as *const libc::c_char,
            ) != 0i32
            {
                return MAILIMF_ERROR_PARSE as libc::c_int;
            }
            r = mailmime_description_parse(value, strlen(value), &mut cur_token, &mut description);
            if r != MAILIMF_NO_ERROR as libc::c_int {
                return r;
            }
        }
        5 => {
            if strcasecmp(
                name,
                b"MIME-Version\x00" as *const u8 as *const libc::c_char,
            ) != 0i32
            {
                return MAILIMF_ERROR_PARSE as libc::c_int;
            }
            r = mailmime_version_parse(value, strlen(value), &mut cur_token, &mut version);
            if r != MAILIMF_NO_ERROR as libc::c_int {
                return r;
            }
        }
        6 => {
            if strcasecmp(
                name,
                b"Content-Disposition\x00" as *const u8 as *const libc::c_char,
            ) != 0i32
            {
                return MAILIMF_ERROR_PARSE as libc::c_int;
            }
            r = mailmime_disposition_parse(value, strlen(value), &mut cur_token, &mut disposition);
            if r != MAILIMF_NO_ERROR as libc::c_int {
                return r;
            }
        }
        7 => {
            if strcasecmp(
                name,
                b"Content-Language\x00" as *const u8 as *const libc::c_char,
            ) != 0i32
            {
                return MAILIMF_ERROR_PARSE as libc::c_int;
            }
            r = mailmime_language_parse(value, strlen(value), &mut cur_token, &mut language);
            if r != MAILIMF_NO_ERROR as libc::c_int {
                return r;
            }
        }
        8 => {
            if strcasecmp(
                name,
                b"Content-Location\x00" as *const u8 as *const libc::c_char,
            ) != 0i32
            {
                return MAILIMF_ERROR_PARSE as libc::c_int;
            }
            r = mailmime_location_parse(value, strlen(value), &mut cur_token, &mut location);
            if r != MAILIMF_NO_ERROR as libc::c_int {
                return r;
            }
        }
        _ => return MAILIMF_ERROR_PARSE as libc::c_int,
    }
    mime_field = mailmime_field_new(
        guessed_type,
        content,
        encoding,
        id,
        description,
        version,
        disposition,
        language,
        location,
    );
    if mime_field.is_null() {
        res = MAILIMF_ERROR_MEMORY as libc::c_int;
        if !location.is_null() {
            mailmime_location_free(location);
        }
        if !language.is_null() {
            mailmime_language_free(language);
        }
        if !content.is_null() {
            mailmime_content_free(content);
        }
        if !encoding.is_null() {
            mailmime_encoding_free(encoding);
        }
        if !id.is_null() {
            mailmime_id_free(id);
        }
        if !description.is_null() {
            mailmime_description_free(description);
        }
        return res;
    } else {
        *result = mime_field;
        return MAILIMF_NO_ERROR as libc::c_int;
    };
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_language_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut mailmime_language,
) -> libc::c_int {
    let mut current_block: u64;
    let mut cur_token: size_t = 0;
    let mut r: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    let mut list: *mut clist = 0 as *mut clist;
    let mut language: *mut mailmime_language = 0 as *mut mailmime_language;
    cur_token = *indx;
    list = clist_new();
    if list.is_null() {
        res = MAILIMF_ERROR_MEMORY as libc::c_int
    } else {
        loop {
            let mut atom: *mut libc::c_char = 0 as *mut libc::c_char;
            r = mailimf_unstrict_char_parse(
                message,
                length,
                &mut cur_token,
                ',' as i32 as libc::c_char,
            );
            if r == MAILIMF_NO_ERROR as libc::c_int {
                r = mailimf_atom_parse(message, length, &mut cur_token, &mut atom);
                if r == MAILIMF_NO_ERROR as libc::c_int {
                    r = clist_insert_after(list, (*list).last, atom as *mut libc::c_void);
                    if !(r < 0i32) {
                        continue;
                    }
                    mailimf_atom_free(atom);
                    res = MAILIMF_ERROR_MEMORY as libc::c_int;
                    current_block = 14533943604180559553;
                    break;
                } else {
                    /* do nothing */
                    if r == MAILIMF_ERROR_PARSE as libc::c_int {
                        current_block = 6669252993407410313;
                        break;
                    }
                    res = r;
                    current_block = 11601180562230609130;
                    break;
                }
            } else {
                /* do nothing */
                if r == MAILIMF_ERROR_PARSE as libc::c_int {
                    current_block = 6669252993407410313;
                    break;
                }
                res = r;
                current_block = 11601180562230609130;
                break;
            }
        }
        match current_block {
            11601180562230609130 => {}
            _ => {
                match current_block {
                    6669252993407410313 => {
                        language = mailmime_language_new(list);
                        if language.is_null() {
                            res = MAILIMF_ERROR_MEMORY as libc::c_int
                        } else {
                            *result = language;
                            *indx = cur_token;
                            return MAILIMF_NO_ERROR as libc::c_int;
                        }
                    }
                    _ => {}
                }
                clist_foreach(
                    list,
                    ::std::mem::transmute::<
                        Option<unsafe extern "C" fn(_: *mut libc::c_char) -> ()>,
                        clist_func,
                    >(Some(mailimf_atom_free)),
                    0 as *mut libc::c_void,
                );
                clist_free(list);
            }
        }
    }
    return res;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_version_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut uint32_t,
) -> libc::c_int {
    let mut cur_token: size_t = 0;
    let mut hi: uint32_t = 0;
    let mut low: uint32_t = 0;
    let mut version: uint32_t = 0;
    let mut r: libc::c_int = 0;
    cur_token = *indx;
    r = mailimf_number_parse(message, length, &mut cur_token, &mut hi);
    if r != MAILIMF_NO_ERROR as libc::c_int {
        return r;
    }
    r = mailimf_unstrict_char_parse(message, length, &mut cur_token, '.' as i32 as libc::c_char);
    if r != MAILIMF_NO_ERROR as libc::c_int {
        return r;
    }
    r = mailimf_cfws_parse(message, length, &mut cur_token);
    if r != MAILIMF_NO_ERROR as libc::c_int && r != MAILIMF_ERROR_PARSE as libc::c_int {
        return r;
    }
    r = mailimf_number_parse(message, length, &mut cur_token, &mut low);
    if r != MAILIMF_NO_ERROR as libc::c_int {
        return r;
    }
    version = (hi << 16i32).wrapping_add(low);
    *result = version;
    *indx = cur_token;
    return MAILIMF_NO_ERROR as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_id_parse(
    mut message: *const libc::c_char,
    mut length: size_t,
    mut indx: *mut size_t,
    mut result: *mut *mut libc::c_char,
) -> libc::c_int {
    return mailimf_msg_id_parse(message, length, indx, result);
}
unsafe extern "C" fn guess_field_type(mut name: *mut libc::c_char) -> libc::c_int {
    let mut state: libc::c_int = 0;
    if *name as libc::c_int == 'M' as i32 {
        return MAILMIME_FIELD_VERSION as libc::c_int;
    }
    if strncasecmp(
        name,
        b"Content-\x00" as *const u8 as *const libc::c_char,
        8i32 as libc::c_ulong,
    ) != 0i32
    {
        return MAILMIME_FIELD_NONE as libc::c_int;
    }
    name = name.offset(8isize);
    state = FIELD_STATE_START as libc::c_int;
    loop {
        match state {
            0 => {
                match toupper(*name as libc::c_uchar as libc::c_int) as libc::c_char as libc::c_int
                {
                    84 => state = FIELD_STATE_T as libc::c_int,
                    73 => return MAILMIME_FIELD_ID as libc::c_int,
                    68 => state = FIELD_STATE_D as libc::c_int,
                    76 => state = FIELD_STATE_L as libc::c_int,
                    _ => return MAILMIME_FIELD_NONE as libc::c_int,
                }
            }
            1 => {
                match toupper(*name as libc::c_uchar as libc::c_int) as libc::c_char as libc::c_int
                {
                    89 => return MAILMIME_FIELD_TYPE as libc::c_int,
                    82 => return MAILMIME_FIELD_TRANSFER_ENCODING as libc::c_int,
                    _ => return MAILMIME_FIELD_NONE as libc::c_int,
                }
            }
            2 => {
                match toupper(*name as libc::c_uchar as libc::c_int) as libc::c_char as libc::c_int
                {
                    69 => return MAILMIME_FIELD_DESCRIPTION as libc::c_int,
                    73 => return MAILMIME_FIELD_DISPOSITION as libc::c_int,
                    _ => return MAILMIME_FIELD_NONE as libc::c_int,
                }
            }
            3 => {
                match toupper(*name as libc::c_uchar as libc::c_int) as libc::c_char as libc::c_int
                {
                    65 => return MAILMIME_FIELD_LANGUAGE as libc::c_int,
                    79 => return MAILMIME_FIELD_LOCATION as libc::c_int,
                    _ => return MAILMIME_FIELD_NONE as libc::c_int,
                }
            }
            _ => {}
        }
        name = name.offset(1isize)
    }
}
#[no_mangle]
#[inline]
pub unsafe extern "C" fn toupper(mut _c: libc::c_int) -> libc::c_int {
    return __toupper(_c);
}
#[no_mangle]
pub unsafe extern "C" fn mailmime_fields_parse(
    mut fields: *mut mailimf_fields,
    mut result: *mut *mut mailmime_fields,
) -> libc::c_int {
    let mut current_block: u64;
    let mut cur: *mut clistiter = 0 as *mut clistiter;
    let mut mime_fields: *mut mailmime_fields = 0 as *mut mailmime_fields;
    let mut list: *mut clist = 0 as *mut clist;
    let mut r: libc::c_int = 0;
    let mut res: libc::c_int = 0;
    list = clist_new();
    if list.is_null() {
        res = MAILIMF_ERROR_MEMORY as libc::c_int
    } else {
        cur = (*(*fields).fld_list).first;
        loop {
            if cur.is_null() {
                current_block = 1109700713171191020;
                break;
            }
            let mut field: *mut mailimf_field = 0 as *mut mailimf_field;
            let mut mime_field: *mut mailmime_field = 0 as *mut mailmime_field;
            field = (if !cur.is_null() {
                (*cur).data
            } else {
                0 as *mut libc::c_void
            }) as *mut mailimf_field;
            if (*field).fld_type == MAILIMF_FIELD_OPTIONAL_FIELD as libc::c_int {
                r = mailmime_field_parse((*field).fld_data.fld_optional_field, &mut mime_field);
                if r == MAILIMF_NO_ERROR as libc::c_int {
                    r = clist_insert_after(list, (*list).last, mime_field as *mut libc::c_void);
                    if r < 0i32 {
                        mailmime_field_free(mime_field);
                        res = MAILIMF_ERROR_MEMORY as libc::c_int;
                        current_block = 17592539310030730040;
                        break;
                    }
                } else if !(r == MAILIMF_ERROR_PARSE as libc::c_int) {
                    /* do nothing */
                    res = r;
                    current_block = 17592539310030730040;
                    break;
                }
            }
            cur = if !cur.is_null() {
                (*cur).next
            } else {
                0 as *mut clistcell_s
            }
        }
        match current_block {
            1109700713171191020 => {
                if (*list).first.is_null() {
                    res = MAILIMF_ERROR_PARSE as libc::c_int
                } else {
                    mime_fields = mailmime_fields_new(list);
                    if mime_fields.is_null() {
                        res = MAILIMF_ERROR_MEMORY as libc::c_int
                    } else {
                        *result = mime_fields;
                        return MAILIMF_NO_ERROR as libc::c_int;
                    }
                }
            }
            _ => {}
        }
        clist_foreach(
            list,
            ::std::mem::transmute::<
                Option<unsafe extern "C" fn(_: *mut mailmime_field) -> ()>,
                clist_func,
            >(Some(mailmime_field_free)),
            0 as *mut libc::c_void,
        );
        clist_free(list);
    }
    return res;
}
