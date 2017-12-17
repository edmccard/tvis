use super::{Boolean, Number, String};

pub const bw: Boolean = Boolean(0);
pub const am: Boolean = Boolean(1);
pub const xsb: Boolean = Boolean(2);
pub const xhp: Boolean = Boolean(3);
pub const xenl: Boolean = Boolean(4);
pub const eo: Boolean = Boolean(5);
pub const gn: Boolean = Boolean(6);
pub const hc: Boolean = Boolean(7);
pub const km: Boolean = Boolean(8);
pub const hs: Boolean = Boolean(9);
pub const in_: Boolean = Boolean(10);
pub const db: Boolean = Boolean(11);
pub const da: Boolean = Boolean(12);
pub const mir: Boolean = Boolean(13);
pub const msgr: Boolean = Boolean(14);
pub const os: Boolean = Boolean(15);
pub const eslok: Boolean = Boolean(16);
pub const xt: Boolean = Boolean(17);
pub const hz: Boolean = Boolean(18);
pub const ul: Boolean = Boolean(19);
pub const xon: Boolean = Boolean(20);
pub const nxon: Boolean = Boolean(21);
pub const mc5i: Boolean = Boolean(22);
pub const chts: Boolean = Boolean(23);
pub const nrrmc: Boolean = Boolean(24);
pub const npc: Boolean = Boolean(25);
pub const ndscr: Boolean = Boolean(26);
pub const ccc: Boolean = Boolean(27);
pub const bce: Boolean = Boolean(28);
pub const hls: Boolean = Boolean(29);
pub const xhpa: Boolean = Boolean(30);
pub const crxm: Boolean = Boolean(31);
pub const daisy: Boolean = Boolean(32);
pub const xvpa: Boolean = Boolean(33);
pub const sam: Boolean = Boolean(34);
pub const cpix: Boolean = Boolean(35);
pub const lpix: Boolean = Boolean(36);
pub const OTbs_b: Boolean = Boolean(37);
pub const OTns: Boolean = Boolean(38);
pub const OTnc: Boolean = Boolean(39);
pub const OTMT: Boolean = Boolean(40);
pub const OTNL: Boolean = Boolean(41);
pub const OTpt: Boolean = Boolean(42);
pub const OTxr: Boolean = Boolean(43);

pub const auto_left_margin: Boolean = Boolean(0);
pub const auto_right_margin: Boolean = Boolean(1);
pub const no_esc_ctlc: Boolean = Boolean(2);
pub const ceol_standout_glitch: Boolean = Boolean(3);
pub const eat_newline_glitch: Boolean = Boolean(4);
pub const erase_overstrike: Boolean = Boolean(5);
pub const generic_type: Boolean = Boolean(6);
pub const hard_copy: Boolean = Boolean(7);
pub const has_meta_key: Boolean = Boolean(8);
pub const has_status_line: Boolean = Boolean(9);
pub const insert_null_glitch: Boolean = Boolean(10);
pub const memory_above: Boolean = Boolean(11);
pub const memory_below: Boolean = Boolean(12);
pub const move_insert_mode: Boolean = Boolean(13);
pub const move_standout_mode: Boolean = Boolean(14);
pub const over_strike: Boolean = Boolean(15);
pub const status_line_esc_ok: Boolean = Boolean(16);
pub const dest_tabs_magic_smso: Boolean = Boolean(17);
pub const tilde_glitch: Boolean = Boolean(18);
pub const transparent_underline: Boolean = Boolean(19);
pub const xon_xoff: Boolean = Boolean(20);
pub const needs_xon_xoff: Boolean = Boolean(21);
pub const prtr_silent: Boolean = Boolean(22);
pub const hard_cursor: Boolean = Boolean(23);
pub const non_rev_rmcup: Boolean = Boolean(24);
pub const no_pad_char: Boolean = Boolean(25);
pub const non_dest_scroll_region: Boolean = Boolean(26);
pub const can_change: Boolean = Boolean(27);
pub const back_color_erase: Boolean = Boolean(28);
pub const hue_lightness_saturation: Boolean = Boolean(29);
pub const col_addr_glitch: Boolean = Boolean(30);
pub const cr_cancels_micro_mode: Boolean = Boolean(31);
pub const has_print_wheel: Boolean = Boolean(32);
pub const row_addr_glitch: Boolean = Boolean(33);
pub const semi_auto_right_margin: Boolean = Boolean(34);
pub const cpi_changes_res: Boolean = Boolean(35);
pub const lpi_changes_res: Boolean = Boolean(36);
pub const backspaces_with_bs: Boolean = Boolean(37);
pub const crt_no_scrolling: Boolean = Boolean(38);
pub const no_correctly_working_cr: Boolean = Boolean(39);
pub const gnu_has_meta_key: Boolean = Boolean(40);
pub const linefeed_is_newline: Boolean = Boolean(41);
pub const has_hardware_tabs: Boolean = Boolean(42);
pub const return_does_clr_eol: Boolean = Boolean(43);

pub const NUM_BOOLS: usize = 44; // restricted?

pub const cols: Number = Number(0);
pub const it: Number = Number(1);
pub const lines: Number = Number(2);
pub const lm: Number = Number(3);
pub const xmc: Number = Number(4);
pub const pb: Number = Number(5);
pub const vt: Number = Number(6);
pub const wsl: Number = Number(7);
pub const nlab: Number = Number(8);
pub const lh: Number = Number(9);
pub const lw: Number = Number(10);
pub const ma: Number = Number(11);
pub const wnum: Number = Number(12);
pub const colors: Number = Number(13);
pub const pairs: Number = Number(14);
pub const ncv: Number = Number(15);
pub const bufsz: Number = Number(16);
pub const spinv: Number = Number(17);
pub const spinh: Number = Number(18);
pub const maddr: Number = Number(19);
pub const mjump: Number = Number(20);
pub const mcs: Number = Number(21);
pub const mls: Number = Number(22);
pub const npins: Number = Number(23);
pub const orc: Number = Number(24);
pub const orl: Number = Number(25);
pub const orhi: Number = Number(26);
pub const orvi: Number = Number(27);
pub const cps: Number = Number(28);
pub const widcs: Number = Number(29);
pub const btns: Number = Number(30);
pub const bitwin: Number = Number(31);
pub const bitype: Number = Number(32);
pub const UTug: Number = Number(33);
pub const OTdC: Number = Number(34);
pub const OTdN: Number = Number(35);
pub const OTdB: Number = Number(36);
pub const OTdT: Number = Number(37);
pub const OTkn: Number = Number(38);

pub const columns: Number = Number(0);
pub const init_tabs: Number = Number(1);
//pub const lines: Number = Number(2); // same as short name
pub const lines_of_memory: Number = Number(3);
pub const magic_cookie_glitch: Number = Number(4);
pub const padding_baud_rate: Number = Number(5);
pub const virtual_terminal: Number = Number(6);
pub const width_status_line: Number = Number(7);
pub const num_labels: Number = Number(8);
pub const label_height: Number = Number(9);
pub const label_width: Number = Number(10);
pub const max_attributes: Number = Number(11);
pub const maximum_windows: Number = Number(12);
pub const max_colors: Number = Number(13);
pub const max_pairs: Number = Number(14);
pub const no_color_video: Number = Number(15);
pub const buffer_capacity: Number = Number(16);
pub const dot_vert_spacing: Number = Number(17);
pub const dot_horz_spacing: Number = Number(18);
pub const max_micro_address: Number = Number(19);
pub const max_micro_jump: Number = Number(20);
pub const micro_col_size: Number = Number(21);
pub const micro_line_size: Number = Number(22);
pub const number_of_pins: Number = Number(23);
pub const output_res_char: Number = Number(24);
pub const output_res_line: Number = Number(25);
pub const output_res_horz_inch: Number = Number(26);
pub const output_res_vert_inch: Number = Number(27);
pub const print_rate: Number = Number(28);
pub const wide_char_size: Number = Number(29);
pub const buttons: Number = Number(30);
pub const bit_image_entwining: Number = Number(31);
pub const bit_image_type: Number = Number(32);
pub const magic_cookie_glitch_ul: Number = Number(33);
pub const carriage_return_delay: Number = Number(34);
pub const new_line_delay: Number = Number(35);
pub const backspace_delay: Number = Number(36);
pub const horizontal_tab_delay: Number = Number(37);
pub const number_of_function_keys: Number = Number(38);

