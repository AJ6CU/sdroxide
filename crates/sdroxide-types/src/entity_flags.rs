//! Which national flag stands for each DXCC entity.
//!
//! The country file names entities but says nothing about flags, so this is a
//! hand-built table from each entity's *primary prefix* — cty.dat's field 8,
//! the stable DXCC identifier — to the flag file that represents it. The codes
//! are ISO 3166-1 alpha-2 where one exists, ISO 3166-2 subdivisions where a
//! DXCC entity is a piece of a country that flies its own flag (Alaska,
//! Hawaii, Scotland, Wales), and a handful of the ISO user-assigned codes the
//! flag set uses for territories ISO 3166-1 skips (`AC` Ascension, `TA`
//! Tristan da Cunha, `CP` Clipperton, `XK` Kosovo, `UN` the United Nations).
//!
//! Keyed by prefix rather than by name because the names in the country file
//! get rewritten — "Swaziland" became "Kingdom of Eswatini" — while the prefix
//! that identifies the entity does not.
//!
//! Islands and dependencies fly the flag of whoever administers them, which is
//! what an operator reading a decode list wants: Rotuma shows Fiji, the
//! Kerguelens show the French Southern Territories. Two entities have no flag
//! anyone flies — the Spratlys and Scarborough Reef are disputed, and the
//! Sovereign Military Order of Malta's is not in a public-domain flag set — so
//! they resolve to none and the list simply shows no flag for them.

