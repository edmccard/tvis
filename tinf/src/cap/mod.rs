#![allow(non_upper_case_globals)]

//! Terminfo capability names.
//!
//! The names for predefined capbilities are used as indices into a
//! [`Desc`](../struct.Desc.html).
//!
//! [`Boolean`](struct.Boolean.html), [`Number`](struct.Number.html), and
//! [`String`](struct.String.html) contain the complete list of names
//! (and short names) for each type of capability: they are the same
//! as those used by the ncurses library, with the following
//! replacements:
//!
//! - `in_` replaces `in`
//! - `if_` replaces `if`
//! - `OTbs_b` replaces boolean `OTbs`
//! - `OTbs_s` replaces string `OTbs`

use std::borrow::Borrow;
use std::ops::Index;
use std::string::String as StdString;

mod names;
#[doc(hidden)]
pub use self::names::*;

/// Boolean capability names.
///
/// The names are used as indices into a [`Desc`](../struct.Desc.html).
///
/// Name                      | Short name
/// --------------------------|-----------
/// `auto_left_margin`        | `bw`
/// `auto_right_margin`       | `am`
/// `no_esc_ctlc`             | `xsb`
/// `ceol_standout_glitch`    | `xhp`
/// `eat_newline_glitch`      | `xenl`
/// `erase_overstrike`        | `eo`
/// `generic_type`            | `gn`
/// `hard_copy`               | `hc`
/// `has_meta_key`            | `km`
/// `has_status_line`         | `hs`
/// `insert_null_glitch`      | `in_`
/// `memory_above`            | `db`
/// `memory_below`            | `da`
/// `move_insert_mode`        | `mir`
/// `move_standout_mode`      | `msgr`
/// `over_strike`             | `os`
/// `status_line_esc_ok`      | `eslok`
/// `dest_tabs_magic_smso`    | `xt`
/// `tilde_glitch`            | `hz`
/// `transparent_underline`   | `ul`
/// `xon_xoff`                | `xon`
/// `needs_xon_xoff`          | `nxon`
/// `prtr_silent`             | `mc5i`
/// `hard_cursor`             | `chts`
/// `non_rev_rmcup`           | `nrrmc`
/// `no_pad_char`             | `npc`
/// `non_dest_scroll_region`  | `ndscr`
/// `can_change`              | `ccc`
/// `back_color_erase`        | `bce`
/// `hue_lightness_saturation`| `hls`
/// `col_addr_glitch`         | `xhpa`
/// `cr_cancels_micro_mode`   | `crxm`
/// `has_print_wheel`         | `daisy`
/// `row_addr_glitch`         | `xvpa`
/// `semi_auto_right_margin`  | `sam`
/// `cpi_changes_res`         | `cpix`
/// `lpi_changes_res`         | `lpix`
/// `backspaces_with_bs`      | `OTbs_b`
/// `crt_no_scrolling`        | `OTns`
/// `no_correctly_working_cr` | `OTnc`
/// `gnu_has_meta_key`        | `OTMT`
/// `linefeed_is_newline`     | `OTNL`
/// `has_hardware_tabs`       | `OTpt`
/// `return_does_clr_eol`     | `OTxr`
#[derive(Clone, Copy, Debug)]
pub struct Boolean(pub(super) usize);

/// Numeric capability names.
///
/// The names are used as indices into a [`Desc`](../struct.Desc.html).
///
/// Name                     | Short name
/// -------------------------|-----------
/// `columns`                | `cols`
/// `init_tabs`              | `it`
/// `lines`                  | `lines`
/// `lines_of_memory`        | `lm`
/// `magic_cookie_glitch`    | `xmc`
/// `padding_baud_rate`      | `pb`
/// `virtual_terminal`       | `vt`
/// `width_status_line`      | `wsl`
/// `num_labels`             | `nlab`
/// `label_height`           | `lh`
/// `label_width`            | `lw`
/// `max_attributes`         | `ma`
/// `maximum_windows`        | `wnum`
/// `max_colors`             | `colors`
/// `max_pairs`              | `pairs`
/// `no_color_video`         | `ncv`
/// `buffer_capacity`        | `bufsz`
/// `dot_vert_spacing`       | `spinv`
/// `dot_horz_spacing`       | `spinh`
/// `max_micro_address`      | `maddr`
/// `max_micro_jump`         | `mjump`
/// `micro_col_size`         | `mcs`
/// `micro_line_size`        | `mls`
/// `number_of_pins`         | `npins`
/// `output_res_char`        | `orc`
/// `output_res_line`        | `orl`
/// `output_res_horz_inch`   | `orhi`
/// `output_res_vert_inch`   | `orvi`
/// `print_rate`             | `cps`
/// `wide_char_size`         | `widcs`
/// `buttons`                | `btns`
/// `bit_image_entwining`    | `bitwin`
/// `bit_image_type`         | `bitype`
/// `magic_cookie_glitch_ul` | `UTug`
/// `carriage_return_delay`  | `OTdC`
/// `new_line_delay`         | `OTdN`
/// `backspace_delay`        | `OTdB`
/// `horizontal_tab_delay`   | `OTdT`
/// `number_of_function_keys`| `OTkn`
#[derive(Clone, Copy, Debug)]
pub struct Number(pub(super) usize);