pub const NUM_INTS: usize = 39; // restricted?

pub const cbt: String = String(0); // out P
pub const bel: String = String(1); // out P
pub const cr: String = String(2); // out P*
pub const csr: String = String(3); // out P* 1 2
pub const tbc: String = String(4); // out P
pub const clear: String = String(5); // out P*
pub const el: String = String(6); // out P
pub const ed: String = String(7); // out P*
pub const hpa: String = String(8); // out P 1
pub const cmdch: String = String(9); // in
pub const cup: String = String(10); // out 1 2
pub const cud1: String = String(11); // out
pub const home: String = String(12); // out
pub const civis: String = String(13); // out
pub const cub1: String = String(14); // out
pub const mrcup: String = String(15); // out 1 2
pub const cnorm: String = String(16); // out
pub const cuf1: String = String(17); // out
pub const ll: String = String(18); // out
pub const cuu1: String = String(19); // out
pub const cvvis: String = String(20); // out
pub const dch1: String = String(21); // out P*
pub const dl1: String = String(22); // out P*
pub const dsl: String = String(23); // out
pub const hd: String = String(24); // out
pub const smacs: String = String(25); // out P
pub const blink: String = String(26); // out
pub const bold: String = String(27); // out
pub const smcup: String = String(28); // out
pub const smdc: String = String(29); // out
pub const dim: String = String(30); // out
pub const smir: String = String(31); // out
pub const invis: String = String(32); // out
pub const prot: String = String(33); // out
pub const rev: String = String(34); // out
pub const smso: String = String(35); // out
pub const smul: String = String(36); // out
pub const ech: String = String(37); // out P 1
pub const rmacs: String = String(38); // out P
pub const sgr0: String = String(39); // out
pub const rmcup: String = String(40); // out
pub const rmdc: String = String(41); // out
pub const rmir: String = String(42); // out
pub const rmso: String = String(43); // out
pub const rmul: String = String(44); // out
pub const flash: String = String(45); // out
pub const ff: String = String(46); // out P*
pub const fsl: String = String(47); // out
pub const is1: String = String(48); // out
pub const is2: String = String(49); // out
pub const is3: String = String(50); // out
pub const if_: String = String(51); // in
pub const ich1: String = String(52); // out P
pub const il1: String = String(53); // out P*
pub const ip: String = String(54); // out? P
pub const kbs: String = String(55); // in
pub const ktbc: String = String(56); // in
pub const kclr: String = String(57); // in
pub const kctab: String = String(58); // in
pub const kdch1: String = String(59); // in
pub const kdl1: String = String(60); // in
pub const kcud1: String = String(61); // in
pub const krmir: String = String(62); // in
pub const kel: String = String(63); // in
pub const ked: String = String(64); // in
pub const kf0: String = String(65); // in
pub const kf1: String = String(66); // in
pub const kf10: String = String(67); // in
pub const kf2: String = String(68); // in
pub const kf3: String = String(69); // in
pub const kf4: String = String(70); // in
pub const kf5: String = String(71); // in
pub const kf6: String = String(72); // in
pub const kf7: String = String(73); // in
pub const kf8: String = String(74); // in
pub const kf9: String = String(75); // in
pub const khome: String = String(76); // in
pub const kich1: String = String(77); // in
pub const kil1: String = String(78); // in
pub const kcub1: String = String(79); // in
pub const kll: String = String(80); // in
pub const knp: String = String(81); // in
pub const kpp: String = String(82); // in
pub const kcuf1: String = String(83); // in
pub const kind: String = String(84); // in
pub const kri: String = String(85); // in
pub const khts: String = String(86); // in
pub const kcuu1: String = String(87); // in
pub const rmkx: String = String(88); // out
pub const smkx: String = String(89); // out
pub const lf0: String = String(90); // in
pub const lf1: String = String(91); // in
pub const lf10: String = String(92); // in
pub const lf2: String = String(93); // in
pub const lf3: String = String(94); // in
pub const lf4: String = String(95); // in
pub const lf5: String = String(96); // in
pub const lf6: String = String(97); // in
pub const lf7: String = String(98); // in
pub const lf8: String = String(99); // in
pub const lf9: String = String(100); // in
pub const rmm: String = String(101); // out
pub const smm: String = String(102); // out
pub const nel: String = String(103); // out P
pub const pad: String = String(104); // in?
pub const dch: String = String(105); // out P* 1
pub const dl: String = String(106); // out P* 1
pub const cud: String = String(107); // out P* 1
pub const ich: String = String(108); // out P* 1
pub const indn: String = String(109); // out P 1
pub const il: String = String(110); // out P* 1
pub const cub: String = String(111); // out P 1
pub const cuf: String = String(112); // out P* 1
pub const rin: String = String(113); // out P 1
pub const cuu: String = String(114); // out P* 1
pub const pfkey: String = String(115); // out 1 2
pub const pfloc: String = String(116); // out 1 2
pub const pfx: String = String(117); // out 1 2
pub const mc0: String = String(118); // out
pub const mc4: String = String(119); // out
pub const mc5: String = String(120); // out
pub const rep: String = String(121); // out P* 1 2
pub const rs1: String = String(122); // out
pub const rs2: String = String(123); // out
pub const rs3: String = String(124); // out
pub const rf: String = String(125); // in
pub const rc: String = String(126); // out
pub const vpa: String = String(127); // out P 1
pub const sc: String = String(128); // out P
pub const ind: String = String(129); // out P
pub const ri: String = String(130); // out P
pub const sgr: String = String(131); // out P 1 2 3 4 5 6 7 8 9
pub const hts: String = String(132); // out
pub const wind: String = String(133); // out 1 2 3 4
pub const ht: String = String(134); // out
pub const tsl: String = String(135); // out 1
pub const uc: String = String(136); // out
pub const hu: String = String(137); // out
pub const iprog: String = String(138); // in
pub const ka1: String = String(139); // in
pub const ka3: String = String(140); // in
pub const kb2: String = String(141); // in
pub const kc1: String = String(142); // in
pub const kc3: String = String(143); // in
pub const mc5p: String = String(144); // out 1
pub const rmp: String = String(145); // out?
pub const acsc: String = String(146); // in
pub const pln: String = String(147); // 1 2
pub const kcbt: String = String(148); // in
pub const smxon: String = String(149); // out
pub const rmxon: String = String(150); // out
pub const smam: String = String(151); // out
pub const rmam: String = String(152); // out
pub const xonc: String = String(153); // in
pub const xoffc: String = String(154); // in
pub const enacs: String = String(155); // out
pub const smln: String = String(156); // out
pub const rmln: String = String(157); // out
pub const kbeg: String = String(158); // in
pub const kcan: String = String(159); // in
pub const kclo: String = String(160); // in
pub const kcmd: String = String(161); // in
pub const kcpy: String = String(162); // in
pub const kcrt: String = String(163); // in
pub const kend: String = String(164); // in
pub const kent: String = String(165); // in
pub const kext: String = String(166); // in
pub const kfnd: String = String(167); // in
pub const khlp: String = String(168); // in
pub const kmrk: String = String(169); // in
pub const kmsg: String = String(170); // in
pub const kmov: String = String(171); // in
pub const knxt: String = String(172); // in
pub const kopn: String = String(173); // in
pub const kopt: String = String(174); // in
pub const kprv: String = String(175); // in
pub const kprt: String = String(176); // in
pub const krdo: String = String(177); // in
pub const kref: String = String(178); // in
pub const krfr: String = String(179); // in
pub const krpl: String = String(180); // in
pub const krst: String = String(181); // in
pub const kres: String = String(182); // in
pub const ksav: String = String(183); // in
pub const kspd: String = String(184); // in
pub const kund: String = String(185); // in
pub const kBEG: String = String(186); // in
pub const kCAN: String = String(187); // in
pub const kCMD: String = String(188); // in
pub const kCPY: String = String(189); // in
pub const kCRT: String = String(190); // in
pub const kDC: String = String(191); // in
pub const kDL: String = String(192); // in
pub const kslt: String = String(193); // in
pub const kEND: String = String(194); // in
pub const kEOL: String = String(195); // in
pub const kEXT: String = String(196); // in
pub const kFND: String = String(197); // in
pub const kHLP: String = String(198); // in
pub const kHOM: String = String(199); // in
pub const kIC: String = String(200); // in
pub const kLFT: String = String(201); // in
pub const kMSG: String = String(202); // in
pub const kMOV: String = String(203); // in
pub const kNXT: String = String(204); // in
pub const kOPT: String = String(205); // in
pub const kPRV: String = String(206); // in
pub const kPRT: String = String(207); // in
pub const kRDO: String = String(208); // in
pub const kRPL: String = String(209); // in
pub const kRIT: String = String(210); // in
pub const kRES: String = String(211); // in
pub const kSAV: String = String(212); // in
pub const kSPD: String = String(213); // in
pub const kUND: String = String(214); // in
pub const rfi: String = String(215); // out?
pub const kf11: String = String(216); // in
pub const kf12: String = String(217); // in
pub const kf13: String = String(218); // in
pub const kf14: String = String(219); // in
pub const kf15: String = String(220); // in
pub const kf16: String = String(221); // in
pub const kf17: String = String(222); // in
pub const kf18: String = String(223); // in
pub const kf19: String = String(224); // in
pub const kf20: String = String(225); // in
pub const kf21: String = String(226); // in
pub const kf22: String = String(227); // in
pub const kf23: String = String(228); // in
pub const kf24: String = String(229); // in
pub const kf25: String = String(230); // in
pub const kf26: String = String(231); // in
pub const kf27: String = String(232); // in
pub const kf28: String = String(233); // in
pub const kf29: String = String(234); // in
pub const kf30: String = String(235); // in
pub const kf31: String = String(236); // in
pub const kf32: String = String(237); // in
pub const kf33: String = String(238); // in
pub const kf34: String = String(239); // in
pub const kf35: String = String(240); // in
pub const kf36: String = String(241); // in
pub const kf37: String = String(242); // in
pub const kf38: String = String(243); // in
pub const kf39: String = String(244); // in
pub const kf40: String = String(245); // in
pub const kf41: String = String(246); // in
pub const kf42: String = String(247); // in
pub const kf43: String = String(248); // in
pub const kf44: String = String(249); // in
pub const kf45: String = String(250); // in
pub const kf46: String = String(251); // in
pub const kf47: String = String(252); // in
pub const kf48: String = String(253); // in
pub const kf49: String = String(254); // in
pub const kf50: String = String(255); // in
pub const kf51: String = String(256); // in
pub const kf52: String = String(257); // in
pub const kf53: String = String(258); // in
pub const kf54: String = String(259); // in
pub const kf55: String = String(260); // in
pub const kf56: String = String(261); // in
pub const kf57: String = String(262); // in
pub const kf58: String = String(263); // in
pub const kf59: String = String(264); // in
pub const kf60: String = String(265); // in
pub const kf61: String = String(266); // in
pub const kf62: String = String(267); // in
pub const kf63: String = String(268); // in
pub const el1: String = String(269); // in
pub const mgc: String = String(270); // in
pub const smgl: String = String(271); // in
pub const smgr: String = String(272); // in
pub const fln: String = String(273); // in
pub const sclk: String = String(274); // out 1 2 3
pub const dclk: String = String(275); // out
pub const rmclk: String = String(276); // out
pub const cwin: String = String(277); // out 1 2 3 4 5
pub const wingo: String = String(278); // out 1
pub const hup: String = String(279); // out
pub const dial: String = String(280); // out 1
pub const qdial: String = String(281); // out 1
pub const tone: String = String(282); // out
pub const pulse: String = String(283); // out
pub const hook: String = String(284); // out
pub const pause: String = String(285); // out
pub const wait: String = String(286); // out
pub const u0: String = String(287); // out
pub const u1: String = String(288); // out
pub const u2: String = String(289); // out
pub const u3: String = String(290); // out
pub const u4: String = String(291); // out
pub const u5: String = String(292); // out
pub const u6: String = String(293); // in?
pub const u7: String = String(294); // out
pub const u8: String = String(295); // in?
pub const u9: String = String(296); // out
pub const op: String = String(297); // out
pub const oc: String = String(298); // out
pub const initc: String = String(299); // out 1 2 3 4
pub const initp: String = String(300); // out 1 2 3 4 5 6 7
pub const scp: String = String(301); // out 1
pub const setf: String = String(302); // out 1
pub const setb: String = String(303); // out 1
pub const cpi: String = String(304); // out 1
pub const lpi: String = String(305); // out 1
pub const chr: String = String(306); // out 1
pub const cvr: String = String(307); // out 1
pub const defc: String = String(308); // out 1 2 3
pub const swidm: String = String(309); // out
pub const sdrfq: String = String(310); // out
pub const sitm: String = String(311); // out
pub const slm: String = String(312); // out
pub const smicm: String = String(313); // out
pub const snlq: String = String(314); // out
pub const snrmq: String = String(315); // out
pub const sshm: String = String(316); // out
pub const ssubm: String = String(317); // out
pub const ssupm: String = String(318); // out
pub const sum: String = String(319); // out
pub const rwidm: String = String(320); // out
pub const ritm: String = String(321); // out
pub const rlm: String = String(322); // out
pub const rmicm: String = String(323); // out
pub const rshm: String = String(324); // out
pub const rsubm: String = String(325); // out
pub const rsupm: String = String(326); // out
pub const rum: String = String(327); // out
pub const mhpa: String = String(328); // out P 1
pub const mcud1: String = String(329); // out
pub const mcub1: String = String(330); // out
pub const mcuf1: String = String(331); // out
pub const mvpa: String = String(332); // out P 1
pub const mcuu1: String = String(333); // out
pub const porder: String = String(334); // ?
pub const mcud: String = String(335); // out P* 1
pub const mcub: String = String(336); // out P 1
pub const mcuf: String = String(337); // out P* 1
pub const mcuu: String = String(338); // out P* 1
pub const scs: String = String(339); // out 1
pub const smgb: String = String(340); // out
pub const smgbp: String = String(341); // out 1 2
pub const smglp: String = String(342); // out 1
pub const smgrp: String = String(343); // out 1
pub const smgt: String = String(344); // out
pub const smgtp: String = String(345); // out 1
pub const sbim: String = String(346); // out
pub const scsd: String = String(347); // out 1 2
pub const rbim: String = String(348); // out
pub const rcsd: String = String(349); // out 1
pub const subcs: String = String(350); // in
pub const supcs: String = String(351); // in
pub const docr: String = String(352); // in
pub const zerom: String = String(353); // out?
pub const csnm: String = String(354); // out 1
pub const kmous: String = String(355); // in
pub const minfo: String = String(356); // in?
pub const reqmp: String = String(357); // out
pub const getm: String = String(358); // out 1
pub const setaf: String = String(359); // out 1
pub const setab: String = String(360); // out 1
pub const pfxl: String = String(361); // out 1 2 3
pub const devt: String = String(362); // in?
pub const csin: String = String(363); // out?
pub const s0ds: String = String(364); // out
pub const s1ds: String = String(365); // out
pub const s2ds: String = String(366); // out
pub const s3ds: String = String(367); // out
pub const smglr: String = String(368); // out 1 2
pub const smgtb: String = String(369); // out 1 2
pub const birep: String = String(370); // out 1 2
pub const binel: String = String(371); // out
pub const bicr: String = String(372); // out
pub const colornm: String = String(373); // out 1
pub const defbi: String = String(374); // out
pub const endbi: String = String(375); // out
pub const setcolor: String = String(376); // out 1
pub const slines: String = String(377); // out 1
pub const dispc: String = String(378); // out 1
pub const smpch: String = String(379); // out
pub const rmpch: String = String(380); // out
pub const smsc: String = String(381); // out
pub const rmsc: String = String(382); // out
pub const pctrm: String = String(383); // in
pub const scesc: String = String(384); // in?
pub const scesa: String = String(385); // in?
pub const ehhlm: String = String(386); // out
pub const elhlm: String = String(387); // out
pub const elohlm: String = String(388); // out
pub const erhlm: String = String(389); // out
pub const ethlm: String = String(390); // out
pub const evhlm: String = String(391); // out
pub const sgr1: String = String(392); // out 1 2 3 4 5 6
pub const slength: String = String(393); // out 1
pub const OTi2: String = String(394);
pub const OTrs: String = String(395);
pub const OTnl: String = String(396);
pub const OTbs_s: String = String(397);
pub const OTko: String = String(398);
pub const OTma: String = String(399);
pub const OTG2: String = String(400);
pub const OTG3: String = String(401);
pub const OTG1: String = String(402);
pub const OTG4: String = String(403);
pub const OTGR: String = String(404);
pub const OTGL: String = String(405);
pub const OTGU: String = String(406);
pub const OTGD: String = String(407);
pub const OTGH: String = String(408);
pub const OTGV: String = String(409);
pub const OTGC: String = String(410);
pub const meml: String = String(411);
pub const memu: String = String(412);
pub const box1: String = String(413);

