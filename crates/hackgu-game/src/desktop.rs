//! # ALTIMIT OS Desktop & Community BBS / Mailer System
//! Reconstructed directly from `ccEventDtbbsview`, `ccEventDtmailer`, and `BA_BBS_CTRL`..`BU_BBS_CTRL`.
//!
//! Simulates the iconic ALTIMIT computer OS interface outside the MMORPG:
//! incoming emails, reply chains with affection bonuses, community forum threads,
//! news articles, and login transitions (`xlogin_01.ccs`).

use serde::{Deserialize, Serialize};
use crate::savedata::SaveData;

/// An email message received from a party companion or CC Corp administration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MailMessage {
    pub mail_id: u32,
    pub sender: String,
    pub subject: String,
    pub body: String,
    pub timestamp_str: String,
    pub is_read: bool,
    pub reply_options: Vec<MailReplyOption>,
    pub selected_reply: Option<usize>,
}

/// An interactive email reply choice that modifies companion affection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MailReplyOption {
    pub text: String,
    pub affection_bonus: u32,
    pub follow_up_response: String,
}

/// A post within a community BBS discussion thread.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BbsPost {
    pub post_number: u32,
    pub author: String,
    pub timestamp: String,
    pub content: String,
    pub image_asset: Option<String>, // e.g. "data/pbbs/pic000.ccs"
}

/// Forum Board Categories (`BA_BBS_CTRL` .. `BU_BBS_CTRL`).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum BbsBoardCategory {
    /// Board A: CC Corp Official System Notices & Maintenance
    OfficialNews,
    /// Board B: The World General Community Discussion
    GeneralCommunity,
    /// Board C: Rumors, PK (Player Killer) reports, Tri-Edge sightings
    RumorsAndPkers,
    /// Board N: Newbie Questions & Beginner Help
    NewbieTips,
    /// Board R: Guild Recruitment & Player Organizations
    GuildRecruiting,
    /// Board U: Unconfirmed Reports & Hidden Area Keyword Hints
    HiddenWordLeads,
}

impl BbsBoardCategory {
    pub fn board_name(&self) -> &'static str {
        match self {
            BbsBoardCategory::OfficialNews => "CC Corp Official News",
            BbsBoardCategory::GeneralCommunity => "The World General Board",
            BbsBoardCategory::RumorsAndPkers => "Rumors & PK Warnings",
            BbsBoardCategory::NewbieTips => "Beginner Help Board",
            BbsBoardCategory::GuildRecruiting => "Guild Recruitment",
            BbsBoardCategory::HiddenWordLeads => "Unconfirmed Reports",
        }
    }
}

/// A full discussion thread on the forum.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BbsThread {
    pub thread_id: u32,
    pub category: BbsBoardCategory,
    pub title: String,
    pub original_poster: String,
    pub posts: Vec<BbsPost>,
    pub unlocks_keyword: Option<(String, String, String)>,
}

/// ALTIMIT OS Desktop Application State.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AltimitDesktop {
    pub unread_mail_count: usize,
    pub emails: Vec<MailMessage>,
    pub forum_threads: Vec<BbsThread>,
    pub active_wallpaper: String,
    pub active_bgm: String,
    pub logged_into_the_world: bool,
}