/// String capability names.
///
/// The names are used as indices into a [`Desc`](../struct.Desc.html).
///
/// Name                       | Short name
/// ---------------------------|-----------
/// `back_tab`                 | `cbt`
/// `bell`                     | `bel`
/// `carriage_return`          | `cr`
/// `change_scroll_region`     | `csr`
/// `clear_all_tabs`           | `tbc`
/// `clear_screen`             | `clear`
/// `clr_eol`                  | `el`
/// `clr_eos`                  | `ed`
/// `column_address`           | `hpa`
/// `command_character`        | `cmdch`
/// `cursor_address`           | `cup`
/// `cursor_down`              | `cud1`
/// `cursor_home`              | `home`
/// `cursor_invisible`         | `civis`
/// `cursor_left`              | `cub1`
/// `cursor_mem_address`       | `mrcup`
/// `cursor_normal`            | `cnorm`
/// `cursor_right`             | `cuf1`
/// `cursor_to_ll`             | `ll`
/// `cursor_up`                | `cuu1`
/// `cursor_visible`           | `cvvis`
/// `delete_character`         | `dch1`
/// `delete_line`              | `dl1`
/// `dis_status_line`          | `dsl`
/// `down_half_line`           | `hd`
/// `enter_alt_charset_mode`   | `smacs`
/// `enter_blink_mode`         | `blink`
/// `enter_bold_mode`          | `bold`
/// `enter_ca_mode`            | `smcup`
/// `enter_delete_mode`        | `smdc`
/// `enter_dim_mode`           | `dim`
/// `enter_insert_mode`        | `smir`
/// `enter_secure_mode`        | `invis`
/// `enter_protected_mode`     | `prot`
/// `enter_reverse_mode`       | `rev`
/// `enter_standout_mode`      | `smso`
/// `enter_underline_mode`     | `smul`
/// `erase_chars`              | `ech`
/// `exit_alt_charset_mode`    | `rmacs`
/// `exit_attribute_mode`      | `sgr0`
/// `exit_ca_mode`             | `rmcup`
/// `exit_delete_mode`         | `rmdc`
/// `exit_insert_mode`         | `rmir`
/// `exit_standout_mode`       | `rmso`
/// `exit_underline_mode`      | `rmul`
/// `flash_screen`             | `flash`
/// `form_feed`                | `ff`
/// `from_status_line`         | `fsl`
/// `init_1string`             | `is1`
/// `init_2string`             | `is2`
/// `init_3string`             | `is3`
/// `init_file`                | `if_`
/// `insert_character`         | `ich1`
/// `insert_line`              | `il1`
/// `insert_padding`           | `ip`
/// `key_backspace`            | `kbs`
/// `key_catab`                | `ktbc`
/// `key_clear`                | `kclr`
/// `key_ctab`                 | `kctab`
/// `key_dc`                   | `kdch1`
/// `key_dl`                   | `kdl1`
/// `key_down`                 | `kcud1`
/// `key_eic`                  | `krmir`
/// `key_eol`                  | `kel`
/// `key_eos`                  | `ked`
/// `key_f0`                   | `kf0`
/// `key_f1`                   | `kf1`
/// `key_f10`                  | `kf10`
/// `key_f2`                   | `kf2`
/// `key_f3`                   | `kf3`
/// `key_f4`                   | `kf4`
/// `key_f5`                   | `kf5`
/// `key_f6`                   | `kf6`
/// `key_f7`                   | `kf7`
/// `key_f8`                   | `kf8`
/// `key_f9`                   | `kf9`
/// `key_home`                 | `khome`
/// `key_ic`                   | `kich1`
/// `key_il`                   | `kil1`
/// `key_left`                 | `kcub1`
/// `key_ll`                   | `kll`
/// `key_npage`                | `knp`
/// `key_ppage`                | `kpp`
/// `key_right`                | `kcuf1`
/// `key_sf`                   | `kind`
/// `key_sr`                   | `kri`
/// `key_stab`                 | `khts`
/// `key_up`                   | `kcuu1`
/// `keypad_local`             | `rmkx`
/// `keypad_xmit`              | `smkx`
/// `lab_f0`                   | `lf0`
/// `lab_f1`                   | `lf1`
/// `lab_f10`                  | `lf10`
/// `lab_f2`                   | `lf2`
/// `lab_f3`                   | `lf3`
/// `lab_f4`                   | `lf4`
/// `lab_f5`                   | `lf5`
/// `lab_f6`                   | `lf6`
/// `lab_f7`                   | `lf7`
/// `lab_f8`                   | `lf8`
/// `lab_f9`                   | `lf9`
/// `meta_off`                 | `rmm`
/// `meta_on`                  | `smm`
/// `newline`                  | `nel`
/// `pad_char`                 | `pad`
/// `parm_dch`                 | `dch`
/// `parm_delete_line`         | `dl`
/// `parm_down_cursor`         | `cud`
/// `parm_ich`                 | `ich`
/// `parm_index`               | `indn`
/// `parm_insert_line`         | `il`
/// `parm_left_cursor`         | `cub`
/// `parm_right_cursor`        | `cuf`
/// `parm_rindex`              | `rin`
/// `parm_up_cursor`           | `cuu`
/// `pkey_key`                 | `pfkey`
/// `pkey_local`               | `pfloc`
/// `pkey_xmit`                | `pfx`
/// `print_screen`             | `mc0`
/// `prtr_off`                 | `mc4`
/// `prtr_on`                  | `mc5`
/// `repeat_char`              | `rep`
/// `reset_1string`            | `rs1`
/// `reset_2string`            | `rs2`
/// `reset_3string`            | `rs3`
/// `reset_file`               | `rf`
/// `restore_cursor`           | `rc`
/// `row_address`              | `vpa`
/// `save_cursor`              | `sc`
/// `scroll_forward`           | `ind`
/// `scroll_reverse`           | `ri`
/// `set_attributes`           | `sgr`
/// `set_tab`                  | `hts`
/// `set_window`               | `wind`
/// `tab`                      | `ht`
/// `to_status_line`           | `tsl`
/// `underline_char`           | `uc`
/// `up_half_line`             | `hu`
/// `init_prog`                | `iprog`
/// `key_a1`                   | `ka1`
/// `key_a3`                   | `ka3`
/// `key_b2`                   | `kb2`
/// `key_c1`                   | `kc1`
/// `key_c3`                   | `kc3`
/// `prtr_non`                 | `mc5p`
/// `char_padding`             | `rmp`
/// `acs_chars`                | `acsc`
/// `plab_norm`                | `pln`
/// `key_btab`                 | `kcbt`
/// `enter_xon_mode`           | `smxon`
/// `exit_xon_mode`            | `rmxon`
/// `enter_am_mode`            | `smam`
/// `exit_am_mode`             | `rmam`
/// `xon_character`            | `xonc`
/// `xoff_character`           | `xoffc`
/// `ena_acs`                  | `enacs`
/// `label_on`                 | `smln`
/// `label_off`                | `rmln`
/// `key_beg`                  | `kbeg`
/// `key_cancel`               | `kcan`
/// `key_close`                | `kclo`
/// `key_command`              | `kcmd`
/// `key_copy`                 | `kcpy`
/// `key_create`               | `kcrt`
/// `key_end`                  | `kend`
/// `key_enter`                | `kent`
/// `key_exit`                 | `kext`
/// `key_find`                 | `kfnd`
/// `key_help`                 | `khlp`
/// `key_mark`                 | `kmrk`
/// `key_message`              | `kmsg`
/// `key_move`                 | `kmov`
/// `key_next`                 | `knxt`
/// `key_open`                 | `kopn`
/// `key_options`              | `kopt`
/// `key_previous`             | `kprv`
/// `key_print`                | `kprt`
/// `key_redo`                 | `krdo`
/// `key_reference`            | `kref`
/// `key_refresh`              | `krfr`
/// `key_replace`              | `krpl`
/// `key_restart`              | `krst`
/// `key_resume`               | `kres`
/// `key_save`                 | `ksav`
/// `key_suspend`              | `kspd`
/// `key_undo`                 | `kund`
/// `key_sbeg`                 | `kBEG`
/// `key_scancel`              | `kCAN`
/// `key_scommand`             | `kCMD`
/// `key_scopy`                | `kCPY`
/// `key_screate`              | `kCRT`
/// `key_sdc`                  | `kDC`
/// `key_sdl`                  | `kDL`
/// `key_select`               | `kslt`
/// `key_send`                 | `kEND`
/// `key_seol`                 | `kEOL`
/// `key_sexit`                | `kEXT`
/// `key_sfind`                | `kFND`
/// `key_shelp`                | `kHLP`
/// `key_shome`                | `kHOM`
/// `key_sic`                  | `kIC`
/// `key_sleft`                | `kLFT`
/// `key_smessage`             | `kMSG`
/// `key_smove`                | `kMOV`
/// `key_snext`                | `kNXT`
/// `key_soptions`             | `kOPT`
/// `key_sprevious`            | `kPRV`
/// `key_sprint`               | `kPRT`
/// `key_sredo`                | `kRDO`
/// `key_sreplace`             | `kRPL`
/// `key_sright`               | `kRIT`
/// `key_srsume`               | `kRES`
/// `key_ssave`                | `kSAV`
/// `key_ssuspend`             | `kSPD`
/// `key_sundo`                | `kUND`
/// `req_for_input`            | `rfi`
/// `key_f11`                  | `kf11`
/// `key_f12`                  | `kf12`
/// `key_f13`                  | `kf13`
/// `key_f14`                  | `kf14`
/// `key_f15`                  | `kf15`
/// `key_f16`                  | `kf16`
/// `key_f17`                  | `kf17`
/// `key_f18`                  | `kf18`
/// `key_f19`                  | `kf19`
/// `key_f20`                  | `kf20`
/// `key_f21`                  | `kf21`
/// `key_f22`                  | `kf22`
/// `key_f23`                  | `kf23`
/// `key_f24`                  | `kf24`
/// `key_f25`                  | `kf25`
/// `key_f26`                  | `kf26`
/// `key_f27`                  | `kf27`
/// `key_f28`                  | `kf28`
/// `key_f29`                  | `kf29`
/// `key_f30`                  | `kf30`
/// `key_f31`                  | `kf31`
/// `key_f32`                  | `kf32`
/// `key_f33`                  | `kf33`
/// `key_f34`                  | `kf34`
/// `key_f35`                  | `kf35`
/// `key_f36`                  | `kf36`
/// `key_f37`                  | `kf37`
/// `key_f38`                  | `kf38`
/// `key_f39`                  | `kf39`
/// `key_f40`                  | `kf40`
/// `key_f41`                  | `kf41`
/// `key_f42`                  | `kf42`
/// `key_f43`                  | `kf43`
/// `key_f44`                  | `kf44`
/// `key_f45`                  | `kf45`
/// `key_f46`                  | `kf46`
/// `key_f47`                  | `kf47`
/// `key_f48`                  | `kf48`
/// `key_f49`                  | `kf49`
/// `key_f50`                  | `kf50`
/// `key_f51`                  | `kf51`
/// `key_f52`                  | `kf52`
/// `key_f53`                  | `kf53`
/// `key_f54`                  | `kf54`
/// `key_f55`                  | `kf55`
/// `key_f56`                  | `kf56`
/// `key_f57`                  | `kf57`
/// `key_f58`                  | `kf58`
/// `key_f59`                  | `kf59`
/// `key_f60`                  | `kf60`
/// `key_f61`                  | `kf61`
/// `key_f62`                  | `kf62`
/// `key_f63`                  | `kf63`
/// `clr_bol`                  | `el1`
/// `clear_margins`            | `mgc`
/// `set_left_margin`          | `smgl`
/// `set_right_margin`         | `smgr`
/// `label_format`             | `fln`
/// `set_clock`                | `sclk`
/// `display_clock`            | `dclk`
/// `remove_clock`             | `rmclk`
/// `create_window`            | `cwin`
/// `goto_window`              | `wingo`
/// `hangup`                   | `hup`
/// `dial_phone`               | `dial`
/// `quick_dial`               | `qdial`
/// `tone`                     | `tone`
/// `pulse`                    | `pulse`
/// `flash_hook`               | `hook`
/// `fixed_pause`              | `pause`
/// `wait_tone`                | `wait`
/// `user0`                    | `u0`
/// `user1`                    | `u1`
/// `user2`                    | `u2`
/// `user3`                    | `u3`
/// `user4`                    | `u4`
/// `user5`                    | `u5`
/// `user6`                    | `u6`
/// `user7`                    | `u7`
/// `user8`                    | `u8`
/// `user9`                    | `u9`
/// `orig_pair`                | `op`
/// `orig_colors`              | `oc`
/// `initialize_color`         | `initc`
/// `initialize_pair`          | `initp`
/// `set_color_pair`           | `scp`
/// `set_foreground`           | `setf`
/// `set_background`           | `setb`
/// `change_char_pitch`        | `cpi`
/// `change_line_pitch`        | `lpi`
/// `change_res_horz`          | `chr`
/// `change_res_vert`          | `cvr`
/// `define_char`              | `defc`
/// `enter_doublewide_mode`    | `swidm`
/// `enter_draft_quality`      | `sdrfq`
/// `enter_italics_mode`       | `sitm`
/// `enter_leftward_mode`      | `slm`
/// `enter_micro_mode`         | `smicm`
/// `enter_near_letter_quality`| `snlq`
/// `enter_normal_quality`     | `snrmq`
/// `enter_shadow_mode`        | `sshm`
/// `enter_subscript_mode`     | `ssubm`
/// `enter_superscript_mode`   | `ssupm`
/// `enter_upward_mode`        | `sum`
/// `exit_doublewide_mode`     | `rwidm`
/// `exit_italics_mode`        | `ritm`
/// `exit_leftward_mode`       | `rlm`
/// `exit_micro_mode`          | `rmicm`
/// `exit_shadow_mode`         | `rshm`
/// `exit_subscript_mode`      | `rsubm`
/// `exit_superscript_mode`    | `rsupm`
/// `exit_upward_mode`         | `rum`
/// `micro_column_address`     | `mhpa`
/// `micro_down`               | `mcud1`
/// `micro_left`               | `mcub1`
/// `micro_right`              | `mcuf1`
/// `micro_row_address`        | `mvpa`
/// `micro_up`                 | `mcuu1`
/// `order_of_pins`            | `porder`
/// `parm_down_micro`          | `mcud`
/// `parm_left_micro`          | `mcub`
/// `parm_right_micro`         | `mcuf`
/// `parm_up_micro`            | `mcuu`
/// `select_char_set`          | `scs`
/// `set_bottom_margin`        | `smgb`
/// `set_bottom_margin_parm`   | `smgbp`
/// `set_left_margin_parm`     | `smglp`
/// `set_right_margin_parm`    | `smgrp`
/// `set_top_margin`           | `smgt`
/// `set_top_margin_parm`      | `smgtp`
/// `start_bit_image`          | `sbim`
/// `start_char_set_def`       | `scsd`
/// `stop_bit_image`           | `rbim`
/// `stop_char_set_def`        | `rcsd`
/// `subscript_characters`     | `subcs`
/// `superscript_characters`   | `supcs`
/// `these_cause_cr`           | `docr`
/// `zero_motion`              | `zerom`
/// `char_set_names`           | `csnm`
/// `key_mouse`                | `kmous`
/// `mouse_info`               | `minfo`
/// `req_mouse_pos`            | `reqmp`
/// `get_mouse`                | `getm`
/// `set_a_foreground`         | `setaf`
/// `set_a_background`         | `setab`
/// `pkey_plab`                | `pfxl`
/// `device_type`              | `devt`
/// `code_set_init`            | `csin`
/// `set0_des_seq`             | `s0ds`
/// `set1_des_seq`             | `s1ds`
/// `set2_des_seq`             | `s2ds`
/// `set3_des_seq`             | `s3ds`
/// `set_lr_margin`            | `smglr`
/// `set_tb_margin`            | `smgtb`
/// `bit_image_repeat`         | `birep`
/// `bit_image_newline`        | `binel`
/// `bit_image_carriage_return`| `bicr`
/// `color_names`              | `colornm`
/// `define_bit_image_region`  | `defbi`
/// `end_bit_image_region`     | `endbi`
/// `set_color_band`           | `setcolor`
/// `set_page_length`          | `slines`
/// `display_pc_char`          | `dispc`
/// `enter_pc_charset_mode`    | `smpch`
/// `exit_pc_charset_mode`     | `rmpch`
/// `enter_scancode_mode`      | `smsc`
/// `exit_scancode_mode`       | `rmsc`
/// `pc_term_options`          | `pctrm`
/// `scancode_escape`          | `scesc`
/// `alt_scancode_esc`         | `scesa`
/// `enter_horizontal_hl_mode` | `ehhlm`
/// `enter_left_hl_mode`       | `elhlm`
/// `enter_low_hl_mode`        | `elohlm`
/// `enter_right_hl_mode`      | `erhlm`
/// `enter_top_hl_mode`        | `ethlm`
/// `enter_vertical_hl_mode`   | `evhlm`
/// `set_a_attributes`         | `sgr1`
/// `set_pglen_inch`           | `slength`
/// `termcap_init2`            | `OTi2`
/// `termcap_reset`            | `OTrs`
/// `linefeed_if_not_lf`       | `OTnl`
/// `backspace_if_not_bs`      | `OTbs_s`
/// `other_non_function_keys`  | `OTko`
/// `arrow_key_map`            | `OTma`
/// `acs_ulcorner`             | `OTG2`
/// `acs_llcorner`             | `OTG3`
/// `acs_urcorner`             | `OTG1`
/// `acs_lrcorner`             | `OTG4`
/// `acs_ltee`                 | `OTGR`
/// `acs_rtee`                 | `OTGL`
/// `acs_btee`                 | `OTGU`
/// `acs_ttee`                 | `OTGD`
/// `acs_hline`                | `OTGH`
/// `acs_vline`                | `OTGV`
/// `acs_plus`                 | `OTGC`
/// `memory_lock`              | `meml`
/// `memory_unlock`            | `memu`
/// `box_chars_1`              | `box1`
#[derive(Clone, Copy, Debug)]
pub struct String(pub(super) usize);