pub const back_tab: String = String(0);
pub const bell: String = String(1);
pub const carriage_return: String = String(2);
pub const change_scroll_region: String = String(3);
pub const clear_all_tabs: String = String(4);
pub const clear_screen: String = String(5);
pub const clr_eol: String = String(6);
pub const clr_eos: String = String(7);
pub const column_address: String = String(8);
pub const command_character: String = String(9);
pub const cursor_address: String = String(10);
pub const cursor_down: String = String(11);
pub const cursor_home: String = String(12);
pub const cursor_invisible: String = String(13);
pub const cursor_left: String = String(14);
pub const cursor_mem_address: String = String(15);
pub const cursor_normal: String = String(16);
pub const cursor_right: String = String(17);
pub const cursor_to_ll: String = String(18);
pub const cursor_up: String = String(19);
pub const cursor_visible: String = String(20);
pub const delete_character: String = String(21);
pub const delete_line: String = String(22);
pub const dis_status_line: String = String(23);
pub const down_half_line: String = String(24);
pub const enter_alt_charset_mode: String = String(25);
pub const enter_blink_mode: String = String(26);
pub const enter_bold_mode: String = String(27);
pub const enter_ca_mode: String = String(28);
pub const enter_delete_mode: String = String(29);
pub const enter_dim_mode: String = String(30);
pub const enter_insert_mode: String = String(31);
pub const enter_secure_mode: String = String(32);
pub const enter_protected_mode: String = String(33);
pub const enter_reverse_mode: String = String(34);
pub const enter_standout_mode: String = String(35);
pub const enter_underline_mode: String = String(36);
pub const erase_chars: String = String(37);
pub const exit_alt_charset_mode: String = String(38);
pub const exit_attribute_mode: String = String(39);
pub const exit_ca_mode: String = String(40);
pub const exit_delete_mode: String = String(41);
pub const exit_insert_mode: String = String(42);
pub const exit_standout_mode: String = String(43);
pub const exit_underline_mode: String = String(44);
pub const flash_screen: String = String(45);
pub const form_feed: String = String(46);
pub const from_status_line: String = String(47);
pub const init_1string: String = String(48);
pub const init_2string: String = String(49);
pub const init_3string: String = String(50);
pub const init_file: String = String(51);
pub const insert_character: String = String(52);
pub const insert_line: String = String(53);
pub const insert_padding: String = String(54);
pub const key_backspace: String = String(55);
pub const key_catab: String = String(56);
pub const key_clear: String = String(57);
pub const key_ctab: String = String(58);
pub const key_dc: String = String(59);
pub const key_dl: String = String(60);
pub const key_down: String = String(61);
pub const key_eic: String = String(62);
pub const key_eol: String = String(63);
pub const key_eos: String = String(64);
pub const key_f0: String = String(65);
pub const key_f1: String = String(66);
pub const key_f10: String = String(67);
pub const key_f2: String = String(68);
pub const key_f3: String = String(69);
pub const key_f4: String = String(70);
pub const key_f5: String = String(71);
pub const key_f6: String = String(72);
pub const key_f7: String = String(73);
pub const key_f8: String = String(74);
pub const key_f9: String = String(75);
pub const key_home: String = String(76);
pub const key_ic: String = String(77);
pub const key_il: String = String(78);
pub const key_left: String = String(79);
pub const key_ll: String = String(80);
pub const key_npage: String = String(81);
pub const key_ppage: String = String(82);
pub const key_right: String = String(83);
pub const key_sf: String = String(84);
pub const key_sr: String = String(85);
pub const key_stab: String = String(86);
pub const key_up: String = String(87);
pub const keypad_local: String = String(88);
pub const keypad_xmit: String = String(89);
pub const lab_f0: String = String(90);
pub const lab_f1: String = String(91);
pub const lab_f10: String = String(92);
pub const lab_f2: String = String(93);
pub const lab_f3: String = String(94);
pub const lab_f4: String = String(95);
pub const lab_f5: String = String(96);
pub const lab_f6: String = String(97);
pub const lab_f7: String = String(98);
pub const lab_f8: String = String(99);
pub const lab_f9: String = String(100);
pub const meta_off: String = String(101);
pub const meta_on: String = String(102);
pub const newline: String = String(103);
pub const pad_char: String = String(104);
pub const parm_dch: String = String(105);
pub const parm_delete_line: String = String(106);
pub const parm_down_cursor: String = String(107);
pub const parm_ich: String = String(108);
pub const parm_index: String = String(109);
pub const parm_insert_line: String = String(110);
pub const parm_left_cursor: String = String(111);
pub const parm_right_cursor: String = String(112);
pub const parm_rindex: String = String(113);
pub const parm_up_cursor: String = String(114);
pub const pkey_key: String = String(115);
pub const pkey_local: String = String(116);
pub const pkey_xmit: String = String(117);
pub const print_screen: String = String(118);
pub const prtr_off: String = String(119);
pub const prtr_on: String = String(120);
pub const repeat_char: String = String(121);
pub const reset_1string: String = String(122);
pub const reset_2string: String = String(123);
pub const reset_3string: String = String(124);
pub const reset_file: String = String(125);
pub const restore_cursor: String = String(126);
pub const row_address: String = String(127);
pub const save_cursor: String = String(128);
pub const scroll_forward: String = String(129);
pub const scroll_reverse: String = String(130);
pub const set_attributes: String = String(131);
pub const set_tab: String = String(132);
pub const set_window: String = String(133);
pub const tab: String = String(134);
pub const to_status_line: String = String(135);
pub const underline_char: String = String(136);
pub const up_half_line: String = String(137);
pub const init_prog: String = String(138);
pub const key_a1: String = String(139);
pub const key_a3: String = String(140);
pub const key_b2: String = String(141);
pub const key_c1: String = String(142);
pub const key_c3: String = String(143);
pub const prtr_non: String = String(144);
pub const char_padding: String = String(145);
pub const acs_chars: String = String(146);
pub const plab_norm: String = String(147);
pub const key_btab: String = String(148);
pub const enter_xon_mode: String = String(149);
pub const exit_xon_mode: String = String(150);
pub const enter_am_mode: String = String(151);
pub const exit_am_mode: String = String(152);
pub const xon_character: String = String(153);
pub const xoff_character: String = String(154);
pub const ena_acs: String = String(155);
pub const label_on: String = String(156);
pub const label_off: String = String(157);
pub const key_beg: String = String(158);
pub const key_cancel: String = String(159);
pub const key_close: String = String(160);
pub const key_command: String = String(161);
pub const key_copy: String = String(162);
pub const key_create: String = String(163);
pub const key_end: String = String(164);
pub const key_enter: String = String(165);
pub const key_exit: String = String(166);
pub const key_find: String = String(167);
pub const key_help: String = String(168);
pub const key_mark: String = String(169);
pub const key_message: String = String(170);
pub const key_move: String = String(171);
pub const key_next: String = String(172);
pub const key_open: String = String(173);
pub const key_options: String = String(174);
pub const key_previous: String = String(175);
pub const key_print: String = String(176);
pub const key_redo: String = String(177);
pub const key_reference: String = String(178);
pub const key_refresh: String = String(179);
pub const key_replace: String = String(180);
pub const key_restart: String = String(181);
pub const key_resume: String = String(182);
pub const key_save: String = String(183);
pub const key_suspend: String = String(184);
pub const key_undo: String = String(185);
pub const key_sbeg: String = String(186);
pub const key_scancel: String = String(187);
pub const key_scommand: String = String(188);
pub const key_scopy: String = String(189);
pub const key_screate: String = String(190);
pub const key_sdc: String = String(191);
pub const key_sdl: String = String(192);
pub const key_select: String = String(193);
pub const key_send: String = String(194);
pub const key_seol: String = String(195);
pub const key_sexit: String = String(196);
pub const key_sfind: String = String(197);
pub const key_shelp: String = String(198);
pub const key_shome: String = String(199);
pub const key_sic: String = String(200);
pub const key_sleft: String = String(201);
pub const key_smessage: String = String(202);
pub const key_smove: String = String(203);
pub const key_snext: String = String(204);
pub const key_soptions: String = String(205);
pub const key_sprevious: String = String(206);
pub const key_sprint: String = String(207);
pub const key_sredo: String = String(208);
pub const key_sreplace: String = String(209);
pub const key_sright: String = String(210);
pub const key_srsume: String = String(211);
pub const key_ssave: String = String(212);
pub const key_ssuspend: String = String(213);
pub const key_sundo: String = String(214);
pub const req_for_input: String = String(215);
pub const key_f11: String = String(216);
pub const key_f12: String = String(217);
pub const key_f13: String = String(218);
pub const key_f14: String = String(219);
pub const key_f15: String = String(220);
pub const key_f16: String = String(221);
pub const key_f17: String = String(222);
pub const key_f18: String = String(223);
pub const key_f19: String = String(224);
pub const key_f20: String = String(225);
pub const key_f21: String = String(226);
pub const key_f22: String = String(227);
pub const key_f23: String = String(228);
pub const key_f24: String = String(229);
pub const key_f25: String = String(230);
pub const key_f26: String = String(231);
pub const key_f27: String = String(232);
pub const key_f28: String = String(233);
pub const key_f29: String = String(234);
pub const key_f30: String = String(235);
pub const key_f31: String = String(236);
pub const key_f32: String = String(237);
pub const key_f33: String = String(238);
pub const key_f34: String = String(239);
pub const key_f35: String = String(240);
pub const key_f36: String = String(241);
pub const key_f37: String = String(242);
pub const key_f38: String = String(243);
pub const key_f39: String = String(244);
pub const key_f40: String = String(245);
pub const key_f41: String = String(246);
pub const key_f42: String = String(247);
pub const key_f43: String = String(248);
pub const key_f44: String = String(249);
pub const key_f45: String = String(250);
pub const key_f46: String = String(251);
pub const key_f47: String = String(252);
pub const key_f48: String = String(253);
pub const key_f49: String = String(254);
pub const key_f50: String = String(255);
pub const key_f51: String = String(256);
pub const key_f52: String = String(257);
pub const key_f53: String = String(258);
pub const key_f54: String = String(259);
pub const key_f55: String = String(260);
pub const key_f56: String = String(261);
pub const key_f57: String = String(262);
pub const key_f58: String = String(263);
pub const key_f59: String = String(264);
pub const key_f60: String = String(265);
pub const key_f61: String = String(266);
pub const key_f62: String = String(267);
pub const key_f63: String = String(268);
pub const clr_bol: String = String(269);
pub const clear_margins: String = String(270);
pub const set_left_margin: String = String(271);
pub const set_right_margin: String = String(272);
pub const label_format: String = String(273);
pub const set_clock: String = String(274);
pub const display_clock: String = String(275);
pub const remove_clock: String = String(276);
pub const create_window: String = String(277);
pub const goto_window: String = String(278);
pub const hangup: String = String(279);
pub const dial_phone: String = String(280);
pub const quick_dial: String = String(281);
//pub const tone: String = String(282); // same as short name
//pub const pulse: String = String(283); // same as short name
pub const flash_hook: String = String(284);
pub const fixed_pause: String = String(285);
pub const wait_tone: String = String(286);
pub const user0: String = String(287);
pub const user1: String = String(288);
pub const user2: String = String(289);
pub const user3: String = String(290);
pub const user4: String = String(291);
pub const user5: String = String(292);
pub const user6: String = String(293);
pub const user7: String = String(294);
pub const user8: String = String(295);
pub const user9: String = String(296);
pub const orig_pair: String = String(297);
pub const orig_colors: String = String(298);
pub const initialize_color: String = String(299);
pub const initialize_pair: String = String(300);
pub const set_color_pair: String = String(301);
pub const set_foreground: String = String(302);
pub const set_background: String = String(303);
pub const change_char_pitch: String = String(304);
pub const change_line_pitch: String = String(305);
pub const change_res_horz: String = String(306);
pub const change_res_vert: String = String(307);
pub const define_char: String = String(308);
pub const enter_doublewide_mode: String = String(309);
pub const enter_draft_quality: String = String(310);
pub const enter_italics_mode: String = String(311);
pub const enter_leftward_mode: String = String(312);
pub const enter_micro_mode: String = String(313);
pub const enter_near_letter_quality: String = String(314);
pub const enter_normal_quality: String = String(315);
pub const enter_shadow_mode: String = String(316);
pub const enter_subscript_mode: String = String(317);
pub const enter_superscript_mode: String = String(318);
pub const enter_upward_mode: String = String(319);
pub const exit_doublewide_mode: String = String(320);
pub const exit_italics_mode: String = String(321);
pub const exit_leftward_mode: String = String(322);
pub const exit_micro_mode: String = String(323);
pub const exit_shadow_mode: String = String(324);
pub const exit_subscript_mode: String = String(325);
pub const exit_superscript_mode: String = String(326);
pub const exit_upward_mode: String = String(327);
pub const micro_column_address: String = String(328);
pub const micro_down: String = String(329);
pub const micro_left: String = String(330);
pub const micro_right: String = String(331);
pub const micro_row_address: String = String(332);
pub const micro_up: String = String(333);
pub const order_of_pins: String = String(334);
pub const parm_down_micro: String = String(335);
pub const parm_left_micro: String = String(336);
pub const parm_right_micro: String = String(337);
pub const parm_up_micro: String = String(338);
pub const select_char_set: String = String(339);
pub const set_bottom_margin: String = String(340);
pub const set_bottom_margin_parm: String = String(341);
pub const set_left_margin_parm: String = String(342);
pub const set_right_margin_parm: String = String(343);
pub const set_top_margin: String = String(344);
pub const set_top_margin_parm: String = String(345);
pub const start_bit_image: String = String(346);
pub const start_char_set_def: String = String(347);
pub const stop_bit_image: String = String(348);
pub const stop_char_set_def: String = String(349);
pub const subscript_characters: String = String(350);
pub const superscript_characters: String = String(351);
pub const these_cause_cr: String = String(352);
pub const zero_motion: String = String(353);
pub const char_set_names: String = String(354);
pub const key_mouse: String = String(355);
pub const mouse_info: String = String(356);
pub const req_mouse_pos: String = String(357);
pub const get_mouse: String = String(358);
pub const set_a_foreground: String = String(359);
pub const set_a_background: String = String(360);
pub const pkey_plab: String = String(361);
pub const device_type: String = String(362);
pub const code_set_init: String = String(363);
pub const set0_des_seq: String = String(364);
pub const set1_des_seq: String = String(365);
pub const set2_des_seq: String = String(366);
pub const set3_des_seq: String = String(367);
pub const set_lr_margin: String = String(368);
pub const set_tb_margin: String = String(369);
pub const bit_image_repeat: String = String(370);
pub const bit_image_newline: String = String(371);
pub const bit_image_carriage_return: String = String(372);
pub const color_names: String = String(373);
pub const define_bit_image_region: String = String(374);
pub const end_bit_image_region: String = String(375);
pub const set_color_band: String = String(376);
pub const set_page_length: String = String(377);
pub const display_pc_char: String = String(378);
pub const enter_pc_charset_mode: String = String(379);
pub const exit_pc_charset_mode: String = String(380);
pub const enter_scancode_mode: String = String(381);
pub const exit_scancode_mode: String = String(382);
pub const pc_term_options: String = String(383);
pub const scancode_escape: String = String(384);
pub const alt_scancode_esc: String = String(385);
pub const enter_horizontal_hl_mode: String = String(386);
pub const enter_left_hl_mode: String = String(387);
pub const enter_low_hl_mode: String = String(388);
pub const enter_right_hl_mode: String = String(389);
pub const enter_top_hl_mode: String = String(390);
pub const enter_vertical_hl_mode: String = String(391);
pub const set_a_attributes: String = String(392);
pub const set_pglen_inch: String = String(393);
pub const termcap_init2: String = String(394);
pub const termcap_reset: String = String(395);
pub const linefeed_if_not_lf: String = String(396);
pub const backspace_if_not_bs: String = String(397);
pub const other_non_function_keys: String = String(398);
pub const arrow_key_map: String = String(399);
pub const acs_ulcorner: String = String(400);
pub const acs_llcorner: String = String(401);
pub const acs_urcorner: String = String(402);
pub const acs_lrcorner: String = String(403);
pub const acs_ltee: String = String(404);
pub const acs_rtee: String = String(405);
pub const acs_btee: String = String(406);
pub const acs_ttee: String = String(407);
pub const acs_hline: String = String(408);
pub const acs_vline: String = String(409);
pub const acs_plus: String = String(410);
pub const memory_lock: String = String(411);
pub const memory_unlock: String = String(412);
pub const box_chars_1: String = String(413);

