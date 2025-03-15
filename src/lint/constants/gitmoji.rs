// Latest Gitmoji version: v3.15
// The unicode emojis were normalised using GNU sed:
// sed -i 's/\xEF\xB8\x8F//g' gitmoji.rs

use std::collections::HashSet;
use std::sync::LazyLock;

pub static UNICODE_EMOJIS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        "🎨", // :art:
        "⚡", // :zap: without variation selector
        "🔥", // :fire:
        "🐛", // :bug:
        "🚑", // :ambulance:
        "✨", // :sparkles:
        "📝", // :memo:
        "🚀", // :rocket:
        "💄", // :lipstick:
        "🎉", // :tada:
        "✅", // :white_check_mark:
        "🔒", // :lock:
        "🔐", // :closed_lock_with_key:
        "🔖", // :bookmark:
        "🚨", // :rotating_light:
        "🚧", // :construction:
        "💚", // :green_heart:
        "⬇",  // :arrow_down:
        "⬆",  // :arrow_up:
        "📌", // :pushpin:
        "👷", // :construction_worker:
        "📈", // :chart_with_upwards_trend:
        "♻",  // :recycle:
        "➕", // :heavy_plus_sign:
        "➖", // :heavy_minus_sign:
        "🔧", // :wrench:
        "🔨", // :hammer:
        "🌐", // :globe_with_meridians:
        "✏",  // :pencil2:
        "💩", // :poop:
        "⏪", // :rewind:
        "🔀", // :twisted_rightwards_arrows:
        "📦", // :package:
        "👽", // :alien:
        "🚚", // :truck:
        "📄", // :page_facing_up:
        "💥", // :boom:
        "🍱", // :bento:
        "♿", // :wheelchair:
        "💡", // :bulb:
        "🍻", // :beers:
        "💬", // :speech_balloon:
        "🗃",  // :card_file_box:
        "🔊", // :loud_sound:
        "🔇", // :mute:
        "👥", // :busts_in_silhouette:
        "🚸", // :children_crossing:
        "🏗",  // :building_construction:
        "📱", // :iphone:
        "🤡", // :clown_face:
        "🥚", // :egg:
        "🙈", // :see_no_evil:
        "📸", // :camera_flash:
        "⚗",  // :alembic:
        "🔍", // :mag:
        "🏷",  // :label:
        "🌱", // :seedling:
        "🚩", // :triangular_flag_on_post:
        "🥅", // :goal_net:
        "💫", // :dizzy:
        "🗑",  // :wastebasket:
        "🛂", // :passport_control:
        "🩹", // :adhesive_bandage:
        "🧐", // :monocle_face:
        "⚰",  // :coffin:
        "🧪", // :test_tube:
        "👔", // :necktie:
        "🩺", // :stethoscope:
        "🧱", // :bricks:
        "🧑‍💻", // :technologist:
        "💸", // :money_with_wings:
        "🧵", // :thread:
        "🦺", // :safety_vest:
        "✈",  // :airplane:
    ])
});

pub static STRING_EMOJIS: LazyLock<HashSet<&'static str>> = LazyLock::new(|| {
    HashSet::from([
        ":art:",
        ":zap:",
        ":fire:",
        ":bug:",
        ":ambulance:",
        ":sparkles:",
        ":memo:",
        ":rocket:",
        ":lipstick:",
        ":tada:",
        ":white_check_mark:",
        ":lock:",
        ":closed_lock_with_key:",
        ":bookmark:",
        ":rotating_light:",
        ":construction:",
        ":green_heart:",
        ":arrow_down:",
        ":arrow_up:",
        ":pushpin:",
        ":construction_worker:",
        ":chart_with_upwards_trend:",
        ":recycle:",
        ":heavy_plus_sign:",
        ":heavy_minus_sign:",
        ":wrench:",
        ":hammer:",
        ":globe_with_meridians:",
        ":pencil2:",
        ":poop:",
        ":rewind:",
        ":twisted_rightwards_arrows:",
        ":package:",
        ":alien:",
        ":truck:",
        ":page_facing_up:",
        ":boom:",
        ":bento:",
        ":wheelchair:",
        ":bulb:",
        ":beers:",
        ":speech_balloon:",
        ":card_file_box:",
        ":loud_sound:",
        ":mute:",
        ":busts_in_silhouette:",
        ":children_crossing:",
        ":building_construction:",
        ":iphone:",
        ":clown_face:",
        ":egg:",
        ":see_no_evil:",
        ":camera_flash:",
        ":alembic:",
        ":mag:",
        ":label:",
        ":seedling:",
        ":triangular_flag_on_post:",
        ":goal_net:",
        ":dizzy:",
        ":wastebasket:",
        ":passport_control:",
        ":adhesive_bandage:",
        ":monocle_face:",
        ":coffin:",
        ":test_tube:",
        ":necktie:",
        ":stethoscope:",
        ":bricks:",
        ":technologist:",
        ":money_with_wings:",
        ":thread:",
        ":safety_vest:",
        ":airplane:",
    ])
});
