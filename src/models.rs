use serde::{Deserialize, Serialize};

/// 作者信息
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Author {
    /// 作者ID
    pub uid: String,
    /// 作者名称
    pub name: String,
    /// 作者头像URL
    pub avatar: String,
}

/// 图片信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImgInfo {
    /// 图片URL
    pub url: String,
    /// LivePhoto视频地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub live_photo_url: Option<String>,
}

/// 视频解析信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VideoParseInfo {
    /// 作者信息
    pub author: Author,
    /// 视频标题/描述
    pub title: String,
    /// 视频播放地址（无水印）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
    /// 音乐播放地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub music_url: Option<String>,
    /// 视频封面地址
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cover_url: Option<String>,
    /// 图集图片地址列表
    #[serde(default)]
    pub images: Vec<ImgInfo>,
}

impl VideoParseInfo {
    pub fn new() -> Self {
        Self {
            author: Author::default(),
            title: String::new(),
            video_url: None,
            music_url: None,
            cover_url: None,
            images: Vec::new(),
        }
    }
}

impl Default for VideoParseInfo {
    fn default() -> Self {
        Self::new()
    }
}

/// 视频平台来源
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum VideoSource {
    /// 抖音
    DouYin,
    /// 快手
    KuaiShou,
    /// 皮皮虾
    PiPiXia,
    /// 火山
    HuoShan,
    /// 微博
    WeiBo,
    /// 微视
    WeiShi,
    /// 绿洲
    LvZhou,
    /// 最右
    ZuiYou,
    /// 度小视(原全民小视频)
    QuanMin,
    /// 西瓜
    XiGua,
    /// 梨视频
    LiShiPin,
    /// 皮皮搞笑
    PiPiGaoXiao,
    /// 虎牙
    HuYa,
    /// AcFun
    AcFun,
    /// 逗拍
    DouPai,
    /// 美拍
    MeiPai,
    /// 全民K歌
    QuanMinKGe,
    /// 六间房
    SixRoom,
    /// 新片场
    XinPianChang,
    /// 好看视频
    HaoKan,
    /// 小红书
    RedBook,
    /// 哔哩哔哩
    BiliBili,
}

impl VideoSource {
    /// 从字符串解析视频来源
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "douyin" => Some(Self::DouYin),
            "kuaishou" => Some(Self::KuaiShou),
            "pipixia" => Some(Self::PiPiXia),
            "huoshan" => Some(Self::HuoShan),
            "weibo" => Some(Self::WeiBo),
            "weishi" => Some(Self::WeiShi),
            "lvzhou" => Some(Self::LvZhou),
            "zuiyou" => Some(Self::ZuiYou),
            "quanmin" => Some(Self::QuanMin),
            "xigua" => Some(Self::XiGua),
            "lishipin" => Some(Self::LiShiPin),
            "pipigaoxiao" => Some(Self::PiPiGaoXiao),
            "huya" => Some(Self::HuYa),
            "acfun" => Some(Self::AcFun),
            "doupai" => Some(Self::DouPai),
            "meipai" => Some(Self::MeiPai),
            "quanminkge" => Some(Self::QuanMinKGe),
            "sixroom" => Some(Self::SixRoom),
            "xinpianchang" => Some(Self::XinPianChang),
            "haokan" => Some(Self::HaoKan),
            "redbook" | "xiaohongshu" => Some(Self::RedBook),
            "bilibili" => Some(Self::BiliBili),
            _ => None,
        }
    }

    /// 转换为字符串
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DouYin => "douyin",
            Self::KuaiShou => "kuaishou",
            Self::PiPiXia => "pipixia",
            Self::HuoShan => "huoshan",
            Self::WeiBo => "weibo",
            Self::WeiShi => "weishi",
            Self::LvZhou => "lvzhou",
            Self::ZuiYou => "zuiyou",
            Self::QuanMin => "quanmin",
            Self::XiGua => "xigua",
            Self::LiShiPin => "lishipin",
            Self::PiPiGaoXiao => "pipigaoxiao",
            Self::HuYa => "huya",
            Self::AcFun => "acfun",
            Self::DouPai => "doupai",
            Self::MeiPai => "meipai",
            Self::QuanMinKGe => "quanminkge",
            Self::SixRoom => "sixroom",
            Self::XinPianChang => "xinpianchang",
            Self::HaoKan => "haokan",
            Self::RedBook => "redbook",
            Self::BiliBili => "bilibili",
        }
    }

    /// 获取视频来源的中文名称
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::DouYin => "抖音",
            Self::KuaiShou => "快手",
            Self::PiPiXia => "皮皮虾",
            Self::HuoShan => "火山",
            Self::WeiBo => "微博",
            Self::WeiShi => "微视",
            Self::LvZhou => "绿洲",
            Self::ZuiYou => "最右",
            Self::QuanMin => "度小视",
            Self::XiGua => "西瓜",
            Self::LiShiPin => "梨视频",
            Self::PiPiGaoXiao => "皮皮搞笑",
            Self::HuYa => "虎牙",
            Self::AcFun => "AcFun",
            Self::DouPai => "逗拍",
            Self::MeiPai => "美拍",
            Self::QuanMinKGe => "全民K歌",
            Self::SixRoom => "六间房",
            Self::XinPianChang => "新片场",
            Self::HaoKan => "好看视频",
            Self::RedBook => "小红书",
            Self::BiliBili => "哔哩哔哩",
        }
    }

    /// 获取视频分享域名列表
    pub fn share_url_domains(&self) -> Vec<&'static str> {
        match self {
            Self::DouYin => vec!["v.douyin.com", "www.iesdouyin.com", "www.douyin.com"],
            Self::KuaiShou => vec!["v.kuaishou.com", "www.kuaishou.com"],
            Self::PiPiXia => vec!["h5.pipix.com"],
            Self::HuoShan => vec!["share.huoshan.com"],
            Self::WeiBo => vec!["weibo.com"],
            Self::WeiShi => vec!["isee.weishi.qq.com"],
            Self::LvZhou => vec!["weibo.cn"],
            Self::ZuiYou => vec!["share.xiaochuankeji.cn"],
            Self::QuanMin => vec!["xspshare.baidu.com"],
            Self::XiGua => vec!["v.ixigua.com"],
            Self::LiShiPin => vec!["www.pearvideo.com"],
            Self::PiPiGaoXiao => vec!["h5.pipigx.com"],
            Self::HuYa => vec!["v.huya.com"],
            Self::AcFun => vec!["www.acfun.cn"],
            Self::DouPai => vec!["doupai.cc"],
            Self::MeiPai => vec!["meipai.com"],
            Self::QuanMinKGe => vec!["kg.qq.com"],
            Self::SixRoom => vec!["6.cn"],
            Self::XinPianChang => vec!["xinpianchang.com"],
            Self::HaoKan => vec!["haokan.baidu.com", "haokan.hao123.com"],
            Self::RedBook => vec!["www.xiaohongshu.com", "xhslink.com"],
            Self::BiliBili => vec!["bilibili.com", "b23.tv"],
        }
    }
}

/// HTTP响应结构
#[derive(Debug, Serialize, Deserialize)]
pub struct HttpResponse<T> {
    pub code: i32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> HttpResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            code: 200,
            msg: "解析成功".to_string(),
            data: Some(data),
        }
    }

    pub fn error(msg: String) -> Self {
        Self {
            code: 201,
            msg,
            data: None,
        }
    }
}

