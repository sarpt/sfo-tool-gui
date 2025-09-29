#[derive(strum::EnumString, PartialEq, Eq, strum::Display, strum::AsRefStr, Clone, Hash)]
pub enum Keys {
  #[strum(serialize = "ACCOUNT_ID")]
  AccountId,
  #[strum(serialize = "ACCOUNTID")]
  Accountid,
  #[strum(serialize = "ANALOG_MODE")]
  AnalogMode,
  #[strum(serialize = "APP_VER")]
  AppVer,
  #[strum(serialize = "ATTRIBUTE")]
  Attribute,
  #[strum(serialize = "BOOTABLE")]
  Bootable,
  #[strum(serialize = "CATEGORY")]
  Category,
  #[strum(serialize = "CONTENT_ID")]
  ContentId,
  #[strum(serialize = "DETAIL")]
  Detail,
  #[strum(serialize = "GAMEDATA_ID")]
  GamedataId,
  #[strum(serialize = "ITEM_PRIORITY")]
  ItemPriority,
  #[strum(serialize = "LANG")]
  Lang,
  #[strum(serialize = "LICENSE")]
  License,
  #[strum(serialize = "NP_COMMUNICATION_ID")]
  NpCommunicationId,
  #[strum(serialize = "NP_COMM_ID")]
  NpCommId,
  #[strum(serialize = "PADDING")]
  Padding,
  #[strum(serialize = "PARAMS")]
  Params,
  #[strum(serialize = "PARAMS2")]
  Params2,
  #[strum(serialize = "PARENTAL_LEVEL_x")]
  ParentalLevelX,
  #[strum(serialize = "PARENTAL_LEVEL")]
  ParentalLevel,
  #[strum(serialize = "PARENTALLEVEL")]
  Parantallevel,
  #[strum(serialize = "PATCH_FILE")]
  PatchFile,
  #[strum(serialize = "PS3_SYSTEM_VER")]
  Ps3SystemVer,
  #[strum(serialize = "REGION_DENY")]
  RegionDeny,
  #[strum(serialize = "RESOLUTION")]
  Resolution,
  #[strum(serialize = "SAVEDATA_DETAIL")]
  SavedataDetail,
  #[strum(serialize = "SAVEDATA_DIRECTORY")]
  SavedataDirectory,
  #[strum(serialize = "SAVEDATA_FILE_LIST")]
  SavedataFileList,
  #[strum(serialize = "SAVEDATA_LIST_PARAM")]
  SavedataListParam,
  #[strum(serialize = "SAVEDATA_PARAMS")]
  SavedataParams,
  #[strum(serialize = "SAVEDATA_TITLE")]
  SavedataTitle,
  #[strum(serialize = "SOUND_FORMAT")]
  SoundFormat,
  #[strum(serialize = "SOURCE")]
  Source,
  #[strum(serialize = "SUB_TITLE")]
  SubTitle,
  #[strum(serialize = "TARGET_APP_VER")]
  TargetAppVer,
  #[strum(serialize = "TITLE")]
  Title,
  #[strum(serialize = "TITLE_ID")]
  TitleId,
  #[strum(serialize = "TITLE_XX")]
  TitleXx,
  #[strum(serialize = "TITLEID0XX")]
  Titleid0xx,
  #[strum(serialize = "VERSION")]
  Version,
  #[strum(serialize = "XMB_APPS")]
  XmbApps,
  #[strum(serialize = "{0}", default)]
  Unknown(String),
}
