use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::{copy, Write};
use std::path::Path;
// use uuid::Uuid;

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

async fn package(packageid: String) -> Result<()> {
    let hostname = "localhost:8443";
    let url = format!("https://{hostname}/v1/contents/object/test_user/package/{packageid}"); // テスト用のAPIエンドポイント
    let response: Vec<ReturnContentV1> = reqwest::get(url).await?.json().await?; // GETリクエストを送信してJSONレスポンスを取得
    println!("{:?}", response); // レスポンスを表示
    for item in response {
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
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        let subcommand = &args[1];
        if subcommand == "package" {
            let packageid = args[2].to_string();
            package(packageid).await?;
        }
    }
    Ok(())
}