impl AltimitDesktop {
    /// Construct authentic Vol 1 Rebirth starter desktop environment.
    pub fn new_vol1_defaults() -> Self {
        let emails = vec![
            MailMessage {
                mail_id: 1,
                sender: "Atoli".into(),
                subject: "Thank you for today!".into(),
                body: "Haseo, thank you for helping me earlier in the city! If you're free, do you want to explore together again?".into(),
                timestamp_str: "2017/04/10 14:22".into(),
                is_read: false,
                reply_options: vec![
                    MailReplyOption {
                        text: "Sure, let's party up whenever you're ready.".into(),
                        affection_bonus: 30,
                        follow_up_response: "Yay! I'll be waiting at Mac Anu Central Dome!".into(),
                    },
                    MailReplyOption {
                        text: "I'm busy with my own goals right now.".into(),
                        affection_bonus: 5,
                        follow_up_response: "Oh, okay... Let me know if you ever have time!".into(),
                    },
                ],
                selected_reply: None,
            },
            MailMessage {
                mail_id: 2,
                sender: "Silabus".into(),
                subject: "Guild Master Canard".into(),
                body: "Hey Haseo! Now that you're registered with Canard, don't forget to check the Mercenary Union quest board!".into(),
                timestamp_str: "2017/04/10 15:05".into(),
                is_read: false,
                reply_options: vec![
                    MailReplyOption {
                        text: "Got it. I'll check it out.".into(),
                        affection_bonus: 20,
                        follow_up_response: "Great! Let's do our best for Canard!".into(),
                    },
                ],
                selected_reply: None,
            },
        ];

        let forum_threads = vec![
            BbsThread {
                thread_id: 101,
                category: BbsBoardCategory::RumorsAndPkers,
                title: "Warning: Terror of Death spotted".into(),
                original_poster: "Anonymous PK".into(),
                posts: vec![
                    BbsPost {
                        post_number: 1,
                        author: "Shadow99".into(),
                        timestamp: "2017/04/10 12:10".into(),
                        content: "Watch out in the Delta server ruins. A black-cloaked Adept Rogue with twin blades wiped out an entire PK party in seconds.".into(),
                        image_asset: Some("data/pbbs/pic000.ccs".into()),
                    },
                    BbsPost {
                        post_number: 2,
                        author: "Bordeaux".into(),
                        timestamp: "2017/04/10 12:15".into(),
                        content: "Hmph, just rumors. If he crosses my path, I'll delete him myself.".into(),
                        image_asset: None,
                    },
                ],
                unlocks_keyword: None,
            },
            BbsThread {
                thread_id: 102,
                category: BbsBoardCategory::HiddenWordLeads,
                title: "Strange light in Delta Cavern".into(),
                original_poster: "Explorer_Jim".into(),
                posts: vec![
                    BbsPost {
                        post_number: 1,
                        author: "Explorer_Jim".into(),
                        timestamp: "2017/04/10 13:40".into(),
                        content: "I entered the keywords 'Submissive' 'Wrath' 'Corpse' and saw a glowing seal at the bottom of the cavern.".into(),
                        image_asset: Some("data/pbbs/pic006.ccs".into()),
                    },
                ],
                unlocks_keyword: Some(("Submissive".into(), "Wrath".into(), "Corpse".into())),
            },
        ];

        Self {
            unread_mail_count: emails.len(),
            emails,
            forum_threads,
            active_wallpaper: "Altimit Default Blue".into(),
            active_bgm: "Desktop Calm".into(),
            logged_into_the_world: false,
        }
    }

    /// Read an email, marking it read and updating unread counter.
    pub fn read_email(&mut self, mail_id: u32) -> Option<&MailMessage> {
        if let Some(msg) = self.emails.iter_mut().find(|m| m.mail_id == mail_id) {
            if !msg.is_read {
                msg.is_read = true;
                self.unread_mail_count = self.unread_mail_count.saturating_sub(1);
            }
            Some(msg)
        } else {
            None
        }
    }

    /// Send a reply to an email, granting companion affection and returning response.
    pub fn reply_email(
        &mut self,
        mail_id: u32,
        reply_index: usize,
        save: &mut SaveData,
    ) -> Result<String, &'static str> {
        let msg = self.emails.iter_mut().find(|m| m.mail_id == mail_id)
            .ok_or("Email not found")?;

        if msg.selected_reply.is_some() {
            return Err("Already replied to this email");
        }

        let option = msg.reply_options.get(reply_index)
            .ok_or("Invalid reply option index")?;

        let affection_bonus = option.affection_bonus;
        let follow_up = option.follow_up_response.clone();
        msg.selected_reply = Some(reply_index);

        // Update companion affection in save data
        if let Some(member) = save.party_members.iter_mut().find(|m| m.name.eq_ignore_ascii_case(&msg.sender)) {
            member.affection = (member.affection + affection_bonus).min(1000);
        }

        Ok(follow_up)
    }

    /// Read a BBS thread and harvest any secret Area Words mentioned into player save.
    pub fn read_bbs_thread(&self, thread_id: u32, save: &mut SaveData) -> Option<&BbsThread> {
        let thread = self.forum_threads.iter().find(|t| t.thread_id == thread_id)?;

        if let Some((ref w1, ref w2, ref w3)) = thread.unlocks_keyword {
            save.unlocked_first_words.insert(w1.clone());
            save.unlocked_second_words.insert(w2.clone());
            save.unlocked_third_words.insert(w3.clone());
        }

        Some(thread)
    }

    /// Log in to The World MMORPG (`xlogin_01.ccs`).
    pub fn login_to_world(&mut self) {
        self.logged_into_the_world = true;
    }

    /// Log out back to ALTIMIT OS desktop.
    pub fn logout_to_desktop(&mut self) {
        self.logged_into_the_world = false;
    }
}