pub const NUM_STRS: usize = 414; // restricted?

pub(super) static BOOLS: &[&str] = &[
    "bw", "am", "xsb", "xhp", "xenl", "eo", "gn", "hc", "km", "hs", "in_",
    "db", "da", "mir", "msgr", "os", "eslok", "xt", "hz", "ul", "xon", "nxon",
    "mc5i", "chts", "nrrmc", "npc", "ndscr", "ccc", "bce", "hls", "xhpa",
    "crxm", "daisy", "xvpa", "sam", "cpix", "lpix", "OTbs_b", "OTns", "OTnc",
    "OTMT", "OTNL", "OTpt", "OTxr",
];

pub(super) static BOOLEANS: &[&str] = &[
    "auto_left_margin",
    "auto_right_margin",
    "no_esc_ctlc",
    "ceol_standout_glitch",
    "eat_newline_glitch",
    "erase_overstrike",
    "generic_type",
    "hard_copy",
    "has_meta_key",
    "has_status_line",
    "insert_null_glitch",
    "memory_above",
    "memory_below",
    "move_insert_mode",
    "move_standout_mode",
    "over_strike",
    "status_line_esc_ok",
    "dest_tabs_magic_smso",
    "tilde_glitch",
    "transparent_underline",
    "xon_xoff",
    "needs_xon_xoff",
    "prtr_silent",
    "hard_cursor",
    "non_rev_rmcup",
    "no_pad_char",
    "non_dest_scroll_region",
    "can_change",
    "back_color_erase",
    "hue_lightness_saturation",
    "col_addr_glitch",
    "cr_cancels_micro_mode",
    "has_print_wheel",
    "row_addr_glitch",
    "semi_auto_right_margin",
    "cpi_changes_res",
    "lpi_changes_res",
    "backspaces_with_bs",
    "crt_no_scrolling",
    "no_correctly_working_cr",
    "gnu_has_meta_key",
    "linefeed_is_newline",
    "has_hardware_tabs",
    "return_does_clr_eol",
];