/// A name of a user-defined capability.
#[derive(PartialEq, Clone, Debug)]
pub struct UserDef(pub(super) StdString);

impl UserDef {
    /// Creates a `UserDef` from a string.
    pub fn named<T: AsRef<str>>(name: T) -> UserDef {
        UserDef(name.as_ref().into())
    }

    /// The string form of this name.
    pub fn name(&self) -> &str {
        &self.0
    }
}

impl Boolean {
    /// The `Boolean` capabilitiy name corresponding to the string
    /// `name`.
    pub fn named<T: Borrow<str>>(name: T) -> Option<Boolean> {
        let name = name.borrow();
        let len = name.len();
        let pos = if len >= 2 && len <= 6 {
            BOOLS.iter().position(|&n| n == name)
        } else if len >= 8 && len <= 24 {
            BOOLEANS.iter().position(|&n| n == name)
        } else {
            None
        };
        pos.map(Boolean)
    }

    /// An iterator over the predefined boolean capabilities.
    pub fn iter() -> BoolIter {
        BoolIter { current: 0 }
    }

    /// The short name of the capability.
    pub fn short_name(&self) -> &'static str {
        BOOLS[self.0]
    }

    /// The long name of the capability.
    pub fn long_name(&self) -> &'static str {
        BOOLEANS[self.0]
    }
}

