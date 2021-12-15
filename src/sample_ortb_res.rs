pub const INSTREAM_PRE_ROLL: &str = r#"{
  "id": "vast-response-from-msan",
  "seatbid": [
    {
      "seat": "vast_seat1",
      "bid": [
        {
          "impid": "1",
          "price": 9.051027,
          "id": "vast_seat1-bid1-from-msan",
          "adm": "&lt;VAST version=\"2.0\">&lt;Ad id=\"preroll-1\">&lt;InLine>&lt;AdSystem>2.0&lt;/AdSystem>&lt;AdTitle>5773100&lt;/AdTitle>&lt;Creatives>&lt;Creative>&lt;Linear>&lt;Duration>00:00:15&lt;/Duration>&lt;MediaFiles>&lt;MediaFile height=\"360\" width=\"640\" type=\"video/mp4\" delivery=\"streaming\">http://ndtrs.net/videos/video.mp4&lt;/MediaFile></MediaFiles>&lt;/Linear></Creative>&lt;Creative>&lt;CompanionAds>&lt;Companion height=\"250\" width=\"300\" id=\"573242\">&lt;HTMLResource>&lt;![CDATA[&lt;A href=\"http://ndtrs.net/beacon\" target=\"_blank\">&lt;IMG SRC=\"http://ndtrs.net/images/image.png\" ALT=\"Click Here\" />&lt;/A>&lt;img src=\"http://ndtrs.net/images/image.png\" height=\"1\" width=\"1\" />]]>&lt;/HTMLResource>&lt;/Companion>&lt;/CompanionAds>&lt;/Creative>&lt;/Creatives>&lt;/InLine>&lt;/Ad>&lt;/VAST>",
          "adomain": [
            "example.com"
          ],
          "crid": "16969426",
          "cid": "2445",
          "iurl": "http://ndtrs.net/images/image.png",
          "burl":"http://us-east-user.demo.net/bill_notice/smaato_bid?rid=2G2J5QmI7JIKuEoeOyv-Fob7-MI19Iwm6q7tBiURHo8KCBFQ1tLBZTZHYH66VyIYN5amxUtCifKIY_l6IzDfJmiku4ukesiP5GdMtQ-0WRWnbtImXV7MWGTRUV23mQhIN9n8lAe_GfyhvwBOYX3ZcQEG8rlKkdBMM9iWRmAW25QjbJ6p6uS6KpTkujKcXqDXeDn76blEVDMdsOtyAd8871FPeN8MHxWK4bSKHy8uwN5P_LEMJL87i3yBG4UaZnxm--KUTpdLGo_ZqNmIQclot1VbuTcWZu9Z_wlDpq62FmP6vKHOs5o73onK9xlhP8IWiM_jEZGozvP7EHQiA0wpZ9jDKyEwUkLGkj_XyMkGqTjhjyeskjEbP5RAFojdXNp4EfuiVt6q-ToRHZ_Pjsq3HOurWfmZl2TVig7JAOVpYBt-sV8Nwiw67oxN3R1rQfhsHvhMBcFapqiAu0sOlNmMyJjwCAThPYX2eii3xy9MUe6KXUJSAcxj6xPFHM-TIm9KHKhFrilqpk6E2-8GLBIGoVMbKojd74bf9i5qIG14HJEb99zoHxG5rh_nXJ4DytgftwphiiKRmqSkpEIxsCjj0mGfeOXVWDBNpHF2GJ738XF9DTSE33IUkfYD7Ug&amp;p=${AUCTION_PRICE}&amp;aid=",
          "lurl":"http://us-east-user.demo.net/bidloss?lossreason=${AUCTION_LOSS}",
          "nurl": "http://ndtrs.net/winnotice/win?case=rtbVastResponse"
        }
      ]
    }
  ]
}"#;