pub(super) static NUMS: &[&str] = &[
    "cols", "it", "lines", "lm", "xmc", "pb", "vt", "wsl", "nlab", "lh", "lw",
    "ma", "wnum", "colors", "pairs", "ncv", "bufsz", "spinv", "spinh", "maddr",
    "mjump", "mcs", "mls", "npins", "orc", "orl", "orhi", "orvi", "cps",
    "widcs", "btns", "bitwin", "bitype", "UTug", "OTdC", "OTdN", "OTdB",
    "OTdT", "OTkn",
];

pub(super) static NUMBERS: &[&str] = &[
    "columns",
    "init_tabs",
    "lines",
    "lines_of_memory",
    "magic_cookie_glitch",
    "padding_baud_rate",
    "virtual_terminal",
    "width_status_line",
    "num_labels",
    "label_height",
    "label_width",
    "max_attributes",
    "maximum_windows",
    "max_colors",
    "max_pairs",
    "no_color_video",
    "buffer_capacity",
    "dot_vert_spacing",
    "dot_horz_spacing",
    "max_micro_address",
    "max_micro_jump",
    "micro_col_size",
    "micro_line_size",
    "number_of_pins",
    "output_res_char",
    "output_res_line",
    "output_res_horz_inch",
    "output_res_vert_inch",
    "print_rate",
    "wide_char_size",
    "buttons",
    "bit_image_entwining",
    "bit_image_type",
    "magic_cookie_glitch_ul",
    "carriage_return_delay",
    "new_line_delay",
    "backspace_delay",
    "horizontal_tab_delay",
    "number_of_function_keys",
];

