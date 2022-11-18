use lambda_runtime::Error;
use super::publisher::PublishRecord;
use super::utils::{transform_match_result, transform_lobby_type, transform_game_mode};

pub struct WebhookPublisher {
    pub client: webhook::client::WebhookClient
}

impl WebhookPublisher {

    pub async fn publish(&self, publish_record: &PublishRecord) -> Result<(), Error> {
        self.client.send(|mut message| {
            message = message.content(&format!("https://stratz.com/matches/{}", publish_record.match_id));
            message = message.embed(|mut embed| {
                embed = embed.author(
                    &publish_record.guild_name,
                    Some(format!("https://stratz.com/guilds/{}", publish_record.guild_id)),
                    Some(format!("https://steamusercontent-a.akamaihd.net/ugc/{}/", publish_record.guild_logo))
                );
                embed = embed.title(&format!(
                    "{} - {} - {}",
                    transform_match_result(&publish_record.match_result),
                    transform_lobby_type(&publish_record.lobby_type),
                    transform_game_mode(&publish_record.game_mode)
                ));

                let mut radiant_field = String::new();
                for player_stats in publish_record.player_stats_radiant.iter() {
                    let line = format!("{} {} [{}/{}/{}]\n",
                        match_hero_emoji(player_stats.hero_id),
                        player_stats.name,
                        player_stats.kills,
                        player_stats.deaths,
                        player_stats.assists
                    );
                    radiant_field.push_str(&line);
                }

                let mut dire_field = String::new();
                for player_stats in publish_record.player_stats_dire.iter() {
                    let line = format!("{} {} [{}/{}/{}]\n",
                        match_hero_emoji(player_stats.hero_id),
                        player_stats.name,
                        player_stats.kills,
                        player_stats.deaths,
                        player_stats.assists
                    );
                    dire_field.push_str(&line);
                }

                if radiant_field.len() > 0 {
                    embed = embed.field("<:radiant:958274781919207505> Radiant", &radiant_field, true);
                }

                if dire_field.len() > 0 {
                    embed = embed.field("<:dire:958274694203719740> Dire", &dire_field, true);
                }

                embed = embed.field(":clock3: Duration", &publish_record.duration_field, false);
                embed = embed.footer("Powered by STRATZ", Some(String::from("https://cdn.discordapp.com/icons/268890221943324677/12b63c55a83a715ec569e91e40641db0.webp?size=96")));
                embed = embed.timestamp(&publish_record.end.to_rfc3339());

                return embed;
            });

            return message;
        }).await?;

        Ok(())
    }

}

