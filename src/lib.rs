use {
	lazy_static::lazy_static,
	phf::phf_map,
	poise::serenity_prelude::{EmojiId, ReactionType},
	rand::Rng as _,
	std::collections::HashMap,
};

mod args;
pub use args::Args;

pub mod tracing;

mod config;
pub use config::Config;

pub mod discord;
pub use discord::{State, StateExt};

pub mod events;
pub use events::EventHandler;

pub type Error = color_eyre::Report;
pub type Result<T> = color_eyre::Result<T>;

pub static REACTIONS: phf::Map<&'static str, Emotion> = phf_map! {
	"cute" => Emotion::Flushed,
	"love" => Emotion::Flushed,
	"gay" => Emotion::Flushed,
	"hot" => Emotion::Flushed,
	"sex" => Emotion::Flushed,
	"cum" => Emotion::Flushed,
	"balls" => Emotion::Flushed,

	"throw" => Emotion::Sad,
	"fail" => Emotion::Sad,
	"slow" => Emotion::Sad,
	"bad" => Emotion::Sad,
	"sad" => Emotion::Sad,
	"pensive" => Emotion::Sad,

	"fast" => Emotion::Awe,
	"wikid" => Emotion::Awe,
	"record" => Emotion::Awe,
	"pb" => Emotion::Awe,

	"funny" => Emotion::Funny,
	"lmao" => Emotion::Funny,
	"kekw" => Emotion::Funny,

	"dumb" => Emotion::Stupid,
	"idiot" => Emotion::Stupid,
	"retard" => Emotion::Stupid,
	"stupid" => Emotion::Stupid,
	"julianio" => Emotion::Stupid,
	"juli" => Emotion::Stupid,
	"jak" => Emotion::Stupid,

	"sus" => Emotion::Sus,
	"among" => Emotion::Sus,
	"amogus" => Emotion::Sus,

	"pedo" => Emotion::Weird,
	"racist" => Emotion::Weird,
	"breed" => Emotion::Weird,
	"rail" => Emotion::Weird,
	"penis" => Emotion::Weird,
	"incest" => Emotion::Weird,
	"kys" => Emotion::Weird,
	"foreskin" => Emotion::Weird,
};

lazy_static! {
	static ref EMOJIS: HashMap<Emotion, Vec<ReactionType>> = {
		HashMap::from_iter([
			(Emotion::Flushed, vec![
				ReactionType::Unicode(String::from("😳")),
				ReactionType::Custom {
					animated: false,
					id: EmojiId(975446235735543849),
					name: Some(String::from("joeFlushed")),
				},
				ReactionType::Custom {
					animated: false,
					id: EmojiId(992891956990586951),
					name: Some(String::from("catblush")),
				},
			]),
			(Emotion::Sad, vec![
				ReactionType::Unicode(String::from("😔")),
				ReactionType::Custom {
					animated: false,
					id: EmojiId(975446358796410890),
					name: Some(String::from("joePensive")),
				},
			]),
			(Emotion::Awe, vec![
				ReactionType::Unicode(String::from("🥶")),
				ReactionType::Custom {
					animated: false,
					id: EmojiId(779333271011852309),
					name: Some(String::from("WIKID")),
				},
			]),
			(Emotion::Funny, vec![
				ReactionType::Unicode(String::from("😂")),
				ReactionType::Unicode(String::from("😹")),
				ReactionType::Custom {
					animated: false,
					id: EmojiId(710839595553783808),
					name: Some(String::from("KEKW")),
				},
				ReactionType::Custom {
					animated: false,
					id: EmojiId(975446312835248168),
					name: Some(String::from("JOMEGALUL")),
				},
			]),
			(Emotion::Stupid, vec![
				ReactionType::Unicode(String::from("😡")),
				ReactionType::Custom {
					animated: false,
					id: EmojiId(1089259796814905465),
					name: Some(String::from("Dentge")),
				},
			]),
			(Emotion::Sus, vec![
				ReactionType::Custom {
					animated: false,
					id: EmojiId(975450505394413579),
					name: Some(String::from("Susge")),
				},
				ReactionType::Custom {
					animated: false,
					id: EmojiId(947467755727241287),
					name: Some(String::from("schnosesus")),
				},
			]),
			(Emotion::Weird, vec![
				ReactionType::Custom {
					animated: false,
					id: EmojiId(1057761537516896338),
					name: Some(String::from("amonge")),
				},
				ReactionType::Custom {
					animated: false,
					id: EmojiId(1091121617876287518),
					name: Some(String::from("HUH")),
				},
			]),
			(Emotion::WhatsApp, vec![ReactionType::Custom {
				animated: false,
				id: EmojiId(998940776136450128),
				name: Some(String::from("whatsapp")),
			}]),
		])
	};
}

pub static PROMPTS: phf::Map<&'static str, &'static str> = phf_map! {
	"hi joebot" => "hi there",
	"hey joebot" => "hi there",

	"joebot do you love me" => "of course I love you",
};

pub static COPY_PASTAS: phf::Map<&'static str, &'static str> = phf_map! {
	"wad" => "LoB #7 M u g e n: ourfather is the WAD tech to gain speed , so if you just WAD and dont agin its still ourfather !!!!!!!",
	"admin" => "DONOR *M u g e n: i sadi 5 words and alpha already threathenin me...admin abuse",
	"glog" => "ADMIN M u g e n: GLOGGLOG9000 ON THE FUCKING PEIRFS DICK",
	"faggot" => "iBrahizy: please call me a faggot in every language you speak",
	"romania" => "*ambi: spain are romanians?",
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Emotion {
	Flushed,
	Sad,
	Awe,
	Funny,
	Stupid,
	Sus,
	Weird,
	WhatsApp,
}

impl Emotion {
	pub fn judge(message: &str) -> Option<Self> {
		let message = message.to_lowercase();
		let mut counters = HashMap::<Self, usize>::new();

		for word in message.split(' ') {
			if let Some(emotion) = REACTIONS.get(word) {
				counters
					.entry(*emotion)
					.and_modify(|count| *count += 1)
					.or_insert(1);
			}
		}

		counters
			.into_iter()
			.max_by_key(|&(_, count)| count)
			.map(|(emotion, _)| emotion)
	}
}

impl From<&Emotion> for Option<ReactionType> {
	fn from(emotion: &Emotion) -> Self {
		let emojis = EMOJIS.get(emotion)?;
		let n = rand::thread_rng().gen_range(0..emojis.len());
		emojis.get(n).cloned()
	}
}
