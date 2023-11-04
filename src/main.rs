use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::copy;
use std::path::Path;
use structopt::StructOpt;

#[cfg(feature = "hostname_local")]
const HOSTNAME: &str = "localhost:8443";

#[cfg(feature = "hostname_dev")]
const HOSTNAME: &str = ""; //localhost以外の開発用サーバ

// どちらのfeatureも無効な場合のデフォルト値
#[cfg(not(any(feature = "hostname_local", feature = "hostname_dev")))]
const HOSTNAME: &str = "virtuai.art";

#[derive(Debug, Serialize, Deserialize)]
pub struct ReturnContentV1 {
    // id: Uuid, // パッケージID
    // title: String, // タイトル
    // content: String, // コンテンツの分類
    obj: String, // オブジェクトまたはサムネのダウンロードURL（一定期間）
                 // price: Vec<common::PriceV1>, // (値段, 通貨（solanaや円、$など）)
                 // user_ids: Vec<Uuid>, // ユーザのIDs
                 // group_ids: Vec<Uuid>, // グループのIDs（一応）
                 // // dependency_ids: Vec<common::ContentV1>, // 依存関係にあるコンテンツ
                 // // dependent_ids: Vec<common::ContentV1>, // 依存されているコンテンツ
                 // version: u32, // プロトコルバージョン 1,2,3...
                 // good: Vec<Uuid>, // いいねしたユーザID一覧
                 // comments: Vec<Uuid>, // コメントID一覧
                 // // allow: common::AllowV1, // 許可設定
                 // discription: String, // 説明
                 // tag: Vec<String>, // タグ一覧
                 // custom_divide_contents: Vec<common::RateMapV1>, // カスタム分配率（contentsID: 割合 の配列）
}

#[derive(StructOpt, Debug)]
enum Cli {
    Package {
        packageid: String,
        #[structopt(short, long, default_value = "100")]
        limit: usize,
    },
}

async fn package(packageid: String, limit: usize) -> Result<()> {
    let url = format!(r#"https://{HOSTNAME}/graphql?query=
    query{{
        content(id: "{packageid}", author: "testUser", contentType: "package") {{
            id
            title
            content
            obj
            pageView
        }}
    }}"#); // テスト用のAPIエンドポイント
    dbg!(&url);
    let response: Vec<ReturnContentV1> = reqwest::get(url).await?.json().await?; // GETリクエストを送信してJSONレスポンスを取得
    println!("{:?}", response); // レスポンスを表示
    for item in response.iter().take(limit) {
        let imageurl = item.obj.to_string();
        println!("download {:?}", imageurl);
        let response2 = reqwest::get(&imageurl).await?; // GETリクエストを送信してJSONレスポンスを取得
        let content = response2.bytes().await?;
        let file_path = Path::new(&imageurl);
        let mut ext = "dat".to_string();
        if let Some(extension) = file_path.extension() {
            if let Some(extension_str) = extension.to_str() {
                ext = extension_str.to_string();
            }
        }
        let filename = format!(
            "z{}.{ext}",
            chrono::Utc::now().format("%Y%m%d%H%M%S%.f").to_string()
        );
        let mut dest = File::create(filename)?; // ファイルを作成して開く
        copy(&mut content.as_ref(), &mut dest)?; // レスポンスのデータをファイルに書き込む
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Cli::from_args();
    dbg!(&args);
    match args {
        Cli::Package { packageid, limit } => {
            package(packageid, limit).await?;
        }
    }
    Ok(())
}
