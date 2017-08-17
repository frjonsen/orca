use super::*;

fn init_reddit() -> Reddit {
	use std::env;
	
	let mut reddit = Reddit::new("OrcaLibTest", "v0.0.2", "/u/IntrepidPig/");
	
	let id = match env::var("REDDIT_APP_ID") {
		Ok(id) => id,
		Err(_) => panic!("REDDIT_APP_ID must be set")
	};
	
	let secret = match env::var("REDDIT_APP_SECRET") {
		Ok(secret) => secret,
		Err(_) => panic!("REDDIT_APP_SECRET must be set")
	};
	let username = match env::var("REDDIT_USERNAME") {
		Ok(username) => username,
		Err(_) => panic!("REDDIT_USERNAME must be set")
	};
	let password = match env::var("REDDIT_PASSWORD") {
		Ok(password) => password,
		Err(_) => panic!("REDDIT_PASSWORD must be set")
	};
	
	reddit.conn.auth = Some(reddit.authorize(username, password, net::auth::OauthApp::Script(id, secret)).unwrap());
	
	reddit
}

#[test]
fn get_posts() {
	init_reddit().get_posts("unixporn".to_string(), sub::Sort::Top(sub::SortTime::All)).unwrap();
}

#[test]
fn post_sort() {
	assert_eq!(sub::Sort::Top(sub::SortTime::All).param(), &[("sort", "top"), ("t", "all")])
}

#[test]
fn test_auth() {
	init_reddit().get_user().unwrap();
}

//#[test(submit)]
fn test_post() {
	println!("{}", init_reddit().submit_self("pigasusland".to_string(), "Test Post".to_string(), "The time is dank-o-clock".to_string(), true).unwrap());
}