pub struct BoolIter {
    current: usize,
}

impl Iterator for BoolIter {
    type Item = Boolean;
    fn next(&mut self) -> Option<Boolean> {
        if self.current < NUM_BOOLS {
            self.current += 1;
            Some(Boolean(self.current - 1))
        } else {
            None
        }
    }
}

impl Number {
    /// The `Number` capabilitiy name corresponding to the string
    /// `name`.
    pub fn named<T: Borrow<str>>(name: T) -> Option<Number> {
        let name = name.borrow();
        let len = name.len();
        let pos = if len >= 2 && len < 5 {
            NUMS.iter().position(|&n| n == name)
        } else if len >= 5 && len <= 23 {
            NUMBERS
                .iter()
                .position(|&n| n == name)
                .or_else(|| NUMS.iter().position(|&n| n == name))
        } else {
            None
        };
        pos.map(Number)
    }

    /// An iterator over the predefined numeric capabilities.
    pub fn iter() -> NumIter {
        NumIter { current: 0 }
    }

    /// The short name of the capability.
    pub fn short_name(&self) -> &'static str {
        NUMS[self.0]
    }

    /// The long name of the capability.
    pub fn long_name(&self) -> &'static str {
        NUMBERS[self.0]
    }
}

pub struct NumIter {
    current: usize,
}

