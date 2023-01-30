use pnet::packet::ip::IpNextHeaderProtocol;

pub fn protocol_to_number(protocol: IpNextHeaderProtocol) -> u8 {
    match protocol {
        IpNextHeaderProtocol(0) => 0,
        IpNextHeaderProtocol(1) => 1,
        IpNextHeaderProtocol(2) => 2,
        IpNextHeaderProtocol(3) => 3,
        IpNextHeaderProtocol(4) => 4,
        IpNextHeaderProtocol(5) => 5,
        IpNextHeaderProtocol(6) => 6,
        IpNextHeaderProtocol(7) => 7,
        IpNextHeaderProtocol(8) => 8,
        IpNextHeaderProtocol(9) => 9,
        IpNextHeaderProtocol(10) => 10,
        IpNextHeaderProtocol(11) => 11,
        IpNextHeaderProtocol(12) => 12,
        IpNextHeaderProtocol(13) => 13,
        IpNextHeaderProtocol(14) => 14,
        IpNextHeaderProtocol(15) => 15,
        IpNextHeaderProtocol(16) => 16,
        IpNextHeaderProtocol(17) => 17,
        IpNextHeaderProtocol(18) => 18,
        IpNextHeaderProtocol(19) => 19,
        IpNextHeaderProtocol(20) => 20,
        IpNextHeaderProtocol(21) => 21,
        IpNextHeaderProtocol(22) => 22,
        IpNextHeaderProtocol(23) => 23,
        IpNextHeaderProtocol(24) => 24,
        IpNextHeaderProtocol(25) => 25,
        IpNextHeaderProtocol(26) => 26,
        IpNextHeaderProtocol(27) => 27,
        IpNextHeaderProtocol(28) => 28,
        IpNextHeaderProtocol(29) => 29,
        IpNextHeaderProtocol(30) => 30,
        IpNextHeaderProtocol(31) => 31,
        IpNextHeaderProtocol(32) => 32,
        IpNextHeaderProtocol(33) => 33,
        IpNextHeaderProtocol(34) => 34,
        IpNextHeaderProtocol(35) => 35,
        IpNextHeaderProtocol(36) => 36,
        IpNextHeaderProtocol(37) => 37,
        IpNextHeaderProtocol(38) => 38,
        IpNextHeaderProtocol(39) => 39,
        IpNextHeaderProtocol(40) => 40,
        IpNextHeaderProtocol(41) => 41,
        IpNextHeaderProtocol(42) => 42,
        IpNextHeaderProtocol(43) => 43,
        IpNextHeaderProtocol(44) => 44,
        IpNextHeaderProtocol(45) => 45,
        IpNextHeaderProtocol(46) => 46,
        IpNextHeaderProtocol(47) => 47,
        IpNextHeaderProtocol(48) => 48,
        IpNextHeaderProtocol(49) => 49,
        IpNextHeaderProtocol(50) => 50,
        IpNextHeaderProtocol(51) => 51,
        IpNextHeaderProtocol(52) => 52,
        IpNextHeaderProtocol(53) => 53,
        IpNextHeaderProtocol(54) => 54,
        IpNextHeaderProtocol(55) => 55,
        IpNextHeaderProtocol(56) => 56,
        IpNextHeaderProtocol(57) => 57,
        IpNextHeaderProtocol(58) => 58,
        IpNextHeaderProtocol(59) => 59,
        IpNextHeaderProtocol(60) => 60,
        IpNextHeaderProtocol(61) => 61,
        IpNextHeaderProtocol(62) => 62,
        IpNextHeaderProtocol(63) => 63,
        IpNextHeaderProtocol(64) => 64,
        IpNextHeaderProtocol(65) => 65,
        IpNextHeaderProtocol(66) => 66,
        IpNextHeaderProtocol(67) => 67,
        IpNextHeaderProtocol(68) => 68,
        IpNextHeaderProtocol(69) => 69,
        IpNextHeaderProtocol(70) => 70,
        IpNextHeaderProtocol(71) => 71,
        IpNextHeaderProtocol(72) => 72,
        IpNextHeaderProtocol(73) => 73,
        IpNextHeaderProtocol(74) => 74,
        IpNextHeaderProtocol(75) => 75,
        IpNextHeaderProtocol(76) => 76,
        IpNextHeaderProtocol(77) => 77,
        IpNextHeaderProtocol(78) => 78,
        IpNextHeaderProtocol(79) => 79,
        IpNextHeaderProtocol(80) => 80,
        IpNextHeaderProtocol(81) => 81,
        IpNextHeaderProtocol(82) => 82,
        IpNextHeaderProtocol(83) => 83,
        IpNextHeaderProtocol(84) => 84,
        IpNextHeaderProtocol(85) => 85,
        IpNextHeaderProtocol(86) => 86,
        IpNextHeaderProtocol(87) => 87,
        IpNextHeaderProtocol(88) => 88,
        IpNextHeaderProtocol(89) => 89,
        IpNextHeaderProtocol(90) => 90,
        IpNextHeaderProtocol(91) => 91,
        IpNextHeaderProtocol(92) => 92,
        IpNextHeaderProtocol(93) => 93,
        IpNextHeaderProtocol(94) => 94,
        IpNextHeaderProtocol(95) => 95,
        IpNextHeaderProtocol(96) => 96,
        IpNextHeaderProtocol(97) => 97,
        IpNextHeaderProtocol(98) => 98,
        IpNextHeaderProtocol(99) => 99,
        IpNextHeaderProtocol(100) => 100,
        IpNextHeaderProtocol(101) => 101,
        IpNextHeaderProtocol(102) => 102,
        IpNextHeaderProtocol(103) => 103,
        IpNextHeaderProtocol(104) => 104,
        IpNextHeaderProtocol(105) => 105,
        IpNextHeaderProtocol(106) => 106,
        IpNextHeaderProtocol(107) => 107,
        IpNextHeaderProtocol(108) => 108,
        IpNextHeaderProtocol(109) => 109,
        IpNextHeaderProtocol(110) => 110,
        IpNextHeaderProtocol(111) => 111,
        IpNextHeaderProtocol(112) => 112,
        IpNextHeaderProtocol(113) => 113,
        IpNextHeaderProtocol(114) => 114,
        IpNextHeaderProtocol(115) => 115,
        IpNextHeaderProtocol(116) => 116,
        IpNextHeaderProtocol(117) => 117,
        IpNextHeaderProtocol(118) => 118,
        IpNextHeaderProtocol(119) => 119,
        IpNextHeaderProtocol(120) => 120,
        IpNextHeaderProtocol(121) => 121,
        IpNextHeaderProtocol(122) => 122,
        IpNextHeaderProtocol(123) => 123,
        IpNextHeaderProtocol(124) => 124,
        IpNextHeaderProtocol(125) => 125,
        IpNextHeaderProtocol(126) => 126,
        IpNextHeaderProtocol(127) => 127,
        IpNextHeaderProtocol(128) => 128,
        IpNextHeaderProtocol(129) => 129,
        IpNextHeaderProtocol(130) => 130,
        IpNextHeaderProtocol(131) => 131,
        IpNextHeaderProtocol(132) => 132,
        IpNextHeaderProtocol(133) => 133,
        IpNextHeaderProtocol(134) => 134,
        IpNextHeaderProtocol(135) => 135,
        IpNextHeaderProtocol(136) => 136,
        IpNextHeaderProtocol(137) => 137,
        IpNextHeaderProtocol(138) => 138,
        IpNextHeaderProtocol(139) => 139,
        IpNextHeaderProtocol(140) => 140,
        IpNextHeaderProtocol(141) => 141,
        IpNextHeaderProtocol(142) => 142,
        IpNextHeaderProtocol(143) => 143,
        IpNextHeaderProtocol(144) => 144,
        IpNextHeaderProtocol(145) => 145,
        IpNextHeaderProtocol(146) => 146,
        IpNextHeaderProtocol(147) => 147,
        IpNextHeaderProtocol(148) => 148,
        IpNextHeaderProtocol(149) => 149,
        IpNextHeaderProtocol(150) => 150,
        IpNextHeaderProtocol(151) => 151,
        IpNextHeaderProtocol(152) => 152,
        IpNextHeaderProtocol(153) => 153,
        IpNextHeaderProtocol(154) => 154,
        IpNextHeaderProtocol(155) => 155,
        IpNextHeaderProtocol(156) => 156,
        IpNextHeaderProtocol(157) => 157,
        IpNextHeaderProtocol(158) => 158,
        IpNextHeaderProtocol(159) => 159,
        IpNextHeaderProtocol(160) => 160,
        IpNextHeaderProtocol(161) => 161,
        IpNextHeaderProtocol(162) => 162,
        IpNextHeaderProtocol(163) => 163,
        IpNextHeaderProtocol(164) => 164,
        IpNextHeaderProtocol(165) => 165,
        IpNextHeaderProtocol(166) => 166,
        IpNextHeaderProtocol(167) => 167,
        IpNextHeaderProtocol(168) => 168,
        IpNextHeaderProtocol(169) => 169,
        IpNextHeaderProtocol(170) => 170,
        IpNextHeaderProtocol(171) => 171,
        IpNextHeaderProtocol(172) => 172,
        IpNextHeaderProtocol(173) => 173,
        IpNextHeaderProtocol(174) => 174,
        IpNextHeaderProtocol(175) => 175,
        IpNextHeaderProtocol(176) => 176,
        IpNextHeaderProtocol(177) => 177,
        IpNextHeaderProtocol(178) => 178,
        IpNextHeaderProtocol(179) => 179,
        IpNextHeaderProtocol(180) => 180,
        IpNextHeaderProtocol(181) => 181,
        IpNextHeaderProtocol(182) => 182,
        IpNextHeaderProtocol(183) => 183,
        IpNextHeaderProtocol(184) => 184,
        IpNextHeaderProtocol(185) => 185,
        IpNextHeaderProtocol(186) => 186,
        IpNextHeaderProtocol(187) => 187,
        IpNextHeaderProtocol(188) => 188,
        IpNextHeaderProtocol(189) => 189,
        IpNextHeaderProtocol(190) => 190,
        IpNextHeaderProtocol(191) => 191,
        IpNextHeaderProtocol(192) => 192,
        IpNextHeaderProtocol(193) => 193,
        IpNextHeaderProtocol(194) => 194,
        IpNextHeaderProtocol(195) => 195,
        IpNextHeaderProtocol(196) => 196,
        IpNextHeaderProtocol(197) => 197,
        IpNextHeaderProtocol(198) => 198,
        IpNextHeaderProtocol(199) => 199,
        IpNextHeaderProtocol(200) => 200,
        IpNextHeaderProtocol(201) => 201,
        IpNextHeaderProtocol(202) => 202,
        IpNextHeaderProtocol(203) => 203,
        IpNextHeaderProtocol(204) => 204,
        IpNextHeaderProtocol(205) => 205,
        IpNextHeaderProtocol(206) => 206,
        IpNextHeaderProtocol(207) => 207,
        IpNextHeaderProtocol(208) => 208,
        IpNextHeaderProtocol(209) => 209,
        IpNextHeaderProtocol(210) => 210,
        IpNextHeaderProtocol(211) => 211,
        IpNextHeaderProtocol(212) => 212,
        IpNextHeaderProtocol(213) => 213,
        IpNextHeaderProtocol(214) => 214,
        IpNextHeaderProtocol(215) => 215,
        IpNextHeaderProtocol(216) => 216,
        IpNextHeaderProtocol(217) => 217,
        IpNextHeaderProtocol(218) => 218,
        IpNextHeaderProtocol(219) => 219,
        IpNextHeaderProtocol(220) => 220,
        IpNextHeaderProtocol(221) => 221,
        IpNextHeaderProtocol(222) => 222,
        IpNextHeaderProtocol(223) => 223,
        IpNextHeaderProtocol(224) => 224,
        IpNextHeaderProtocol(225) => 225,
        IpNextHeaderProtocol(226) => 226,
        IpNextHeaderProtocol(227) => 227,
        IpNextHeaderProtocol(228) => 228,
        IpNextHeaderProtocol(229) => 229,
        IpNextHeaderProtocol(230) => 230,
        IpNextHeaderProtocol(231) => 231,
        IpNextHeaderProtocol(232) => 232,
        IpNextHeaderProtocol(233) => 233,
        IpNextHeaderProtocol(234) => 234,
        IpNextHeaderProtocol(235) => 235,
        IpNextHeaderProtocol(236) => 236,
        IpNextHeaderProtocol(237) => 237,
        IpNextHeaderProtocol(238) => 238,
        IpNextHeaderProtocol(239) => 239,
        IpNextHeaderProtocol(240) => 240,
        IpNextHeaderProtocol(241) => 241,
        IpNextHeaderProtocol(242) => 242,
        IpNextHeaderProtocol(243) => 243,
        IpNextHeaderProtocol(244) => 244,
        IpNextHeaderProtocol(245) => 245,
        IpNextHeaderProtocol(246) => 246,
        IpNextHeaderProtocol(247) => 247,
        IpNextHeaderProtocol(248) => 248,
        IpNextHeaderProtocol(249) => 249,
        IpNextHeaderProtocol(250) => 250,
        IpNextHeaderProtocol(251) => 251,
        IpNextHeaderProtocol(252) => 252,
        IpNextHeaderProtocol(253) => 253,
        IpNextHeaderProtocol(254) => 254,
        IpNextHeaderProtocol(255) => 255,
    }
}