pub(super) static STRS: &[&str] = &[
    "cbt", "bel", "cr", "csr", "tbc", "clear", "el", "ed", "hpa", "cmdch",
    "cup", "cud1", "home", "civis", "cub1", "mrcup", "cnorm", "cuf1", "ll",
    "cuu1", "cvvis", "dch1", "dl1", "dsl", "hd", "smacs", "blink", "bold",
    "smcup", "smdc", "dim", "smir", "invis", "prot", "rev", "smso", "smul",
    "ech", "rmacs", "sgr0", "rmcup", "rmdc", "rmir", "rmso", "rmul", "flash",
    "ff", "fsl", "is1", "is2", "is3", "if_", "ich1", "il1", "ip", "kbs",
    "ktbc", "kclr", "kctab", "kdch1", "kdl1", "kcud1", "krmir", "kel", "ked",
    "kf0", "kf1", "kf10", "kf2", "kf3", "kf4", "kf5", "kf6", "kf7", "kf8",
    "kf9", "khome", "kich1", "kil1", "kcub1", "kll", "knp", "kpp", "kcuf1",
    "kind", "kri", "khts", "kcuu1", "rmkx", "smkx", "lf0", "lf1", "lf10",
    "lf2", "lf3", "lf4", "lf5", "lf6", "lf7", "lf8", "lf9", "rmm", "smm",
    "nel", "pad", "dch", "dl", "cud", "ich", "indn", "il", "cub", "cuf", "rin",
    "cuu", "pfkey", "pfloc", "pfx", "mc0", "mc4", "mc5", "rep", "rs1", "rs2",
    "rs3", "rf", "rc", "vpa", "sc", "ind", "ri", "sgr", "hts", "wind", "ht",
    "tsl", "uc", "hu", "iprog", "ka1", "ka3", "kb2", "kc1", "kc3", "mc5p",
    "rmp", "acsc", "pln", "kcbt", "smxon", "rmxon", "smam", "rmam", "xonc",
    "xoffc", "enacs", "smln", "rmln", "kbeg", "kcan", "kclo", "kcmd", "kcpy",
    "kcrt", "kend", "kent", "kext", "kfnd", "khlp", "kmrk", "kmsg", "kmov",
    "knxt", "kopn", "kopt", "kprv", "kprt", "krdo", "kref", "krfr", "krpl",
    "krst", "kres", "ksav", "kspd", "kund", "kBEG", "kCAN", "kCMD", "kCPY",
    "kCRT", "kDC", "kDL", "kslt", "kEND", "kEOL", "kEXT", "kFND", "kHLP",
    "kHOM", "kIC", "kLFT", "kMSG", "kMOV", "kNXT", "kOPT", "kPRV", "kPRT",
    "kRDO", "kRPL", "kRIT", "kRES", "kSAV", "kSPD", "kUND", "rfi", "kf11",
    "kf12", "kf13", "kf14", "kf15", "kf16", "kf17", "kf18", "kf19", "kf20",
    "kf21", "kf22", "kf23", "kf24", "kf25", "kf26", "kf27", "kf28", "kf29",
    "kf30", "kf31", "kf32", "kf33", "kf34", "kf35", "kf36", "kf37", "kf38",
    "kf39", "kf40", "kf41", "kf42", "kf43", "kf44", "kf45", "kf46", "kf47",
    "kf48", "kf49", "kf50", "kf51", "kf52", "kf53", "kf54", "kf55", "kf56",
    "kf57", "kf58", "kf59", "kf60", "kf61", "kf62", "kf63", "el1", "mgc",
    "smgl", "smgr", "fln", "sclk", "dclk", "rmclk", "cwin", "wingo", "hup",
    "dial", "qdial", "tone", "pulse", "hook", "pause", "wait", "u0", "u1",
    "u2", "u3", "u4", "u5", "u6", "u7", "u8", "u9", "op", "oc", "initc",
    "initp", "scp", "setf", "setb", "cpi", "lpi", "chr", "cvr", "defc",
    "swidm", "sdrfq", "sitm", "slm", "smicm", "snlq", "snrmq", "sshm", "ssubm",
    "ssupm", "sum", "rwidm", "ritm", "rlm", "rmicm", "rshm", "rsubm", "rsupm",
    "rum", "mhpa", "mcud1", "mcub1", "mcuf1", "mvpa", "mcuu1", "porder",
    "mcud", "mcub", "mcuf", "mcuu", "scs", "smgb", "smgbp", "smglp", "smgrp",
    "smgt", "smgtp", "sbim", "scsd", "rbim", "rcsd", "subcs", "supcs", "docr",
    "zerom", "csnm", "kmous", "minfo", "reqmp", "getm", "setaf", "setab",
    "pfxl", "devt", "csin", "s0ds", "s1ds", "s2ds", "s3ds", "smglr", "smgtb",
    "birep", "binel", "bicr", "colornm", "defbi", "endbi", "setcolor",
    "slines", "dispc", "smpch", "rmpch", "smsc", "rmsc", "pctrm", "scesc",
    "scesa", "ehhlm", "elhlm", "elohlm", "erhlm", "ethlm", "evhlm", "sgr1",
    "slength", "OTi2", "OTrs", "OTnl", "OTbs_s", "OTko", "OTma", "OTG2",
    "OTG3", "OTG1", "OTG4", "OTGR", "OTGL", "OTGU", "OTGD", "OTGH", "OTGV",
    "OTGC", "meml", "memu", "box1",
];