impl Iterator for NumIter {
    type Item = Number;
    fn next(&mut self) -> Option<Number> {
        if self.current < NUM_INTS {
            self.current += 1;
            Some(Number(self.current - 1))
        } else {
            None
        }
    }
}

impl String {
    /// The `String` capabilitiy name corresponding to the string
    /// `name`.
    pub fn named<T: Borrow<str>>(name: T) -> Option<String> {
        let name = name.borrow();
        let len = name.len();
        let pos = if len < 6 {
            STRS.iter().position(|&n| n == name)
        } else if len <= 25 {
            STRINGS
                .iter()
                .position(|&n| n == name)
                .or_else(|| STRS.iter().position(|&n| n == name))
        } else {
            None
        };
        pos.map(String)
    }

    /// An iterator over the predefined string capabilities.
    pub fn iter() -> StrIter {
        StrIter { current: 0 }
    }

    /// The short name of the capability.
    pub fn short_name(&self) -> &'static str {
        STRS[self.0]
    }

    /// The long name of the capability.
    pub fn long_name(&self) -> &'static str {
        STRINGS[self.0]
    }
}

pub struct StrIter {
    current: usize,
}

impl Iterator for StrIter {
    type Item = String;
    fn next(&mut self) -> Option<String> {
        if self.current < NUM_STRS {
            self.current += 1;
            Some(String(self.current - 1))
        } else {
            None
        }
    }
}

