# sunny-and-sometimes-cloudy

![build workflow](https://github.com/lockhart9/sunny-and-sometimes-cloudy/actions/workflows/rust.yml/badge.svg)
 
Rust practice.  
CLI tool for getting weather forecasts in Japan.


# How to build

Requires Rust build tools. See (here)[https://www.rust-lang.org/ja/tools/install] for details!
No external dependencies. It's a pure rust application.


## Build & Run

```
$ cargo build --release 

$ ./target/release/sunny-and-sometimes-cloudy -c <city_name>
```

That's it. Simple!


## Example

It dumps forecasts 3 days ahead.

```
$ ./target/release/sunny-and-sometimes-cloudy  -c さいたま
★さいたま★ の天気予報(檜山沙耶は至高)
05/06, 予報: 曇時々晴, 最低気温: -℃, 最高気温: -℃
05/07, 予報: 曇り, 最低気温: 14℃, 最高気温: 20℃
05/08, 予報: 曇時々晴, 最低気温: 13℃, 最高気温: 24℃
```


## City

Following cities in Japan are supported!

```
"稚内"
"旭川"
"留萌"
"網走"
"北見"
"紋別"
"根室"
"釧路"
"帯広"
"室蘭"
"浦河"
"札幌"
"岩見沢"
"倶知安"
"函館"
"江差"
"青森"
"むつ"
"八戸"
"盛岡"
"宮古"
"大船渡"
"仙台"
"白石"
"秋田"
"横手"
"山形"
"米沢"
"酒田"
"新庄"
"福島"
"小名浜"
"若松"
"水戸"
"土浦"
"宇都宮"
"大田原"
"前橋"
"みなかみ"
"さいたま"
"熊谷"
"秩父"
"千葉"
"銚子"
"館山"
"東京"
"大島"
"八丈島"
"父島"
"横浜"
"小田原"
"新潟"
"長岡"
"高田"
"相川"
"富山"
"伏木"
"金沢"
"輪島"
"福井"
"敦賀"
"甲府"
"河口湖"
"長野"
"松本"
"飯田"
"岐阜"
"高山"
"静岡"
"網代"
"三島"
"浜松"
"名古屋"
"豊橋"
"津"
"尾鷲"
"大津"
"彦根"
"京都"
"舞鶴"
"大阪"
"神戸"
"豊岡"
"奈良"
"風屋"
"和歌山"
"潮岬"
"鳥取"
"米子"
"松江"
"浜田"
"西郷"
"岡山"
"津山"
"広島"
"庄原"
"下関"
"山口"
"柳井"
"萩"
"徳島"
"日和佐"
"高松"
"松山"
"新居浜"
"宇和島"
"高知"
"室戸岬"
"清水"
"福岡"
"八幡"
"飯塚"
"久留米"
"佐賀"
"伊万里"
"長崎"
"佐世保"
"厳原"
"福江"
"熊本"
"阿蘇乙姫"
"牛深"
"人吉"
"大分"
"中津"
"日田"
"佐伯"
"宮崎"
"延岡"
"都城"
"高千穂"
"鹿児島"
"鹿屋"
"種子島"
"名瀬"
"那覇"
"名護"
"久米島"
"南大東"
"宮古島"
"石垣島"
"与那国島"
```