pub const DISPLAY: &str = r#"{
  "seatbid":[{
    "seat": "1234",
    "bid":[{
      "nurl":"http:\/\/reports.demo.com\/fb?b=JdZQFdbCARgKMURHWGhvUVl0bSMBJeAhAA&amp;c=MTo6&amp;wp=${AUCTION_PRICE}",
      "burl":"http:\/\/reports.ubimo.com\/burl?K=JdZQFdbCARgKMURHWGhvUVl0bSMBJeAhAA&amp;c=MTo6&amp;wp=${AUCTION_PRICE}",
      "lurl":"http:\/\/reports.ubimo.com\/bidloss?lossreason=${AUCTION_LOSS}",
      "adomain":["example.com"],
      "price":5.5,
      "id":"1FGYhoQYtm",
      "adm": "&lt;ad xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"smaato_ad_v0.9.xsd\" modelVersion=\"0.9\">&lt;imageAd>&lt;imgUrl>https://simage.png&lt;/imgUrl>&lt;clickUrl>http://click&lt;/clickUrl>&lt;width>320&lt;/width>&lt;height>50&lt;/height>&lt;/imageAd>&lt;/ad>",
      "impid":"1",
      "cid":"1234",
      "crid":"1234"
    }]
  }],
  "id":"1FGYhoQYtm"
}
"#;

pub const NATIVE_1_1: &str = r#"{
  "id": "general-dsp",
  "bidid": "general-dps-bid-id",
  "seatbid": [
    {
      "seat": "general-dsp-seat1",
      "bid": [
        {
          "impid": "1",
          "price": 1,
          "id": "general-dsp-native-ad",
          "adm": "{\"native\" :{\"link\":{\"url\":\"http://i.am.a/URL\"},\"assets\":[{\"id\":$TITLE_IDS,\"required\":1,\"title\":{\"text\":\"Title\"}},{\"id\":$IMAGE_IDS,\"required\":1,\"img\":{\"url\":\"https://creatives.smaato.com/creatives/native/1456230556037.png\"}},{\"id\":$IMAGE_IDS,\"required\":1,\"img\":{\"url\":\"https://creatives.smaato.com/creatives/native/1456230556037.png\"}},{\"id\":$DATA_IDS,\"required\":1,\"data\":{\"value\":\"Description\"}},{\"id\":$DATA_IDS,\"required\":1,\"data\":{\"value\":\"CTAText.\"}}],\"imptrackers\":[\"http://tracker.net/tracking/beacon1\",\"http://tracker.net/tracking/beacon2\",\"http://tracker.net/tracking/beacon3\"]}}",
          "adomain": [
            "example.com"
          ],
          "crid": "native-crid",
          "cid": "native-cid",
          "iurl": "http://us-east-user.demo.net/image.png",
          "burl": "http://us-east-user.demo.net/bill_notice/smaato_bid?rid=2G2J5QmI7JIKuEoeOyv-Fob7-MI19Iwm6q7tBiURHo8KCBFQ1tLBZTZHYH66VyIYN5amxUtCifKIY_l6IzDfJmiku4ukesiP5GdMtQ-0WRWnbtImXV7MWGTRUV23mQhIN9n8lAe_GfyhvwBOYX3ZcQEG8rlKkdBMM9iWRmAW25QjbJ6p6uS6KpTkujKcXqDXeDn76blEVDMdsOtyAd8871FPeN8MHxWK4bSKHy8uwN5P_LEMJL87i3yBG4UaZnxm--KUTpdLGo_ZqNmIQclot1VbuTcWZu9Z_wlDpq62FmP6vKHOs5o73onK9xlhP8IWiM_jEZGozvP7EHQiA0wpZ9jDKyEwUkLGkj_XyMkGqTjhjyeskjEbP5RAFojdXNp4EfuiVt6q-ToRHZ_Pjsq3HOurWfmZl2TVig7JAOVpYBt-sV8Nwiw67oxN3R1rQfhsHvhMBcFapqiAu0sOlNmMyJjwCAThPYX2eii3xy9MUe6KXUJSAcxj6xPFHM-TIm9KHKhFrilqpk6E2-8GLBIGoVMbKojd74bf9i5qIG14HJEb99zoHxG5rh_nXJ4DytgftwphiiKRmqSkpEIxsCjj0mGfeOXVWDBNpHF2GJ738XF9DTSE33IUkfYD7Ug&p=${AUCTION_PRICE}&aid=",
          "lurl": "http://us-east-user.demo.net/bidloss?lossreason=${AUCTION_LOSS}",
          "nurl": "http://us-east-user.demo.net/win_notice/smaato_bid?rid=2G2J5QmI7JIKuEoeOyv-Fob7-MI19Iwm6q7tBiURHo8KCBFQ1tLBZTZHYH66VyIYN5amxUtCifKIY_l6IzDfJmiku4ukesiP5GdMtQ-0WRWnbtImXV7MWGTRUV23mQhIN9n8lAe_GfyhvwBOYX3ZcQEG8rlKkdBMM9iWRmAW25QjbJ6p6uS6KpTkujKcXqDXeDn76blEVDMdsOtyAd8871FPeN8MHxWK4bSKHy8uwN5P_LEMJL87i3yBG4UaZnxm--KUTpdLGo_ZqNmIQclot1VbuTcWZu9Z_wlDpq62FmP6vKHOs5o73onK9xlhP8IWiM_jEZGozvP7EHQiA0wpZ9jDKyEwUkLGkj_XyMkGqTjhjyeskjEbP5RAFojdXNp4EfuiVt6q-ToRHZ_Pjsq3HOurWfmZl2TVig7JAOVpYBt-sV8Nwiw67oxN3R1rQfhsHvhMBcFapqiAu0sOlNmMyJjwCAThPYX2eii3xy9MUe6KXUJSAcxj6xPFHM-TIm9KHKhFrilqpk6E2-8GLBIGoVMbKojd74bf9i5qIG14HJEb99zoHxG5rh_nXJ4DytgftwphiiKRmqSkpEIxsCjj0mGfeOXVWDBNpHF2GJ738XF9DTSE33IUkfYD7Ug&p=${AUCTION_PRICE}&aid="
        }
      ]
    }
  ]
}"#;

