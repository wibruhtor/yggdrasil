# TODO

## QOL updates

Nothing...

## ChatSettings

```rust
struct ChatSettings {
	user_id: uuid::Uuid,
	chat_type: String
}

struct ChatColorSettings {
	user_id: uuid::Uuid,
	nickname_color: u32,
	background_color: u32,
	text_color: u32,
	gradient_only_for_custom_nicknames: boolean
}

struct ChatCustomNickname {
	user_id: uuid::Uuid,
	nickname: String,
	start_color: u32,
	end_color: u32
}

struct ChatSizeSettings {
	user_id: uuid::Uuid,
	margin_top: f64,
	margin_left: f64,
	margin_bottom: f64,
	margin_right: f64,
	padding_top: f64,
	padding_left: f64,
	padding_bottom: f64,
	padding_right: f64,
	border_top_left_radius: f64,
	border_top_right_radius: f64,
	border_bottom_left_radius: f64,
	border_bottom_right_radius: f64,
	max_messages: i32
}

struct ChatHideSettings {
	user_id: uuid::Uuid,
	hide_message_pattern: String,
	hide_point_rewards: boolean,
	hide_links: boolean,
	link_replacement: String,
	ban_word_replacement: String,
	ban_word_filter_id: Option<uuid::Uuid>
}

struct ChatHiddenNickname {
	user_id: uuid::Uuid,
	nickname: String,
}

struct ChatFontSettings {
	user_id: uuid::Uuid,
	font_family: String,
	font_weight: i32,
	font_size: f64
}
```