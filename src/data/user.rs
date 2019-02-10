/// Struct that represent's a user that could either be authorized or not
pub enum User {
	/// An authorized user
	Authed(AuthUserData),
	/// An other user
	Other(UserData),
}

/// Data structure that represents the user that is currently authorized
pub struct AuthUserData {
	/// Data that would be present even if the user wasn't present
	pub userdata: UserData,
}

/// Data structure that represents a user's info
pub struct UserData {
	/// Comment karma of the user
	pub comment_karma: i64,
	/// The time the user was created in seconds
	pub created: f64,
	/// I DON't KNoowW
	pub created_utc: f64,
	/// also don't know
	pub has_subscribed: bool,
	/// Whether the user has verified their email
	pub has_verified_email: bool,
	/// Don't know
	pub hide_from_robots: bool,
	/// The id of the user
	pub id: String,
	/// Whether the user is a Reddit employee
	pub is_employee: bool,
	/// Whether the user is friend of the current user
	pub is_friend: bool,
	/// Whether the user has Reddit gold or not
	pub is_gold: bool,
	/// Whether the user is a moderator
	pub is_mod: bool,
	/// Link karma of the user
	pub link_karma: i64,
	/// The user's username
	pub name: String,
}

/// Data structure that represents a user's preferences
pub struct UserPreferences {
	/// Display conversations within the messages section of your inbox
	pub threaded_messages: bool,
	/// Don't show me submissions after I've downvoted them (except my own)
	pub hide_downs: bool,
	/// Label posts that are not safe for work
	pub label_nsfw: bool,
	/// Allow Reddit to use your activity on Reddit to show you more relevant advertisements
	pub activity_relevant_ads: bool,
	/// Use custom themes for all communities. You can also turn this off on a per community basis.
	pub show_stylesheets: bool,
	/// Show profiles in legacy mode
	pub profile_opt_out: bool,
	/// Autoplay Reddit videos on the desktop comments page
	pub video_autoplay: bool,
	/// Allow personalization of content using data from third-party services
	pub third_party_site_data_personalized_content: bool,
	/// Show link flair
	pub show_link_flair: bool,
	/// Use a creddit to automatically renew my gold if it expires
	pub creddit_autorenew: bool,
	/// Show trending subreddits on the home feed (a list of popular and notable subreddits to check out)
	pub show_trending: bool,
	/// Enable private RSS feeds
	pub private_feeds: bool,
	/// Notify me when people say my username
	pub monitor_mentions: bool,
	/// Unknown
	pub public_server_seconds: bool,
	/// Allow my data to be used for research purposes
	pub research: bool,
	/// Ignore suggested sorts
	pub ignore_suggested_sort: bool,
	/// Send email digests
	pub email_digests: bool,
	/// Number of comments to display. [1, 500]
	pub num_comments: i32,
	/// Show me links I’ve recently viewed
	pub clickgadget: bool,
	/// Unknown
	pub use_global_defaults: bool,
	/// Unknown
	pub show_snoovatar: bool,
	/// Enable to view adult and NSFW (not safe for work) content in your feed and search results
	pub over_18: bool,
	/// Send messages as emails
	pub email_messages: bool,
	/// Send message notifications in my browser
	pub live_orangereds: bool,
	/// Unknown
	pub enable_default_themes: bool,
	/// Show legacy search page
	pub legacy_search: bool,
	/// Show additional details in the domain text when available (such as the source subreddit or the content author's url/name)
	pub domain_details: bool,
	/// Collapse the left sidebar in legacy mode
	pub collapse_left_bar: bool,
	/// Languge in IETF format, i.e. 'en-us'
	pub lang: String,
	/// Don't show me submissions after I've upvoted them (except my own)
	pub hide_ups: bool,
	/// Allow Reddit to use data provided by third-parties to show you more relevant advertisements on Reddit.
	pub third_party_data_personalized_ads: bool,
	/// Allow reddit to log my outbound clicks for personalization
	pub allow_clicktracking: bool,
	/// Don't allow search engines to index my user profile
	pub hide_from_robots: bool,
	/// Show link to connected twitter account on profile page
	pub show_twitter: bool,
	/// Compress the link display
	pub compress: bool,
	/// Unknown
	pub store_visits: bool,
	/// Enable threaded modmail display
	pub threaded_modmail: bool,
	/// Don't show submissions below this score
	pub min_link_score: i32,
	/// Media preview
	/// on: auto-expand media previews
	/// off: don't auto-expand media previews on comments pages
	/// subreddit: expand media previews based on that subreddit's media preferences
	pub media_preview: String,
	/// Enable night mode
	pub nightmode: bool,
	/// Show a dagger (†) on comments voted controversial
	pub highlight_controversial: bool,
	/// Personalize popular by geolocation, such as 'SE', 'CA', 'GLOBAL', etc. null if user has never changed it.
	pub geopopular: Option<String>,
	/// Allow personalization of advertisements using data from third-party services
	pub third_party_site_data_personalized_ads: bool,
	/// Unknown
	pub show_promote: Option<bool>,
	/// Don't show comments below this score
	pub min_comment_score: i32,
	/// Make my votes public
	pub public_votes: bool,
	/// Show the spotlight box on the home feed
	pub organic: bool,
	/// Collapse messages after I’ve read them
	pub collapse_read_messages: bool,
	/// Show user flair
	pub show_flair: bool,
	/// Mark messages as read when I open my inbox
	pub mark_messages_read: bool,
	/// Hide images for NSFW/18+ content (Don't show thumbnails or media previews for anything labeled NSFW)
	pub no_profanity: bool,
	/// Hide ads
	pub hide_ads: bool,
	/// Opt into beta tests
	pub beta: bool,
	/// Show which communities I am active in on my profile.
	pub top_karma_subreddits: bool,
	/// Open links in a new window
	pub newwindow: bool,
	/// Number of links (posts) to show per page
	pub numsites: i32,
	/// Media thumbnails
	/// on: show thumbnails next to links
	/// off: don't show thumbnails next to links
	/// subreddit: show thumbnails based on that subreddit's media preferences
	pub media: Option<String>,
	/// Show how much gold you have remaining on your userpage
	pub show_gold_expiration: bool,
	/// Highlight new comments
	pub highlight_new_comments: bool,
	/// Unsubscribe from all emails
	pub email_unsubscribe_all: bool,
	/// Default soring order. Valid are: top, confidence (best), old, qa, controversial, new
	pub default_comment_sort: String,
	/// Who may send messages to the user. Valid settings are 'whitelisted' and 'everyone'. May be null if the user has never set it explicity.
	pub accept_pms: Option<String>,
}