pub const NATIVE_1_2: &str = r#"{
  "id":"5a514203-b7a9-48d0-ad2e-4da05bf0c735",
  "bidid":"5d7801c2be564c8100f00011",
  "seatbid":[
     {
        "bid":[
           {
              "id":"5d7801c2be564c8100f00011-1062015",
              "price":2.0,
              "impid":"1",
              "nurl":"http://node-p2e-gie4ah.sitescout.com/smaato/win/aid:5d7801c2be564c8100f00011;c:${AUCTION_PRICE};s:;cid:1062015;ts:1568145858061;d:MzMzMTUwMi1uYXRpdmU",
              "adm":"{\"native\":{\"link\":{\"url\":\"http://clickserv.sitescout.com/clk/8cc00d7733a331ba/88e42568017fb609/0-0/0///~_aid_~5d7801c2be564c8100f00011//cidentLy8vLy8\",\"clicktrackers\":[\"https://adrta.com/c?clid=ss&paid=ss&cb=1568145858061&avid=112053&caid=1062015&publisherId=1100043642&kv5=&plid=3331502&segment=&kv4=100.10.38.5&kv14=&kv1=native&siteId=120207704&kv7=17&kv15=UNKNOWN&kv16=&kv17=&kv18=com.particlenews.newsbreak&kv24=MOBILE_APP&kv20=&kv21=eb6fe99843ff5310bd0cd40d267826fff05d8aa3&kv22=9863a35c-c598-4c80-b091-d7adf379bb26&kv11=5d7801c2be564c8100f00011&kv3=&kv2=\"]},\"ver\":\"1.2\",\"assets\":[{\"id\":5,\"required\":1,\"data\":{\"type\":2,\"value\":\"Lose 3 to 5 pounds per week. Keep it off with free support for life!\"}},{\"id\":2,\"required\":1,\"img\":{\"type\":3,\"url\":\"http://cdn01.basis.net/112100/112053/uhSvl5gEElOXwX0T.jpg\",\"w\":1200,\"h\":627}},{\"id\":1,\"required\":1,\"title\":{\"text\":\"Lose Weight For Life!\"}},{\"id\":6,\"required\":1,\"data\":{\"type\":12,\"value\":\"Learn more\"}},{\"id\":3,\"required\":1,\"img\":{\"type\":1,\"url\":\"http://cdn01.basis.net/112100/112053/5MkC88n4Mc8MdPa4.jpg\",\"w\":240,\"h\":240}}],\"imptrackers\":[\"https://node-p2e-gie4ah.sitescout.com/smaato/px/aid:5d7801c2be564c8100f00011;c:${AUCTION_PRICE};s:;cid:1062015;ts:1568145858061\",\"https://pixel-sync.sitescout.com/dmp/pixelSync?nid=17&wb=1&hdid=0&id=9863a35c-c598-4c80-b091-d7adf379bb26&idType=4\",\"https://ad.doubleclick.net/ddm/trackimp/N428001.3027484BASISDSP/B22322261.248240382;dc_trk_aid=447774364;dc_trk_cid=119057854;u=210363%7C5d7801c2be564c8100f00011%7C%7Ccom.particlenews.newsbreak%7CAWAKEN002CP+%5BCPA%5D+GT+%28Zips%2B15+mi%29_Display%7CMOBILE_APP%7C1062015;ord=[timestamp];dc_lat=;dc_rdid=;tag_for_child_directed_treatment=;tfua=?\"],\"jstracker\":\"\"}}",
              "adomain":[
                 "awaken180weightloss.com"
              ],
              "iurl":"http://preview.sitescout.ad/preview?adOnly=1&amp;ad=8cc00d7733a331ba",
              "crid":"3331502",
              "cid":"1062015",
              "attr":[

              ],
              "cat":[
                 "IAB7-44",
                 "IAB7"
              ]
           }
        ],
        "seat":"18878"
     }
  ],
  "cur":"USD"
}"#;