#[derive(Clone, Debug)]
pub(super) enum CapName {
    P(usize),
    U(UserDef),
}

#[derive(Clone, Debug)]
pub(super) enum ICap {
    Bool(CapName, bool),
    Num(CapName, u16),
    Str(CapName, Vec<u8>),
}

/// A generic capability name / value pair.
pub struct Cap(pub(super) ICap);

impl From<(Boolean, bool)> for Cap {
    fn from(val: (Boolean, bool)) -> Cap {
        Cap(ICap::Bool(CapName::P((val.0).0), val.1))
    }
}

impl From<(Number, u16)> for Cap {
    fn from(val: (Number, u16)) -> Cap {
        Cap(ICap::Num(CapName::P((val.0).0), val.1))
    }
}

impl<V> From<(String, V)> for Cap
where
    V: AsRef<[u8]>,
{
    fn from(val: (String, V)) -> Cap {
        Cap(ICap::Str(CapName::P((val.0).0), (val.1).as_ref().into()))
    }
}

impl<K> From<(K, bool)> for Cap
where
    K: Borrow<UserDef>,
{
    fn from(val: (K, bool)) -> Cap {
        Cap(ICap::Bool(CapName::U((val.0).borrow().clone()), val.1))
    }
}

impl<K> From<(K, u16)> for Cap
where
    K: Borrow<UserDef>,
{
    fn from(val: (K, u16)) -> Cap {
        Cap(ICap::Num(CapName::U((val.0).borrow().clone()), val.1))
    }
}

