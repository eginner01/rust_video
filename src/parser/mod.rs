pub mod douyin;
pub mod kuaishou;
pub mod bilibili;
pub mod redbook;
pub mod xigua;
pub mod huoshan;
pub mod pipixia;
pub mod weibo;
pub mod weishi;
pub mod zuiyou;
pub mod lvzhou;
pub mod quanmin;
pub mod lishipin;
pub mod haokan;
pub mod huya;
pub mod pipigaoxiao;
pub mod acfun;
pub mod doupai;
pub mod quanminkge;
pub mod sixroom;
pub mod xinpianchang;

use crate::models::{VideoParseInfo, VideoSource};
use anyhow::Result;
use async_trait::async_trait;

/// 视频解析器 trait
#[async_trait]
pub trait VideoParser: Send + Sync {
    /// 从分享URL解析视频信息
    async fn parse_share_url(&self, share_url: &str) -> Result<VideoParseInfo>;
    
    /// 从视频ID解析视频信息（部分平台支持）
    async fn parse_video_id(&self, _video_id: &str) -> Result<VideoParseInfo> {
        Err(anyhow::anyhow!("该平台不支持通过视频ID解析"))
    }
}

/// 根据URL自动识别平台并解析
pub async fn parse_video_share_url(share_url: &str) -> Result<VideoParseInfo> {
    // 识别平台
    let source = identify_video_source(share_url)?;
    
    // 获取对应平台的解析器
    let parser = get_parser(source)?;
    
    // 解析视频
    parser.parse_share_url(share_url).await
}

/// 根据平台和视频ID解析
pub async fn parse_video_id(source: VideoSource, video_id: &str) -> Result<VideoParseInfo> {
    let parser = get_parser(source)?;
    parser.parse_video_id(video_id).await
}

/// 从URL中识别视频平台
fn identify_video_source(url: &str) -> Result<VideoSource> {
    let sources = [
        VideoSource::DouYin,
        VideoSource::KuaiShou,
        VideoSource::BiliBili,
        VideoSource::RedBook,
        VideoSource::PiPiXia,
        VideoSource::HuoShan,
        VideoSource::WeiBo,
        VideoSource::WeiShi,
        VideoSource::LvZhou,
        VideoSource::ZuiYou,
        VideoSource::QuanMin,
        VideoSource::XiGua,
        VideoSource::LiShiPin,
        VideoSource::PiPiGaoXiao,
        VideoSource::HuYa,
        VideoSource::AcFun,
        VideoSource::DouPai,
        VideoSource::MeiPai,
        VideoSource::QuanMinKGe,
        VideoSource::SixRoom,
        VideoSource::XinPianChang,
        VideoSource::HaoKan,
    ];
    
    for source in sources {
        for domain in source.share_url_domains() {
            if url.contains(domain) {
                return Ok(source);
            }
        }
    }
    
    Err(anyhow::anyhow!("无法识别视频平台"))
}

/// 获取指定平台的解析器
fn get_parser(source: VideoSource) -> Result<Box<dyn VideoParser>> {
    match source {
        VideoSource::DouYin => Ok(Box::new(douyin::DouyinParser)),
        VideoSource::KuaiShou => Ok(Box::new(kuaishou::KuaishouParser)),
        VideoSource::BiliBili => Ok(Box::new(bilibili::BilibiliParser)),
        VideoSource::RedBook => Ok(Box::new(redbook::RedbookParser)),
        VideoSource::XiGua => Ok(Box::new(xigua::XiguaParser)),
        VideoSource::HuoShan => Ok(Box::new(huoshan::HuoshanParser)),
        VideoSource::PiPiXia => Ok(Box::new(pipixia::PipixiaParser)),
        VideoSource::WeiBo => Ok(Box::new(weibo::WeiboParser)),
        VideoSource::WeiShi => Ok(Box::new(weishi::WeishiParser)),
        VideoSource::ZuiYou => Ok(Box::new(zuiyou::ZuiyouParser)),
        VideoSource::LvZhou => Ok(Box::new(lvzhou::LvzhouParser)),
        VideoSource::QuanMin => Ok(Box::new(quanmin::QuanminParser)),
        VideoSource::LiShiPin => Ok(Box::new(lishipin::LishipinParser)),
        VideoSource::HaoKan => Ok(Box::new(haokan::HaokanParser)),
        VideoSource::HuYa => Ok(Box::new(huya::HuyaParser)),
        VideoSource::PiPiGaoXiao => Ok(Box::new(pipigaoxiao::PipigaoxiaoParser)),
        VideoSource::AcFun => Ok(Box::new(acfun::AcfunParser)),
        VideoSource::DouPai => Ok(Box::new(doupai::DoupaiParser)),
        VideoSource::QuanMinKGe => Ok(Box::new(quanminkge::QuanminkgeParser)),
        VideoSource::SixRoom => Ok(Box::new(sixroom::SixroomParser)),
        VideoSource::XinPianChang => Ok(Box::new(xinpianchang::XinpianchangParser)),
        _ => Err(anyhow::anyhow!("平台 {} 暂不支持", source.display_name())),
    }
}

/// 获取所有支持的平台列表
pub fn get_supported_platforms() -> Vec<(VideoSource, &'static str, Vec<&'static str>)> {
    vec![
        (VideoSource::DouYin, "抖音", VideoSource::DouYin.share_url_domains()),
        (VideoSource::KuaiShou, "快手", VideoSource::KuaiShou.share_url_domains()),
        (VideoSource::BiliBili, "哔哩哔哩", VideoSource::BiliBili.share_url_domains()),
        (VideoSource::RedBook, "小红书", VideoSource::RedBook.share_url_domains()),
        (VideoSource::XiGua, "西瓜视频", VideoSource::XiGua.share_url_domains()),
        (VideoSource::HuoShan, "火山视频", VideoSource::HuoShan.share_url_domains()),
        (VideoSource::PiPiXia, "皮皮虾", VideoSource::PiPiXia.share_url_domains()),
        (VideoSource::WeiBo, "微博", VideoSource::WeiBo.share_url_domains()),
        (VideoSource::WeiShi, "微视", VideoSource::WeiShi.share_url_domains()),
        (VideoSource::ZuiYou, "最右", VideoSource::ZuiYou.share_url_domains()),
        (VideoSource::LvZhou, "绿洲", VideoSource::LvZhou.share_url_domains()),
        (VideoSource::QuanMin, "全民小视频", VideoSource::QuanMin.share_url_domains()),
        (VideoSource::LiShiPin, "梨视频", VideoSource::LiShiPin.share_url_domains()),
        (VideoSource::HaoKan, "好看视频", VideoSource::HaoKan.share_url_domains()),
        (VideoSource::HuYa, "虎牙", VideoSource::HuYa.share_url_domains()),
        (VideoSource::PiPiGaoXiao, "皮皮搞笑", VideoSource::PiPiGaoXiao.share_url_domains()),
        (VideoSource::AcFun, "AcFun", VideoSource::AcFun.share_url_domains()),
        (VideoSource::DouPai, "豆拍", VideoSource::DouPai.share_url_domains()),
        (VideoSource::QuanMinKGe, "全民K歌", VideoSource::QuanMinKGe.share_url_domains()),
        (VideoSource::SixRoom, "六间房", VideoSource::SixRoom.share_url_domains()),
        (VideoSource::XinPianChang, "新片场", VideoSource::XinPianChang.share_url_domains()),
    ]
}