pub const PREFERRED_DEAL: &str = r#"{
  "id": "openDealTwoBids",
  "seatbid": [
    {
      "bid": [
        {
          "id": "firstBidWithDealId",
          "impid": "1",
          "price": 10.0,
          "burl":"http://us-east-user.demo.net/bill_notice/smaato_bid?rid=2G2J5QmI7JIKuEoeOyv-Fob7-MI19Iwm6q7tBiURHo8KCBFQ1tLBZTZHYH66VyIYN5amxUtCifKIY_l6IzDfJmiku4ukesiP5GdMtQ-0WRWnbtImXV7MWGTRUV23mQhIN9n8lAe_GfyhvwBOYX3ZcQEG8rlKkdBMM9iWRmAW25QjbJ6p6uS6KpTkujKcXqDXeDn76blEVDMdsOtyAd8871FPeN8MHxWK4bSKHy8uwN5P_LEMJL87i3yBG4UaZnxm--KUTpdLGo_ZqNmIQclot1VbuTcWZu9Z_wlDpq62FmP6vKHOs5o73onK9xlhP8IWiM_jEZGozvP7EHQiA0wpZ9jDKyEwUkLGkj_XyMkGqTjhjyeskjEbP5RAFojdXNp4EfuiVt6q-ToRHZ_Pjsq3HOurWfmZl2TVig7JAOVpYBt-sV8Nwiw67oxN3R1rQfhsHvhMBcFapqiAu0sOlNmMyJjwCAThPYX2eii3xy9MUe6KXUJSAcxj6xPFHM-TIm9KHKhFrilqpk6E2-8GLBIGoVMbKojd74bf9i5qIG14HJEb99zoHxG5rh_nXJ4DytgftwphiiKRmqSkpEIxsCjj0mGfeOXVWDBNpHF2GJ738XF9DTSE33IUkfYD7Ug&amp;p=${AUCTION_PRICE}&amp;aid=",
          "lurl":"http://us-east-user.demo.net/bidloss?lossreason=${AUCTION_LOSS}",
          "nurl": "http://ajkfkjd.net/winnotice/win",
          "adm": "&lt;ad xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"smaato_ad_v0.9.xsd\" modelVersion=\"0.9\">&lt;imageAd>&lt;imgUrl>http://ajkfkjd.net/images/image.png&lt;/imgUrl>&lt;clickUrl>http://ajkfkjd.net/click&lt;/clickUrl>&lt;width>320&lt;/width>&lt;height>50&lt;/height>&lt;/imageAd>&lt;/ad>",
          "adomain": [
            "example.com"
          ],
          "iurl": "http://ajkfkjd.net/images/image.png",
          "cid": "11111111111",
          "crid": "222222222",
          "dealid": "deal-id-with-bid"
        }
      ]
    },
    {
      "bid": [
        {
          "id": "secondBidWithoutDealId",
          "impid": "1",
          "price": 10.0,
          "burl":"http://us-east-user.demo.net/bill_notice/smaato_bid?rid=2G2J5QmI7JIKuEoeOyv-Fob7-MI19Iwm6q7tBiURHo8KCBFQ1tLBZTZHYH66VyIYN5amxUtCifKIY_l6IzDfJmiku4ukesiP5GdMtQ-0WRWnbtImXV7MWGTRUV23mQhIN9n8lAe_GfyhvwBOYX3ZcQEG8rlKkdBMM9iWRmAW25QjbJ6p6uS6KpTkujKcXqDXeDn76blEVDMdsOtyAd8871FPeN8MHxWK4bSKHy8uwN5P_LEMJL87i3yBG4UaZnxm--KUTpdLGo_ZqNmIQclot1VbuTcWZu9Z_wlDpq62FmP6vKHOs5o73onK9xlhP8IWiM_jEZGozvP7EHQiA0wpZ9jDKyEwUkLGkj_XyMkGqTjhjyeskjEbP5RAFojdXNp4EfuiVt6q-ToRHZ_Pjsq3HOurWfmZl2TVig7JAOVpYBt-sV8Nwiw67oxN3R1rQfhsHvhMBcFapqiAu0sOlNmMyJjwCAThPYX2eii3xy9MUe6KXUJSAcxj6xPFHM-TIm9KHKhFrilqpk6E2-8GLBIGoVMbKojd74bf9i5qIG14HJEb99zoHxG5rh_nXJ4DytgftwphiiKRmqSkpEIxsCjj0mGfeOXVWDBNpHF2GJ738XF9DTSE33IUkfYD7Ug&amp;p=${AUCTION_PRICE}&amp;aid=",
          "lurl":"http://us-east-user.demo.net/bidloss?lossreason=${AUCTION_LOSS}",
          "nurl": "http://ajkfkjd.net/winnotice/win",
          "adm": "&lt;ad xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"smaato_ad_v0.9.xsd\" modelVersion=\"0.9\">&lt;imageAd>&lt;imgUrl>http://ajkfkjd.net/images/image.png&lt;/imgUrl>&lt;clickUrl>http://ajkfkjd.net/click&lt;/clickUrl>&lt;width>320&lt;/width>&lt;height>50&lt;/height>&lt;/imageAd>&lt;/ad>",
          "adomain": [
            "example.com"
          ],
          "iurl": "http://ajkfkjd.net/images/image.png",
          "cid": "11111111111",
          "crid": "222222222"
        }
      ]
    }
  ]
}"#;