impl<K> From<(K, &'static str)> for Cap
where
    K: Borrow<UserDef>,
{
    fn from(val: (K, &'static str)) -> Cap {
        Cap(ICap::Str(
            CapName::U((val.0).borrow().clone()),
            (val.1).into(),
        ))
    }
}

static DEF_BOOL: bool = false;

impl Index<Boolean> for super::Desc {
    type Output = bool;

    /// The value of the boolean capability named by `index`.
    fn index(&self, idx: Boolean) -> &bool {
        if self.bools.len() > idx.0 {
            &self.bools[idx.0]
        } else {
            &DEF_BOOL
        }
    }
}

static DEF_NUM: u16 = 0xffff;

impl Index<Number> for super::Desc {
    type Output = u16;

    /// The value of the numeric capability named by `index`.
    fn index(&self, idx: Number) -> &u16 {
        if self.nums.len() > idx.0 {
            &self.nums[idx.0]
        } else {
            &DEF_NUM
        }
    }
}

static DEF_STR: &[u8] = &[];

impl Index<String> for super::Desc {
    type Output = [u8];

    /// The value of the string capability named by `index`.
    fn index(&self, idx: String) -> &[u8] {
        if self.strings.len() > idx.0 {
            &self.strings[idx.0]
        } else {
            DEF_STR
        }
    }
}