fn match_hero_emoji(hero_id: i16) -> &'static str {
    match hero_id {
        1 => "<:antimage:958248644652458005>",
        2 => "<:axe:958248644547608586>",
        3 => "<:bane:958249951480123394>",
        4 => "<:bloodseeker:958248644585332796>",
        5 => "<:crystal_maiden:958248644606320680>",
        6 => "<:drow_ranger:958248644799238194>",
        7 => "<:earthshaker:958248644748922900>",
        8 => "<:juggernaut:958248644853760052>",
        9 => "<:mirana:958248645038325771>",
        10 => "<:morphling:958248645025759282>",
        11 => "<:shadow_fiend:958248645147385866>",
        12 => "<:phantom_lancer:958249951857610772>",
        13 => "<:puck:958248645013147648>",
        14 => "<:pudge:958248645088645160>",
        15 => "<:razor:958248645134794762>",
        16 => "<:sand_king:958248645113815080>",
        17 => "<:storm_spirit:958249951262031934>",
        18 => "<:sven:958249951467548682>",
        19 => "<:tiny:958249951681450035>",
        20 => "<:vengeful_spirit:958249951710826516>",
        21 => "<:windranger:958249951652106310>",
        22 => "<:zeus:958249951459168288>",
        23 => "<:kunkka:958248645059313694>",
        25 => "<:lina:958248645000560660>",
        26 => "<:lion:958248644971229194>",
        27 => "<:shadow_shaman:958248645193502771>",
        28 => "<:slardar:958248645214486578>",
        29 => "<:tidehunter:958249951228469269>",
        30 => "<:witch_doctor:958249951715004446>",
        31 => "<:lich:958248644992172032>",
        32 => "<:riki:958248645138980914>",
        33 => "<:enigma:958248644954456094>",
        34 => "<:tinker:958249951480127518>",
        35 => "<:sniper:958248645155762196>",
        36 => "<:necrophos:958248644698595379>",
        37 => "<:warlock:958249951740182569>",
        38 => "<:beastmaster:958248644581146644>",
        39 => "<:queen_of_pain:958248644736331829>",
        40 => "<:venomancer:958249951580815400>",
        41 => "<:faceless_void:958248644912484382>",
        42 => "<:wraith_king:958248645239664700>",
        43 => "<:death_prophet:958248644740517910>",
        44 => "<:phantom_assassin:958249951941500938>",
        45 => "<:pugna:958248644937662465>",
        46 => "<:templar_assassin:958249952050544691>",
        47 => "<:viper:958249951207497769>",
        48 => "<:luna:958249951966674995>",
        49 => "<:dragon_knight:958248644803436544>",
        50 => "<:dazzle:958248644476301324>",
        51 => "<:clockwerk:958248645210284032>",
        52 => "<:leshrac:958248644912504883>",
        53 => "<:natures_prophet:958248644560162888>",
        54 => "<:lifestealer:958248645084467240>",
        55 => "<:dark_seer:958248644644073502>",
        56 => "<:clinkz:958249951735980042>",
        57 => "<:omniknight:958248645080252426>",
        58 => "<:enchantress:958248644853764097>",
        59 => "<:huskar:958248644967022642>",
        60 => "<:night_stalker:958248645004767282>",
        61 => "<:broodmother:958248644702777364>",
        62 => "<:bounty_hunter:958248644627271690>",
        63 => "<:weaver:958249951429812266>",
        64 => "<:jakiro:958249951568220190>",
        65 => "<:batrider:958248644560191589>",
        66 => "<:chen:958248644644057149>",
        67 => "<:spectre:958248645235474473>",
        69 => "<:doom:958248644698591232>",
        68 => "<:ancient_apparition:958248644572762153>",
        70 => "<:ursa:958249951845027860>",
        71 => "<:spirit_breaker:958249951492730900>",
        72 => "<:gyrocopter:958249951983456276>",
        73 => "<:alchemist:958248644719558716>",
        74 => "<:invoker:958249951429800009>",
        75 => "<:silencer:958248645143199774>",
        76 => "<:outworld_destroyer:958249951702441994>",
        77 => "<:lycan:958249951958290432>",
        78 => "<:brewmaster:958249951840854026>",
        79 => "<:shadow_demon:958249951454982187>",
        80 => "<:lone_druid:958249951798886400>",
        81 => "<:chaos_knight:958249951840845894>",
        82 => "<:meepo:958249952218345482>",
        83 => "<:treant_protector:958249951626924073>",
        84 => "<:ogre_magi:958249952000233472>",
        85 => "<:undying:958249951987634176>",
        86 => "<:rubick:958249951895388192>",
        87 => "<:disruptor:958249952256086046>",
        88 => "<:nyx_assassin:958249952130240562>",
        89 => "<:naga_siren:958249952100904990>",
        90 => "<:keeper_of_the_light:958249952105095218>",
        91 => "<:io:958249952054759424>",
        92 => "<:visage:958249952113459321>",
        93 => "<:slark:958249952218325002>",
        94 => "<:medusa:958249952193155092>",
        95 => "<:troll_warlord:958249952201564210>",
        96 => "<:centaur_warrunner:958249952184782848>",
        97 => "<:magnus:958249952226738196>",
        98 => "<:timbersaw:958249952251904050>",
        99 => "<:bristleback:958251187243745280>",
        100 => "<:tusk:958251186950111253>",
        101 => "<:skywrath_mage:958251187260502036>",
        102 => "<:abaddon:958251187180806146>",
        103 => "<:elder_titan:958251187289878598>",
        104 => "<:legion_commander:958251187117908018>",
        105 => "<:techies:958251187222740992>",
        106 => "<:ember_spirit:958251187143065610>",
        107 => "<:earth_spirit:958251187172438046>",
        108 => "<:underlord:958251187369549844>",
        109 => "<:terrorblade:958251187382153226>",
        110 => "<:phoenix:958251187214381096>",
        111 => "<:oracle:958251187306627072>",
        112 => "<:winter_wyvern:958251187281489980>",
        113 => "<:arc_warden:958251187340197898>",
        114 => "<:monkey_king:958251187205992469>",
        119 => "<:dark_willow:958251187591868446>",
        120 => "<:pangolier:958251187470233631>",
        121 => "<:grimstroke:958251187709304862>",
        123 => "<:hoodwink:958251187856105532>",
        126 => "<:void_spirit:958251187772215386>",
        128 => "<:snapfire:958251188023873587>",
        129 => "<:mars:958251187696726016>",
        135 => "<:dawnbreaker:958251187608645633>",
        136 => "<:marci:958254609397334026>",
        137 => "<:primal_beast:958254609397342258>",
        _ => ":grey_question:",
    }
}