pub const PRIVATE_EXCHANGE: &str = r#"{
  "id": "dspForPX",
  "seatbid": [
    {
      "bid": [
        {
          "id": "dspForPX",
          "impid": "1",
          "price": 99.0,
          "adid": "534365",
          "burl":"http://us-east-user.demo.net/bill_notice/smaato_bid?rid=2G2J5QmI7JIKuEoeOyv-Fob7-MI19Iwm6q7tBiURHo8KCBFQ1tLBZTZHYH66VyIYN5amxUtCifKIY_l6IzDfJmiku4ukesiP5GdMtQ-0WRWnbtImXV7MWGTRUV23mQhIN9n8lAe_GfyhvwBOYX3ZcQEG8rlKkdBMM9iWRmAW25QjbJ6p6uS6KpTkujKcXqDXeDn76blEVDMdsOtyAd8871FPeN8MHxWK4bSKHy8uwN5P_LEMJL87i3yBG4UaZnxm--KUTpdLGo_ZqNmIQclot1VbuTcWZu9Z_wlDpq62FmP6vKHOs5o73onK9xlhP8IWiM_jEZGozvP7EHQiA0wpZ9jDKyEwUkLGkj_XyMkGqTjhjyeskjEbP5RAFojdXNp4EfuiVt6q-ToRHZ_Pjsq3HOurWfmZl2TVig7JAOVpYBt-sV8Nwiw67oxN3R1rQfhsHvhMBcFapqiAu0sOlNmMyJjwCAThPYX2eii3xy9MUe6KXUJSAcxj6xPFHM-TIm9KHKhFrilqpk6E2-8GLBIGoVMbKojd74bf9i5qIG14HJEb99zoHxG5rh_nXJ4DytgftwphiiKRmqSkpEIxsCjj0mGfeOXVWDBNpHF2GJ738XF9DTSE33IUkfYD7Ug&amp;p=${AUCTION_PRICE}&amp;aid=",
          "lurl":"http://us-east-user.demo.net/bidloss?lossreason=${AUCTION_LOSS}",
          "nurl": "http://ajkfkjd.net/winnotice/win?dspForPX",
          "adm": "&lt;ad xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"smaato_ad_v0.9.xsd\" modelVersion=\"0.9\">&lt;imageAd>&lt;imgUrl>http://ajkfkjd.net/images/image.png&lt;/imgUrl>&lt;clickUrl>http://ajkfkjd.net/click&lt;/clickUrl>&lt;width>320&lt;/width>&lt;height>50&lt;/height>&lt;/imageAd>&lt;/ad>",
          "adomain": [
            "example.com"
          ],
          "iurl": "http://ajkfkjd.net/images/image.png?dspForPX",
          "cid": "435435435",
          "crid": "435435",
          "attr": [
          ],
          "dealid": "83fafb25-09d2-4ace-812a-70c260cdf70c"
        }
      ],
      "seat": "seat1",
      "group": 0
    }
  ],
  "bidid": "dspForPX_bidid",
  "cur": "USD"
}"#;

