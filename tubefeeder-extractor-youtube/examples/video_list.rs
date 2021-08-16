/*
 * Copyright 2021 Julian Schmidhuber <github@schmiddi.anonaddy.com>
 *
 * This file is part of Tubefeeder-extractor.
 *
 * Tubefeeder-extractor is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * Tubefeeder-extractor is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with Tubefeeder-extractor.  If not, see <https://www.gnu.org/licenses/>.
 */

extern crate tubefeeder_extractor_youtube as tf_yt;

use tf_core::{Generator, Pipeline, Video};
use tf_yt::YTSubscription;
use tf_yt::YTVideo;

const SUBSCRIPTION_IDS: &'static [&'static str] = &[
    "UCYO_jab_esuFRV4b17AJtAw",
    "UCKtix2xNNXdcEfEFnoOnvMw",
    "UCld68syR8Wi-GY_n4CaoJGA",
    "UCkK9UDm_ZNrq_rIXCz3xCGA",
    "UC0e3QhIYukixgh5VVpKHH9Q",
    "UCrv269YwJzuZL3dH5PCgxUw",
    "UC9-y-6csu5WGm29I7JiwpnA",
    "UClq42foiSgl7sSpLupnugGA",
    "UCdC0An4ZPNr_YiFiYoVbwaw",
    "UC_ViSsVg_3JUDyLS3E2Un5g",
    "UCVls1GmFKf6WlTraIb_IaJg",
    "UCJYVW4HVrMWpUvvsjPQc8-Q",
    "UCrUL8K81R4VBzm-KOYwrcxQ",
    "UC5dBZm6ztKizdUnN7Puz3QQ",
    "UCjhkuC_Pi85wGjnB0I1ydxw",
    "UCELShpTQ3-sXIjHNO--NHwg",
    "UCv1Kcz-CuGM6mxzL3B1_Eiw",
    "UCW5OrUZ4SeUYkUg1XqcjFYA",
    "UCR9Gcq0CMm6YgTzsDxAxjOQ",
    "UCYJ61XIK64sp6ZFFS8sctxw",
    "UCuCkxoKLYO_EQ2GeFtbM_bw",
    "UC32w6uX5qtmUtF4QQQ2PKaQ",
    "UChi5MyXJLQuPni3dM19Ar3g",
    "UCHSI8erNrN6hs3sUK6oONLA",
    "UCZ9x-z3iOnIbJxVpm1rsu2A",
    "UC-2YHgc363EdcusLIBbgxzg",
    "UCXuqSBlHAE6Xw-yeJA0Tunw",
    "UCd4XwUn2Lure2NHHjukoCwA",
    "UClcE-kVhqyiHCcjYwcpfj9w",
    "UCsT9vcsUaj6YvUg0F7ntjkg",
    "UCdHDE389WqZX-TP6wPN1Llg",
    "UCY1kMZp36IQSyNx_9h4mpCg",
    "UC7YOGHUfC1Tb6E4pudI9STA",
    "UChFur_NwVSbUozOcF_F2kMg",
    "UCoxcjq-8xIDTYp3uz647V5A",
    "UCj4zC1Hfj-uc90FUXzRamNw",
    "UCIIVvAp6DP3a2MmoIuIjvQA",
    "UCKzJFdi57J53Vr_BkTfN3uQ",
    "UCK8XIGR5kRidIw2fWqwyHRA",
    "UCYVU6rModlGxvJbszCclGGw",
    "UC6P52TA_Ks_-1hdhGegGV6w",
    "UCcZUc0Wbt4EPVktbB8FrugQ",
    "UCgSJ92_7N3_TOHvKxN2yV1w",
    "UCmtyQOKKmrMVaKuRXz02jbQ",
    "UCzWQYUVCpZqtN93H8RR44Qw",
    "UCCI6C8hD-hTZi2JEmS7zvQw",
    "UCKGMHVipEvuZudhHD05FOYA",
    "UC6107grRI4m0o2-emgoDnAA",
    "UCSju5G2aFaWMqn-_0YBtq5A",
    "UCEIwxahdLz7bap-VDs9h35A",
    "UCfa7jJFYnn3P5LdJXsFkrjw",
    "UCj1VqrHhDte54oLgPG4xpuQ",
    "UCoryWpk4QVYKFCJul9KBdyw",
    "UCAiiOTio8Yu69c3XnR7nQBQ",
    "UCs6KfncB4OV6Vug4o_bzijg",
    "UC1VLQPn9cYSqx8plbk9RxxQ",
    "UC3sznuotAs2ohg_U__Jzj_Q",
    "UCo_IB5145EVNcf8hw1Kku7w",
    "UCjr2bPAyPV7t35MvcgT3W8Q",
    "UC5UAwBUum7CPN5buc-_N1Fw",
    "UCUkRj4qoT1bsWpE_C8lZYoQ",
    "UC64UiPJwM_e9AqAd7RiD7JA",
    "UCBa659QWEk1AI4Tg--mrJ2A",
    "UCbfYPyITQ-7l4upoX8nvctg",
    "UCb-cM927p9tWkqmpOrBabOQ",
    "UCHnyfMqiRRG1u-2MsSQLbXA",
    "UC6nSFpj9HTCZ5t-N3Rm3-HA",
    "UCqmugCqELzhIMNYnsjScXXw",
    "UCwmFOfFuvRPI112vR5DNnrA",
    "UC3fCT-K6Ik05UBKIMuGTfsA",
    "UCsnGwSIHyoYN0kiINAGUKxg",
    "UCU9pX8hKcrx06XfOB-VQLdw",
];

#[tokio::main(flavor = "current_thread")]
pub async fn main() {
    env_logger::init();
    log::info!("Logging enabled");
    let pipeline = Pipeline::<YTSubscription, YTVideo>::new();
    let subscriptions = pipeline.subscription_list();

    SUBSCRIPTION_IDS
        .iter()
        .map(|id| YTSubscription::new(id))
        .for_each(|sub| subscriptions.lock().unwrap().add(sub));

    println!("VIDEOS: ");
    for video in pipeline.generate().await.0.take(100) {
        let video = video.lock().unwrap();
        let subscription = video.subscription();
        println!(
            "{}: {}",
            subscription.name().unwrap_or(subscription.id()),
            video.title()
        );
    }
}