pub(super) static STRINGS: &[&str] = &[
    "back_tab",
    "bell",
    "carriage_return",
    "change_scroll_region",
    "clear_all_tabs",
    "clear_screen",
    "clr_eol",
    "clr_eos",
    "column_address",
    "command_character",
    "cursor_address",
    "cursor_down",
    "cursor_home",
    "cursor_invisible",
    "cursor_left",
    "cursor_mem_address",
    "cursor_normal",
    "cursor_right",
    "cursor_to_ll",
    "cursor_up",
    "cursor_visible",
    "delete_character",
    "delete_line",
    "dis_status_line",
    "down_half_line",
    "enter_alt_charset_mode",
    "enter_blink_mode",
    "enter_bold_mode",
    "enter_ca_mode",
    "enter_delete_mode",
    "enter_dim_mode",
    "enter_insert_mode",
    "enter_secure_mode",
    "enter_protected_mode",
    "enter_reverse_mode",
    "enter_standout_mode",
    "enter_underline_mode",
    "erase_chars",
    "exit_alt_charset_mode",
    "exit_attribute_mode",
    "exit_ca_mode",
    "exit_delete_mode",
    "exit_insert_mode",
    "exit_standout_mode",
    "exit_underline_mode",
    "flash_screen",
    "form_feed",
    "from_status_line",
    "init_1string",
    "init_2string",
    "init_3string",
    "init_file",
    "insert_character",
    "insert_line",
    "insert_padding",
    "key_backspace",
    "key_catab",
    "key_clear",
    "key_ctab",
    "key_dc",
    "key_dl",
    "key_down",
    "key_eic",
    "key_eol",
    "key_eos",
    "key_f0",
    "key_f1",
    "key_f10",
    "key_f2",
    "key_f3",
    "key_f4",
    "key_f5",
    "key_f6",
    "key_f7",
    "key_f8",
    "key_f9",
    "key_home",
    "key_ic",
    "key_il",
    "key_left",
    "key_ll",
    "key_npage",
    "key_ppage",
    "key_right",
    "key_sf",
    "key_sr",
    "key_stab",
    "key_up",
    "keypad_local",
    "keypad_xmit",
    "lab_f0",
    "lab_f1",
    "lab_f10",
    "lab_f2",
    "lab_f3",
    "lab_f4",
    "lab_f5",
    "lab_f6",
    "lab_f7",
    "lab_f8",
    "lab_f9",
    "meta_off",
    "meta_on",
    "newline",
    "pad_char",
    "parm_dch",
    "parm_delete_line",
    "parm_down_cursor",
    "parm_ich",
    "parm_index",
    "parm_insert_line",
    "parm_left_cursor",
    "parm_right_cursor",
    "parm_rindex",
    "parm_up_cursor",
    "pkey_key",
    "pkey_local",
    "pkey_xmit",
    "print_screen",
    "prtr_off",
    "prtr_on",
    "repeat_char",
    "reset_1string",
    "reset_2string",
    "reset_3string",
    "reset_file",
    "restore_cursor",
    "row_address",
    "save_cursor",
    "scroll_forward",
    "scroll_reverse",
    "set_attributes",
    "set_tab",
    "set_window",
    "tab",
    "to_status_line",
    "underline_char",
    "up_half_line",
    "init_prog",
    "key_a1",
    "key_a3",
    "key_b2",
    "key_c1",
    "key_c3",
    "prtr_non",
    "char_padding",
    "acs_chars",
    "plab_norm",
    "key_btab",
    "enter_xon_mode",
    "exit_xon_mode",
    "enter_am_mode",
    "exit_am_mode",
    "xon_character",
    "xoff_character",
    "ena_acs",
    "label_on",
    "label_off",
    "key_beg",
    "key_cancel",
    "key_close",
    "key_command",
    "key_copy",
    "key_create",
    "key_end",
    "key_enter",
    "key_exit",
    "key_find",
    "key_help",
    "key_mark",
    "key_message",
    "key_move",
    "key_next",
    "key_open",
    "key_options",
    "key_previous",
    "key_print",
    "key_redo",
    "key_reference",
    "key_refresh",
    "key_replace",
    "key_restart",
    "key_resume",
    "key_save",
    "key_suspend",
    "key_undo",
    "key_sbeg",
    "key_scancel",
    "key_scommand",
    "key_scopy",
    "key_screate",
    "key_sdc",
    "key_sdl",
    "key_select",
    "key_send",
    "key_seol",
    "key_sexit",
    "key_sfind",
    "key_shelp",
    "key_shome",
    "key_sic",
    "key_sleft",
    "key_smessage",
    "key_smove",
    "key_snext",
    "key_soptions",
    "key_sprevious",
    "key_sprint",
    "key_sredo",
    "key_sreplace",
    "key_sright",
    "key_srsume",
    "key_ssave",
    "key_ssuspend",
    "key_sundo",
    "req_for_input",
    "key_f11",
    "key_f12",
    "key_f13",
    "key_f14",
    "key_f15",
    "key_f16",
    "key_f17",
    "key_f18",
    "key_f19",
    "key_f20",
    "key_f21",
    "key_f22",
    "key_f23",
    "key_f24",
    "key_f25",
    "key_f26",
    "key_f27",
    "key_f28",
    "key_f29",
    "key_f30",
    "key_f31",
    "key_f32",
    "key_f33",
    "key_f34",
    "key_f35",
    "key_f36",
    "key_f37",
    "key_f38",
    "key_f39",
    "key_f40",
    "key_f41",
    "key_f42",
    "key_f43",
    "key_f44",
    "key_f45",
    "key_f46",
    "key_f47",
    "key_f48",
    "key_f49",
    "key_f50",
    "key_f51",
    "key_f52",
    "key_f53",
    "key_f54",
    "key_f55",
    "key_f56",
    "key_f57",
    "key_f58",
    "key_f59",
    "key_f60",
    "key_f61",
    "key_f62",
    "key_f63",
    "clr_bol",
    "clear_margins",
    "set_left_margin",
    "set_right_margin",
    "label_format",
    "set_clock",
    "display_clock",
    "remove_clock",
    "create_window",
    "goto_window",
    "hangup",
    "dial_phone",
    "quick_dial",
    "tone",
    "pulse",
    "flash_hook",
    "fixed_pause",
    "wait_tone",
    "user0",
    "user1",
    "user2",
    "user3",
    "user4",
    "user5",
    "user6",
    "user7",
    "user8",
    "user9",
    "orig_pair",
    "orig_colors",
    "initialize_color",
    "initialize_pair",
    "set_color_pair",
    "set_foreground",
    "set_background",
    "change_char_pitch",
    "change_line_pitch",
    "change_res_horz",
    "change_res_vert",
    "define_char",
    "enter_doublewide_mode",
    "enter_draft_quality",
    "enter_italics_mode",
    "enter_leftward_mode",
    "enter_micro_mode",
    "enter_near_letter_quality",
    "enter_normal_quality",
    "enter_shadow_mode",
    "enter_subscript_mode",
    "enter_superscript_mode",
    "enter_upward_mode",
    "exit_doublewide_mode",
    "exit_italics_mode",
    "exit_leftward_mode",
    "exit_micro_mode",
    "exit_shadow_mode",
    "exit_subscript_mode",
    "exit_superscript_mode",
    "exit_upward_mode",
    "micro_column_address",
    "micro_down",
    "micro_left",
    "micro_right",
    "micro_row_address",
    "micro_up",
    "order_of_pins",
    "parm_down_micro",
    "parm_left_micro",
    "parm_right_micro",
    "parm_up_micro",
    "select_char_set",
    "set_bottom_margin",
    "set_bottom_margin_parm",
    "set_left_margin_parm",
    "set_right_margin_parm",
    "set_top_margin",
    "set_top_margin_parm",
    "start_bit_image",
    "start_char_set_def",
    "stop_bit_image",
    "stop_char_set_def",
    "subscript_characters",
    "superscript_characters",
    "these_cause_cr",
    "zero_motion",
    "char_set_names",
    "key_mouse",
    "mouse_info",
    "req_mouse_pos",
    "get_mouse",
    "set_a_foreground",
    "set_a_background",
    "pkey_plab",
    "device_type",
    "code_set_init",
    "set0_des_seq",
    "set1_des_seq",
    "set2_des_seq",
    "set3_des_seq",
    "set_lr_margin",
    "set_tb_margin",
    "bit_image_repeat",
    "bit_image_newline",
    "bit_image_carriage_return",
    "color_names",
    "define_bit_image_region",
    "end_bit_image_region",
    "set_color_band",
    "set_page_length",
    "display_pc_char",
    "enter_pc_charset_mode",
    "exit_pc_charset_mode",
    "enter_scancode_mode",
    "exit_scancode_mode",
    "pc_term_options",
    "scancode_escape",
    "alt_scancode_esc",
    "enter_horizontal_hl_mode",
    "enter_left_hl_mode",
    "enter_low_hl_mode",
    "enter_right_hl_mode",
    "enter_top_hl_mode",
    "enter_vertical_hl_mode",
    "set_a_attributes",
    "set_pglen_inch",
    "termcap_init2",
    "termcap_reset",
    "linefeed_if_not_lf",
    "backspace_if_not_bs",
    "other_non_function_keys",
    "arrow_key_map",
    "acs_ulcorner",
    "acs_llcorner",
    "acs_urcorner",
    "acs_lrcorner",
    "acs_ltee",
    "acs_rtee",
    "acs_btee",
    "acs_ttee",
    "acs_hline",
    "acs_vline",
    "acs_plus",
    "memory_lock",
    "memory_unlock",
    "box_chars_1",
];