pub const RICH_MEDIA: &str = r#"{
  "cur":"USD",
  "seatbid":[{
    "seat":"16",
    "bid":[{
      "price":1.1,
      "id":"16-2e8e019a-2451-4f00-a84e-0e4c4c36c0e6-1",
      "impid":"1",
      "nurl":"http://us-east-user.demo.net/win_notice/smaato_bid?rid=2G2J5QmI7JIKuEoeOyv-Fob7-MI19Iwm6q7tBiURHo8KCBFQ1tLBZTZHYH66VyIYN5amxUtCifKIY_l6IzDfJmiku4ukesiP5GdMtQ-0WRWnbtImXV7MWGTRUV23mQhIN9n8lAe_GfyhvwBOYX3ZcQEG8rlKkdBMM9iWRmAW25QjbJ6p6uS6KpTkujKcXqDXeDn76blEVDMdsOtyAd8871FPeN8MHxWK4bSKHy8uwN5P_LEMJL87i3yBG4UaZnxm--KUTpdLGo_ZqNmIQclot1VbuTcWZu9Z_wlDpq62FmP6vKHOs5o73onK9xlhP8IWiM_jEZGozvP7EHQiA0wpZ9jDKyEwUkLGkj_XyMkGqTjhjyeskjEbP5RAFojdXNp4EfuiVt6q-ToRHZ_Pjsq3HOurWfmZl2TVig7JAOVpYBt-sV8Nwiw67oxN3R1rQfhsHvhMBcFapqiAu0sOlNmMyJjwCAThPYX2eii3xy9MUe6KXUJSAcxj6xPFHM-TIm9KHKhFrilqpk6E2-8GLBIGoVMbKojd74bf9i5qIG14HJEb99zoHxG5rh_nXJ4DytgftwphiiKRmqSkpEIxsCjj0mGfeOXVWDBNpHF2GJ738XF9DTSE33IUkfYD7Ug&amp;p=${AUCTION_PRICE}&amp;aid=",
      "burl":"http://us-east-user.demo.net/bill_notice/smaato_bid?rid=2G2J5QmI7JIKuEoeOyv-Fob7-MI19Iwm6q7tBiURHo8KCBFQ1tLBZTZHYH66VyIYN5amxUtCifKIY_l6IzDfJmiku4ukesiP5GdMtQ-0WRWnbtImXV7MWGTRUV23mQhIN9n8lAe_GfyhvwBOYX3ZcQEG8rlKkdBMM9iWRmAW25QjbJ6p6uS6KpTkujKcXqDXeDn76blEVDMdsOtyAd8871FPeN8MHxWK4bSKHy8uwN5P_LEMJL87i3yBG4UaZnxm--KUTpdLGo_ZqNmIQclot1VbuTcWZu9Z_wlDpq62FmP6vKHOs5o73onK9xlhP8IWiM_jEZGozvP7EHQiA0wpZ9jDKyEwUkLGkj_XyMkGqTjhjyeskjEbP5RAFojdXNp4EfuiVt6q-ToRHZ_Pjsq3HOurWfmZl2TVig7JAOVpYBt-sV8Nwiw67oxN3R1rQfhsHvhMBcFapqiAu0sOlNmMyJjwCAThPYX2eii3xy9MUe6KXUJSAcxj6xPFHM-TIm9KHKhFrilqpk6E2-8GLBIGoVMbKojd74bf9i5qIG14HJEb99zoHxG5rh_nXJ4DytgftwphiiKRmqSkpEIxsCjj0mGfeOXVWDBNpHF2GJ738XF9DTSE33IUkfYD7Ug&amp;p=${AUCTION_PRICE}&amp;aid=",
      "lurl":"http://us-east-user.demo.net/bidloss?lossreason=${AUCTION_LOSS}",
      "cid":"3538598",
      "iurl":"http://bid.g.doubleclick.net/xbbe/creativefetch?p=APEucNUUi4brC8Kg06zUjH7LDKx7Ci82QFgAYmQ5siKRKVtLW7upJxs",
      "adomain":["example.com"],
      "adid":"16_12096914",
      "adm":"&lt;?xml version=\"1.0\" encoding=\"ISO-8859-1\"?> &lt;ad xmlns:xsi=\"http://www.w3.org/2001/XMLSchema-instance\" xsi:noNamespaceSchemaLocation=\"http://standards.smaato.com/ad/smaato_ad_v0.9.xsd\" modelVersion=\"0.9\"> &lt;richmediaAd> &lt;content> &lt;![CDATA[ &lt;SCRIPT language=\"JavaScript1.1\" SRC=\"http://bid.g.doubleclick.net/xbbe/creative/adj?d=APEucNXA2G-JzINdXHGxjGolvpNa3yygpqz7ciHL6FuVTgAVv45EgYEAdDrbQ08wOQXFhZugcx2U910Q20yPMBnULDJEyla5FL48O-rh9C4poXieh_iZYykTpAHl71KBPaYgYpdfzJvnLELjpf1sXmdmPIOeWZ-a5UxOJH9NLpuj54OVdgkOpeN48Uf9V-eAkikM1PcDJWBL4bPTle7qkohIFHwqUOJlxnQ_UbZ9ra1fmUObEVSJH6W3uJ3rqYOvd-cE1DXxOvWUEqfy-bsn-8gLPVEuLE01_xx2WRkAFqbhfk-UJCnJmLgrTFUioamTyBW40su1x2Ej4id3y-R-RWvmpHPbxRblWUwtXZ8M7uWF8-PNE4e_FPRj3zyGhlAWX-taN8CqOyV_\"> &lt;/SCRIPT> <NOSCRIPT> <A HREF=\"http://bid.g.doubleclick.net/xbbe/creative/jump?d=APEucNXA2G-JzINdXHGxjGolvpNa3yygpqz7ciHL6FuVTgAVv45EgYEAdDrbQ08wOQXFhZugcx2U910Q20yPMBnULDJEyla5FL48O-rh9C4poXieh_iZYykTpAHl71KBPaYgYpdfzJvnLELjpf1sXmdmPIOeWZ-a5UxOJH9NLpuj54OVdgkOpeN48Uf9V-eAkikM1PcDJWBL4bPTle7qkohIFHwqUOJlxnQ_UbZ9ra1fmUObEVSJH6W3uJ3rqYOvd-cE1DXxOvWUEqfy-bsn-8gLPVEuLE01_xx2WRkAFqbhfk-UJCnJmLgrTFUioamTyBW40su1x2Ej4id3y-R-RWvmpHPbxRblWUwtXZ8M7uWF8-PNE4e_FPRj3zyGhlAWX-taN8CqOyV_\"> <IMG SRC=\"http://bid.g.doubleclick.net/xbbe/creative/ad?d=APEucNXA2G-JzINdXHGxjGolvpNa3yygpqz7ciHL6FuVTgAVv45EgYEAdDrbQ08wOQXFhZugcx2U910Q20yPMBnULDJEyla5FL48O-rh9C4poXieh_iZYykTpAHl71KBPaYgYpdfzJvnLELjpf1sXmdmPIOeWZ-a5UxOJH9NLpuj54OVdgkOpeN48Uf9V-eAkikM1PcDJWBL4bPTle7qkohIFHwqUOJlxnQ_UbZ9ra1fmUObEVSJH6W3uJ3rqYOvd-cE1DXxOvWUEqfy-bsn-8gLPVEuLE01_xx2WRkAFqbhfk-UJCnJmLgrTFUioamTyBW40su1x2Ej4id3y-R-RWvmpHPbxRblWUwtXZ8M7uWF8-PNE4e_FPRj3zyGhlAWX-taN8CqOyV_\" BORDER=0 WIDTH=320 HEIGHT=50 ALT=\"Advertisement\"> </A> &lt;/NOSCRIPT> <script type=\"text/javascript\"> if ('') { document.write('<script src=\"http://c.betrad.com/surly.js?;ad_w=320;ad_h=50;coid=322;nid=4311;cps=\"></sc' + 'ript>'); } </script> &lt;iframe src=\"http://googleads.g.doubleclick.net/xbbe/pixel?d=CLD6GhDf6SMYkqviBQ&amp;v=APEucNVZLac_dGJ6w0zYTxCVWQAcQdpH4PuzoyT9C6HRzn7VaL-k0kVZIasKbtViVirKrS6mrgFNjly9fPhWmN4ssIUvKL9Knb7G5QDCVDHTIp1K8pXjpkk\" style=\"display:none\"> &lt;/iframe> <img width='1' height='1' style='border:0' src='http://bs.serving-sys.com/BurstingPipe/adServer.bs?cn=tf&c=19&mc=imp&pli=14500879&PluID=0&ord=%%CACHEBUSTER%%&rtu=-1'/>\n\n &lt;img width='1' height='1' style='border:0' src='http://ad.atdmt.com/i/img;p=11212200958561;idfa=;idfa_lat=;aaid=;aaid_lat=;cache='/> <DIV STYLE=\"position: absolute; left: 0px; top: 0px; visibility: hidden;\"> <IMG SRC=\"http://bid.g.doubleclick.net/xbbe/beacon?data=APEucNUS4EoEQm9Q3Ene-RM9D9f6DGvm1LE3P42bBjZZ1rOxT6a882v3yjhJwD-L10E0weODQwyVjNWIXyVHvZ9wPVBjPbmTOw\" BORDER=0 WIDTH=1 HEIGHT=1 ALT=\"\" STYLE=\"display:none\"> </DIV> &lt;img src=\"http://us-east-user.bidswitch.net/imp/${AUCTION_PRICE}/http%3A%2F%2Fbid.g.doubleclick.net%2Fxbbe%2Fview%3Fd%3DAPEucNU1IiZ7L6GpDGC7w-mLx5Epkf_VI1_4ziSsfbZNIL8xTyR2KNtMkPiBLEKqOKVAScR4lhH16FzEjI3cdvwJaPHCGq7xG9h695wqBLmiI8jB221fmVrC8X9OdbfblqACvT5vlbPltKUezCL2QZ-C6ZNUiG_zpkkb6iae7omUA5trMWnKPtPHsKxIHzJ55Fjtnt4FR2pGniq4EiBDjhkYFG1byRdCZxta_YcFhdPxGrnUso36KNdyZrRfBOLMdflTJv78ioUT%26pr%3D%24%7BAUCTION_PRICE%7D/2G2J5QmI7JIKuEoeOyv-Fob7-MI19Iwm6q7tBiURHo8KCBFQ1tLBZTZHYH66VyIYN5amxUtCifKIY_l6IzDfJmiku4ukesiP5GdMtQ-0WRWnbtImXV7MWGTRUV23mQhIN9n8lAe_GfyhvwBOYX3ZcQEG8rlKkdBMM9iWRmAW25QjbJ6p6uS6KpTkujKcXqDXeDn76blEVDMdsOtyAd8871FPeN8MHxWK4bSKHy8uwN5P_LEMJL87i3yBG4UaZnxm--KUTpdLGo_ZqNmIQclot1VbuTcWZu9Z_wlDpq62FmP6vKHOs5o73onK9xlhP8IWiM_jEZGozvP7EHQiA0wpZ9jDKyEwUkLGkj_XyMkGqTjhjyeskjEbP5RAFojdXNp4EfuiVt6q-ToRHZ_Pjsq3HOurWfmZl2TVig7JAOVpYBt-sV8Nwiw67oxN3R1rQfhsHvhMBcFapqiAu0sOlNmMyJjwCAThPYX2eii3xy9MUe6KXUJSAcxj6xPFHM-TIm9KHKhFrilqpk6E2-8GLBIGoVMbKojd74bf9i5qIG14HJEb99zoHxG5rh_nXJ4DytgftwphiiKRmqSkpEIxsCjj0mGfeOXVWDBNpHF2GJ738XF9DTSE33IUkfYD7Ug/\" alt=\" \" style=\"display:none\"/> ]]> &lt;/content> &lt;width>320&lt;/width> &lt;height>50&lt;/height> &lt;/richmediaAd> &lt;/ad>"
      }]
  }],
  "id":"BqzFJc1Ze7"
}"#;

pub const MISSING_BID: &str = r#"{
  "cur":"USD",
  "seatbid":[{
    "seat":"16"
  }],
  "id":"BqzFJc1Ze7"
}"#;
