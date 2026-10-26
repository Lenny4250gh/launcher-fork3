//! What the online panel takes from this PC for the profile page and the feed (see
//! `screens::online::Local`): the player's games by title ID, the names of trophies from the games'
//! own trophy files, and the avatars of the players on screen.
//!
//! Called every frame the page is open. It only looks at memory: trophy files are read by the
//! trophy service the home screen already uses (`Effect::LoadTrophies`, off the UI thread), and
//! avatars are fetched by `avatars`.

use std::collections::BTreeSet;

use anyps5_core::online::api::FeedEvent;
use anyps5_core::paths::GameId;

use anyps5_core::online::api::MediaKind;
use anyps5_core::online::media;

use crate::app::avatars;
use crate::app::overlay::OverlayCx;
use crate::screens::online::MediaThumb;
use crate::screens::online::{Avatar, Local, OwnedGame};

/// The size received screenshots are decoded to (shown at most 260 dp wide).
const MEDIA: u32 = 640;

/// The size games are decoded to for the profile page and the feed (shown at most 64 dp).
const ICON: u32 = 192;
/// The size avatars are decoded to (shown at most 72 dp).
const AVATAR: u32 = 192;

/// The screenshots in the messages the launcher holds: the ones on this PC become textures, the
/// others are downloaded once (one file of media is asked for once per run).
fn media_thumbs(cx: &mut OverlayCx<'_>, local: &mut Local) {
    let dir = crate::app::panels::lfg::media_dir(cx.model);
    if !cx.model.online.media.swept {
        cx.model.online.media.swept = true;
        media::sweep(&dir, anyps5_core::online::unix_now());
    }
    let mut wanted: Vec<(String, String)> = Vec::new();
    {
        let online = &cx.model.online;
        for conversation in online.social.conversations() {
            for message in online
                .social
                .messages(&conversation.id)
                .iter()
                .rev()
                .take(24)
            {
                if let Some(info) = &message.media
                    && info.kind == MediaKind::Image
                    && !message.deleted
                {
                    wanted.push((info.id.clone(), info.content_type.clone()));
                }
            }
        }
    }
    for (id, content_type) in wanted {
        let Some(path) = media::cache_file(&dir, &id, &content_type) else {
            continue;
        };
        let online = &mut cx.model.online;
        if online.media.failed.contains(&id) {
            continue;
        }
        if !online.media.ready.contains(&id) {
            if !online.media.asked.insert(id.clone()) {
                continue;
            }
            if path.exists() {
                online.media.ready.insert(id.clone());
            } else {
                let _ = std::fs::create_dir_all(&dir);
                online.fetch_media(&id, &path);
                continue;
            }
        }
        if let Some(loaded) = cx.textures.get(&path, MEDIA) {
            local.media.insert(
                id,
                MediaThumb {
                    texture: loaded.id,
                    aspect: loaded.aspect,
                },
            );
        }
    }
}

pub fn build(cx: &mut OverlayCx<'_>, cloud: Option<&str>) -> Local {
    let mut local = Local::default();
    let online = &cx.model.online;

    // The titles and the avatars on screen.
    let mut titles: BTreeSet<String> = BTreeSet::new();
    let mut hashes: BTreeSet<String> = BTreeSet::new();
    let mut trophy_titles: BTreeSet<String> = BTreeSet::new();
    for item in &online.feed.items {
        match &item.event {
            FeedEvent::Trophy { title_id, .. } => {
                titles.insert(title_id.clone());
                trophy_titles.insert(title_id.clone());
            }
            FeedEvent::FirstPlayed { title_id } => {
                titles.insert(title_id.clone());
            }
            _ => {}
        }
        if let Some(Some(hash)) = online.avatars.get(&item.account.id) {
            hashes.insert(hash.clone());
        }
    }
    // The people of the Friends screen: the player and the friends and requests it lists.
    let people = online
        .account
        .iter()
        .map(|a| a.id)
        .chain(online.friends.friends.iter().map(|f| f.account.id))
        .chain(online.friends.incoming.iter().map(|r| r.account.id))
        .chain(online.friends.outgoing.iter().map(|r| r.account.id));
    for id in people {
        if let Some(Some(hash)) = online.avatars.get(&id) {
            hashes.insert(hash.clone());
        }
    }
    for page in online.profiles.values().filter_map(|e| e.page.as_ref()) {
        titles.extend(page.profile.favourites.iter().flatten().cloned());
        titles.extend(
            page.profile
                .playtime
                .iter()
                .flatten()
                .map(|p| p.title_id.clone()),
        );
        hashes.extend(page.profile.avatar.clone());
    }

    // Games and the names of the trophies in the feed.
    let mut to_read: Vec<GameId> = Vec::new();
    for title in &titles {
        let Some(game) = cx.model.games.iter().find(|g| &g.title_id == title) else {
            continue;
        };
        let icon = game
            .icon
            .as_deref()
            .and_then(|path| cx.textures.get(path, ICON))
            .map(|loaded| loaded.id);
        local.games.insert(
            title.clone(),
            OwnedGame {
                name: game.name.clone(),
                icon,
            },
        );
        if !trophy_titles.contains(title) {
            continue;
        }
        match cx.model.trophies.get(&game.id) {
            Some(set) => {
                // A trophy that is hidden and not earned by the player stays unnamed: its name is
                // a spoiler.
                for trophy in set.trophies.iter().filter(|t| t.service_label == 0) {
                    if !trophy.hidden || trophy.is_unlocked() {
                        local
                            .trophy_names
                            .insert((title.clone(), trophy.number), trophy.name.clone());
                    }
                }
            }
            None => to_read.push(game.id.clone()),
        }
    }
    cx.model.read_trophies_of(&to_read);

    media_thumbs(cx, &mut local);

    // Avatars.
    let dir = cx.model.paths.network_dir().join("avatars");
    for hash in hashes {
        if let Some(ready) = avatars::get(cx.ctx, cloud, &dir, &hash)
            && let Some(loaded) = cx.textures.get(&ready.path, AVATAR)
        {
            local.avatars.insert(
                hash,
                Avatar {
                    texture: loaded.id,
                    banner: ready.banner,
                },
            );
        }
    }
    local
}