/// DXCC primary prefix → flag code, in the country file's own order.
///
/// Scanned linearly, but only once per entity while the country file is being
/// parsed — never per callsign — so the order is the one that makes the table
/// easy to diff against cty.dat rather than one that makes lookup fast.
static FLAGS: &[(&str, &str)] = &[
    ("1S", ""),          // Spratly Islands — disputed, no flag
    ("1A", ""),          // Sov Mil Order of Malta — not in a public-domain set
    ("3A", "MC"),        // Monaco
    ("3B6", "MU"),       // Agalega & St. Brandon
    ("3B8", "MU"),       // Mauritius
    ("3B9", "MU"),       // Rodriguez Island
    ("3C", "GQ"),        // Equatorial Guinea
    ("3C0", "GQ"),       // Annobon Island
    ("3D2", "FJ"),       // Fiji
    ("3D2/c", "FJ"),     // Conway Reef
    ("3D2/r", "FJ"),     // Rotuma Island
    ("3DA", "SZ"),       // Kingdom of Eswatini
    ("3V", "TN"),        // Tunisia
    ("3W", "VN"),        // Vietnam
    ("3X", "GN"),        // Guinea
    ("3Y/b", "BV"),      // Bouvet
    ("3Y/p", "NO"),      // Peter 1 Island — Norwegian dependency
    ("4J", "AZ"),        // Azerbaijan
    ("4L", "GE"),        // Georgia
    ("4O", "ME"),        // Montenegro
    ("4S", "LK"),        // Sri Lanka
    ("4U1I", "UN"),      // ITU HQ — a UN agency
    ("4U1U", "UN"),      // United Nations HQ
    ("4U1V", "UN"),      // Vienna Intl Ctr
    ("4W", "TL"),        // Timor - Leste
    ("4X", "IL"),        // Israel
    ("5A", "LY"),        // Libya
    ("5B", "CY"),        // Cyprus
    ("5H", "TZ"),        // Tanzania
    ("5N", "NG"),        // Nigeria
    ("5R", "MG"),        // Madagascar
    ("5T", "MR"),        // Mauritania
    ("5U", "NE"),        // Niger
    ("5V", "TG"),        // Togo
    ("5W", "WS"),        // Samoa
    ("5X", "UG"),        // Uganda
    ("5Z", "KE"),        // Kenya
    ("6W", "SN"),        // Senegal
    ("6Y", "JM"),        // Jamaica
    ("7O", "YE"),        // Yemen
    ("7P", "LS"),        // Lesotho
    ("7Q", "MW"),        // Malawi
    ("7X", "DZ"),        // Algeria
    ("8P", "BB"),        // Barbados
    ("8Q", "MV"),        // Maldives
    ("8R", "GY"),        // Guyana
    ("9A", "HR"),        // Croatia
    ("9G", "GH"),        // Ghana
    ("9H", "MT"),        // Malta
    ("9J", "ZM"),        // Zambia
    ("9K", "KW"),        // Kuwait
    ("9L", "SL"),        // Sierra Leone
    ("9M2", "MY"),       // West Malaysia
    ("9M6", "MY"),       // East Malaysia
    ("9N", "NP"),        // Nepal
    ("9Q", "CD"),        // Dem. Rep. of the Congo
    ("9U", "BI"),        // Burundi
    ("9V", "SG"),        // Singapore
    ("9X", "RW"),        // Rwanda
    ("9Y", "TT"),        // Trinidad & Tobago
    ("A2", "BW"),        // Botswana
    ("A3", "TO"),        // Tonga
    ("A4", "OM"),        // Oman
    ("A5", "BT"),        // Bhutan
    ("A6", "AE"),        // United Arab Emirates
    ("A7", "QA"),        // Qatar
    ("A9", "BH"),        // Bahrain
    ("AP", "PK"),        // Pakistan
    ("BS7", ""),         // Scarborough Reef — disputed, no flag
    ("BV", "TW"),        // Taiwan
    ("BV9P", "TW"),      // Pratas Island
    ("BY", "CN"),        // China
    ("C2", "NR"),        // Nauru
    ("C3", "AD"),        // Andorra
    ("C5", "GM"),        // The Gambia
    ("C6", "BS"),        // Bahamas
    ("C9", "MZ"),        // Mozambique
    ("CE", "CL"),        // Chile
    ("CE0X", "CL"),      // San Felix & San Ambrosio
    ("CE0Y", "CL"),      // Easter Island
    ("CE0Z", "CL"),      // Juan Fernandez Islands
    ("CE9", "AQ"),       // Antarctica
    ("CM", "CU"),        // Cuba
    ("CN", "MA"),        // Morocco
    ("CP", "BO"),        // Bolivia
    ("CT", "PT"),        // Portugal
    ("CT3", "PT"),       // Madeira Islands
    ("CU", "PT"),        // Azores
    ("CX", "UY"),        // Uruguay
    ("CY0", "CA"),       // Sable Island
    ("CY9", "CA"),       // St. Paul Island
    ("D2", "AO"),        // Angola
    ("D4", "CV"),        // Cape Verde
    ("D6", "KM"),        // Comoros
    ("DL", "DE"),        // Fed. Rep. of Germany
    ("DU", "PH"),        // Philippines
    ("E3", "ER"),        // Eritrea
    ("E4", "PS"),        // Palestine
    ("E5/n", "CK"),      // North Cook Islands
    ("E5/s", "CK"),      // South Cook Islands
    ("E6", "NU"),        // Niue
    ("E7", "BA"),        // Bosnia-Herzegovina
    ("EA", "ES"),        // Spain
    ("EA6", "ES-IB"),    // Balearic Islands
    ("EA8", "ES-CN"),    // Canary Islands
    ("EA9", "ES-CE"),    // Ceuta & Melilla
    ("EI", "IE"),        // Ireland
    ("EK", "AM"),        // Armenia
    ("EL", "LR"),        // Liberia
    ("EP", "IR"),        // Iran
    ("ER", "MD"),        // Moldova
    ("ES", "EE"),        // Estonia
    ("ET", "ET"),        // Ethiopia
    ("EU", "BY"),        // Belarus
    ("EX", "KG"),        // Kyrgyzstan
    ("EY", "TJ"),        // Tajikistan
    ("EZ", "TM"),        // Turkmenistan
    ("F", "FR"),         // France
    ("FG", "GP"),        // Guadeloupe
    ("FH", "YT"),        // Mayotte
    ("FJ", "BL"),        // St. Barthelemy
    ("FK", "NC"),        // New Caledonia
    ("FK/c", "NC"),      // Chesterfield Islands
    ("FM", "MQ"),        // Martinique
    ("FO", "PF"),        // French Polynesia
    ("FO/a", "PF"),      // Austral Islands
    ("FO/c", "CP"),      // Clipperton Island
    ("FO/m", "PF"),      // Marquesas Islands
    ("FP", "PM"),        // St. Pierre & Miquelon
    ("FR", "RE"),        // Reunion Island
    ("FS", "MF"),        // St. Martin
    ("FT/g", "TF"),      // Glorioso Islands
    ("FT/j", "TF"),      // Juan de Nova, Europa
    ("FT/t", "TF"),      // Tromelin Island
    ("FT/w", "TF"),      // Crozet Island
    ("FT/x", "TF"),      // Kerguelen Islands
    ("FT/z", "TF"),      // Amsterdam & St. Paul Is.
    ("FW", "WF"),        // Wallis & Futuna Islands
    ("FY", "GF"),        // French Guiana
    ("G", "GB-ENG"),     // England
    ("GD", "IM"),        // Isle of Man
    ("GI", "GB-NIR"),    // Northern Ireland
    ("GJ", "JE"),        // Jersey
    ("GM", "GB-SCT"),    // Scotland
    ("GM/s", "GB-SCT"),  // Shetland Islands
    ("GU", "GG"),        // Guernsey
    ("GW", "GB-WLS"),    // Wales
    ("H4", "SB"),        // Solomon Islands
    ("H40", "SB"),       // Temotu Province
    ("HA", "HU"),        // Hungary
    ("HB", "CH"),        // Switzerland
    ("HB0", "LI"),       // Liechtenstein
    ("HC", "EC"),        // Ecuador
    ("HC8", "EC"),       // Galapagos Islands
    ("HH", "HT"),        // Haiti
    ("HI", "DO"),        // Dominican Republic
    ("HK", "CO"),        // Colombia
    ("HK0/a", "CO-SAP"), // San Andres & Providencia
    ("HK0/m", "CO"),     // Malpelo Island
    ("HL", "KR"),        // Republic of Korea
    ("HP", "PA"),        // Panama
    ("HR", "HN"),        // Honduras
    ("HS", "TH"),        // Thailand
    ("HV", "VA"),        // Vatican City
    ("HZ", "SA"),        // Saudi Arabia
    ("I", "IT"),         // Italy
    ("IG9", "IT"),       // African Italy
    ("IS", "IT"),        // Sardinia
    ("IT9", "IT"),       // Sicily
    ("J2", "DJ"),        // Djibouti
    ("J3", "GD"),        // Grenada
    ("J5", "GW"),        // Guinea-Bissau
    ("J6", "LC"),        // St. Lucia
    ("J7", "DM"),        // Dominica
    ("J8", "VC"),        // St. Vincent
    ("JA", "JP"),        // Japan
    ("JD/m", "JP"),      // Minami Torishima
    ("JD/o", "JP"),      // Ogasawara
    ("JT", "MN"),        // Mongolia
    ("JW", "SJ"),        // Svalbard
    ("JW/b", "SJ"),      // Bear Island
    ("JX", "SJ"),        // Jan Mayen
    ("JY", "JO"),        // Jordan
    ("K", "US"),         // United States
    ("KG4", "US"),       // Guantanamo Bay
    ("KH0", "MP"),       // Mariana Islands
    ("KH1", "UM"),       // Baker & Howland Islands
    ("KH2", "GU"),       // Guam
    ("KH3", "UM"),       // Johnston Island
    ("KH4", "UM"),       // Midway Island
    ("KH5", "UM"),       // Palmyra & Jarvis Islands
    ("KH6", "US-HI"),    // Hawaii
    ("KH7K", "US-HI"),   // Kure Island
    ("KH8", "AS"),       // American Samoa
    ("KH8/s", "AS"),     // Swains Island
    ("KH9", "UM"),       // Wake Island
    ("KL", "US-AK"),     // Alaska
    ("KP1", "UM"),       // Navassa Island
    ("KP2", "VI"),       // US Virgin Islands
    ("KP4", "PR"),       // Puerto Rico
    ("KP5", "PR"),       // Desecheo Island
    ("LA", "NO"),        // Norway
    ("LU", "AR"),        // Argentina
    ("LX", "LU"),        // Luxembourg
    ("LY", "LT"),        // Lithuania
    ("LZ", "BG"),        // Bulgaria
    ("OA", "PE"),        // Peru
    ("OD", "LB"),        // Lebanon
    ("OE", "AT"),        // Austria
    ("OH", "FI"),        // Finland
    ("OH0", "AX"),       // Aland Islands
    ("OJ0", "FI"),       // Market Reef
    ("OK", "CZ"),        // Czech Republic
    ("OM", "SK"),        // Slovak Republic
    ("ON", "BE"),        // Belgium
    ("OX", "GL"),        // Greenland
    ("OY", "FO"),        // Faroe Islands
    ("OZ", "DK"),        // Denmark
    ("P2", "PG"),        // Papua New Guinea
    ("P4", "AW"),        // Aruba
    ("P5", "KP"),        // DPR of Korea
    ("PA", "NL"),        // Netherlands
    ("PJ2", "CW"),       // Curacao
    ("PJ4", "BQ"),       // Bonaire
    ("PJ5", "BQ"),       // Saba & St. Eustatius
    ("PJ7", "SX"),       // Sint Maarten
    ("PY", "BR"),        // Brazil
    ("PY0F", "BR"),      // Fernando de Noronha
    ("PY0S", "BR"),      // St. Peter & St. Paul
    ("PY0T", "BR"),      // Trindade & Martim Vaz
    ("PZ", "SR"),        // Suriname
    ("R1FJ", "RU"),      // Franz Josef Land
    ("S0", "EH"),        // Western Sahara
    ("S2", "BD"),        // Bangladesh
    ("S5", "SI"),        // Slovenia
    ("S7", "SC"),        // Seychelles
    ("S9", "ST"),        // Sao Tome & Principe
    ("SM", "SE"),        // Sweden
    ("SP", "PL"),        // Poland
    ("ST", "SD"),        // Sudan
    ("SU", "EG"),        // Egypt
    ("SV", "GR"),        // Greece
    ("SV/a", "GR"),      // Mount Athos
    ("SV5", "GR"),       // Dodecanese
    ("SV9", "GR"),       // Crete
    ("T2", "TV"),        // Tuvalu
    ("T30", "KI"),       // Western Kiribati
    ("T31", "KI"),       // Central Kiribati
    ("T32", "KI"),       // Eastern Kiribati
    ("T33", "KI"),       // Banaba Island
    ("T5", "SO"),        // Somalia
    ("T7", "SM"),        // San Marino
    ("T8", "PW"),        // Palau
    ("TA", "TR"),        // Asiatic Turkey
    ("TA1", "TR"),       // European Turkey
    ("TF", "IS"),        // Iceland
    ("TG", "GT"),        // Guatemala
    ("TI", "CR"),        // Costa Rica
    ("TI9", "CR"),       // Cocos Island
    ("TJ", "CM"),        // Cameroon
    ("TK", "FR"),        // Corsica
    ("TL", "CF"),        // Central African Republic
    ("TN", "CG"),        // Republic of the Congo
    ("TR", "GA"),        // Gabon
    ("TT", "TD"),        // Chad
    ("TU", "CI"),        // Cote d'Ivoire
    ("TY", "BJ"),        // Benin
    ("TZ", "ML"),        // Mali
    ("UA", "RU"),        // European Russia
    ("UA2", "RU"),       // Kaliningrad
    ("UA9", "RU"),       // Asiatic Russia
    ("UK", "UZ"),        // Uzbekistan
    ("UN", "KZ"),        // Kazakhstan
    ("UR", "UA"),        // Ukraine
    ("V2", "AG"),        // Antigua & Barbuda
    ("V3", "BZ"),        // Belize
    ("V4", "KN"),        // St. Kitts & Nevis
    ("V5", "NA"),        // Namibia
    ("V6", "FM"),        // Micronesia
    ("V7", "MH"),        // Marshall Islands
    ("V8", "BN"),        // Brunei Darussalam
    ("VE", "CA"),        // Canada
    ("VK", "AU"),        // Australia
    ("VK0H", "HM"),      // Heard Island
    ("VK0M", "AU-TAS"),  // Macquarie Island — part of Tasmania
    ("VK9C", "CC"),      // Cocos (Keeling) Islands
    ("VK9L", "AU"),      // Lord Howe Island
    ("VK9M", "AU"),      // Mellish Reef
    ("VK9N", "NF"),      // Norfolk Island
    ("VK9W", "AU"),      // Willis Island
    ("VK9X", "CX"),      // Christmas Island
    ("VP2E", "AI"),      // Anguilla
    ("VP2M", "MS"),      // Montserrat
    ("VP2V", "VG"),      // British Virgin Islands
    ("VP5", "TC"),       // Turks & Caicos Islands
    ("VP6", "PN"),       // Pitcairn Island
    ("VP6/d", "PN"),     // Ducie Island
    ("VP8", "FK"),       // Falkland Islands
    ("VP8/g", "GS"),     // South Georgia Island
    ("VP8/h", "AQ"),     // South Shetland Islands
    ("VP8/o", "AQ"),     // South Orkney Islands
    ("VP8/s", "GS"),     // South Sandwich Islands
    ("VP9", "BM"),       // Bermuda
    ("VQ9", "IO"),       // Chagos Islands
    ("VR", "HK"),        // Hong Kong
    ("VU", "IN"),        // India
    ("VU4", "IN"),       // Andaman & Nicobar Is.
    ("VU7", "IN"),       // Lakshadweep Islands
    ("XE", "MX"),        // Mexico
    ("XF4", "MX"),       // Revillagigedo
    ("XT", "BF"),        // Burkina Faso
    ("XU", "KH"),        // Cambodia
    ("XW", "LA"),        // Laos
    ("XX9", "MO"),       // Macao
    ("XZ", "MM"),        // Myanmar
    ("YA", "AF"),        // Afghanistan
    ("YB", "ID"),        // Indonesia
    ("YI", "IQ"),        // Iraq
    ("YJ", "VU"),        // Vanuatu
    ("YK", "SY"),        // Syria
    ("YL", "LV"),        // Latvia
    ("YN", "NI"),        // Nicaragua
    ("YO", "RO"),        // Romania
    ("YS", "SV"),        // El Salvador
    ("YU", "RS"),        // Serbia
    ("YV", "VE"),        // Venezuela
    ("YV0", "VE"),       // Aves Island
    ("Z2", "ZW"),        // Zimbabwe
    ("Z3", "MK"),        // North Macedonia
    ("Z6", "XK"),        // Republic of Kosovo
    ("Z8", "SS"),        // Republic of South Sudan
    ("ZA", "AL"),        // Albania
    ("ZB", "GI"),        // Gibraltar
    ("ZC4", "GB"),       // UK Base Areas on Cyprus
    ("ZD7", "SH"),       // St. Helena
    ("ZD8", "AC"),       // Ascension Island
    ("ZD9", "TA"),       // Tristan da Cunha & Gough
    ("ZF", "KY"),        // Cayman Islands
    ("ZK3", "TK"),       // Tokelau Islands
    ("ZL", "NZ"),        // New Zealand
    ("ZL7", "NZ"),       // Chatham Islands
    ("ZL8", "NZ"),       // Kermadec Islands
    ("ZL9", "NZ"),       // N.Z. Subantarctic Is.
    ("ZP", "PY"),        // Paraguay
    ("ZS", "ZA"),        // South Africa
    ("ZS8", "ZA"),       // Pr. Edward & Marion Is.
];

/// The flag code for a DXCC primary prefix, or `""` when the entity flies no
/// flag we ship (and for any prefix that is not an entity's primary one).
pub(crate) fn flag_for_prefix(prefix: &str) -> &'static str {
    FLAGS.iter().find(|(p, _)| *p == prefix).map(|(_, f)| *f).unwrap_or("")
}

/// Whether the table has a row for this prefix at all — a flagless entity says
/// so with an empty code, which [`flag_for_prefix`] cannot distinguish from a
/// prefix nobody wrote a row for. Only the coverage test needs the difference.
#[cfg(test)]
pub(crate) fn covers_prefix(prefix: &str) -> bool {
    FLAGS.iter().any(|(p, _)| *p == prefix)
}

/// Every prefix the table has a row for, so the coverage test can also catch a
/// row that matches no entity.
#[cfg(test)]
pub(crate) fn all_prefixes() -> Vec<&'static str> {
    FLAGS.iter().map(|(p, _)| *p).collect()
}
