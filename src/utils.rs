use std::collections::HashMap;

/// Parse robots.txt file into a hashmap resembling ({"useragent": ["/path1", "/path2"]}`)
pub fn parse_robots(txt: &str) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();

    // Track state
    let mut active_agents: Vec<&str> = Vec::new();
    let mut was_user = false; // True if the last line was a user agent

    // Remove comments
    let txt = txt.lines().filter_map(|x| {
        if !x.trim_start().starts_with('#') {
            if let Some((x, _)) = x.split_once('#') {
                Some(x)
            } else {
                Some(x)
            }
        } else {
            None
        }
    });

    for line in txt {
        if let Some((_, agent)) = line.trim().split_once("User-agent:") {
            if was_user == false {
                // Clear if we're in a new user-agent block
                active_agents.clear();
            }
            active_agents.push(agent.trim());
            was_user = true;
        } else if let Some((_, disallow)) = line.trim().split_once("Disallow:") {
            for a in &active_agents {
                // Add disallow entry to all active agents
                if let Some(entry) = map.get_mut(a) {
                    entry.push(disallow.trim());
                } else {
                    map.insert(a, vec![disallow.trim()]);
                }
            }
            was_user = false;
        }
    }
    map
}
