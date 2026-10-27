//! Texts of the first start that have no home in the shared tables (docs/DESIGN.md 5.15): the look,
//! controller, online and last pages. English, Turkish and German; button labels and titles in
//! title case, sentences in sentence case.

use crate::ui::i18n::{Arg, Language, substitute};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum OnboardingText {
    /// The online step's title.
    OnlineTitle,
    /// The online step's sentence.
    OnlineSubtitle,
    /// Goes on without signing in or without a folder.
    NotNow,
    /// Under the buttons: where signing in waits later.
    OnlineLater,
    /// Under the dots: `%@` = this step, `%@` = how many.
    StepOf,
    /// The look step's title.
    LookTitle,
    /// The look step's sentence.
    LookSubtitle,
    /// Caption over the three accessibility switches.
    Accessibility,
    /// Switch: fades instead of slides (the Animations setting).
    ReduceMotion,
    /// The controller step's title.
    PadTitle,
    /// `%@` = the controller's name.
    PadConnected,
    /// Shown when the pad went away during the first start.
    PadLost,
    /// Under a confirm choice: `%@` = the other face button.
    PadGoesBack,
    /// The games card without a folder.
    NoFolderTitle,
    /// ... and what Not Now means.
    NoFolderDetail,
    /// Online facts: the privacy default.
    FactPrivateTitle,
    /// Under Private by Default.
    FactPrivateDetail,
    /// Under Appear Offline.
    FactOfflineDetail,
    /// Online facts: what is never uploaded.
    FactLocalTitle,
    /// Under Your Games Stay on This PC.
    FactLocalDetail,
    /// The last page's title.
    DoneTitle,
    /// The last page's sentence.
    DoneSubtitle,
    /// Last page: the Home button of the controller.
    DoneHomeTitle,
    /// Under Home Button.
    DoneHomeDetail,
    /// Last page, without a controller.
    DonePadTitle,
    /// Under Controllers.
    DonePadDetail,
    /// Last page: where everything is.
    DoneSettingsTitle,
    /// Under Settings.
    DoneSettingsDetail,
    /// Last page: with a folder.
    DoneGamesTitle,
    /// Under Your Games.
    DoneGamesDetail,
    /// Last page: without a folder.
    DoneNoFolderTitle,
    /// Under Add Your Games.
    DoneNoFolderDetail,
    /// Last page: the sign-in that follows Start.
    DoneNextSignIn,
    /// Last page: the account that follows Start.
    DoneNextCreate,
    /// Last page, after Sign In or Create Account.
    DoneSignInDetail,
}

impl OnboardingText {
    pub const ALL: [OnboardingText; 35] = [
        OnboardingText::OnlineTitle,
        OnboardingText::OnlineSubtitle,
        OnboardingText::NotNow,
        OnboardingText::OnlineLater,
        OnboardingText::StepOf,
        OnboardingText::LookTitle,
        OnboardingText::LookSubtitle,
        OnboardingText::Accessibility,
        OnboardingText::ReduceMotion,
        OnboardingText::PadTitle,
        OnboardingText::PadConnected,
        OnboardingText::PadLost,
        OnboardingText::PadGoesBack,
        OnboardingText::NoFolderTitle,
        OnboardingText::NoFolderDetail,
        OnboardingText::FactPrivateTitle,
        OnboardingText::FactPrivateDetail,
        OnboardingText::FactOfflineDetail,
        OnboardingText::FactLocalTitle,
        OnboardingText::FactLocalDetail,
        OnboardingText::DoneTitle,
        OnboardingText::DoneSubtitle,
        OnboardingText::DoneHomeTitle,
        OnboardingText::DoneHomeDetail,
        OnboardingText::DonePadTitle,
        OnboardingText::DonePadDetail,
        OnboardingText::DoneSettingsTitle,
        OnboardingText::DoneSettingsDetail,
        OnboardingText::DoneGamesTitle,
        OnboardingText::DoneGamesDetail,
        OnboardingText::DoneNoFolderTitle,
        OnboardingText::DoneNoFolderDetail,
        OnboardingText::DoneNextSignIn,
        OnboardingText::DoneNextCreate,
        OnboardingText::DoneSignInDetail,
    ];

    pub const fn english(self) -> &'static str {
        match self {
            OnboardingText::OnlineTitle => "Play Together",
            OnboardingText::OnlineSubtitle => {
                "Sign in to see your friends, join their parties and calls, and keep your trophies on your profile. Signing in opens your browser for a moment."
            }
            OnboardingText::NotNow => "Not Now",
            OnboardingText::OnlineLater => "You can sign in any time from the top bar.",
            OnboardingText::StepOf => "Step %@ of %@",
            OnboardingText::LookTitle => "Choose Your Look",
            OnboardingText::LookSubtitle => {
                "The background shows your choice. You can change everything here later in Settings."
            }
            OnboardingText::Accessibility => "Accessibility",
            OnboardingText::ReduceMotion => "Reduce Motion",
            OnboardingText::PadTitle => "Your Controller",
            OnboardingText::PadConnected => {
                "%@ is connected. Press one of the four buttons on the right and it lights up below."
            }
            OnboardingText::PadLost => {
                "The controller is not connected right now. Connect it to try the buttons. The choices below still count."
            }
            OnboardingText::PadGoesBack => "%@ goes back",
            OnboardingText::NoFolderTitle => "No folder chosen yet",
            OnboardingText::NoFolderDetail => "You can choose one later in Settings.",
            OnboardingText::FactPrivateTitle => "Private by Default",
            OnboardingText::FactPrivateDetail => {
                "Your profile, trophies and playtime are visible to friends only."
            }
            OnboardingText::FactOfflineDetail => {
                "Friends see when you are online. Appear Offline hides it from everyone, any time you choose."
            }
            OnboardingText::FactLocalTitle => "Your Game Files Stay on This PC",
            OnboardingText::FactLocalDetail => "Nothing from your games folder is uploaded.",
            OnboardingText::DoneTitle => "You're All Set",
            OnboardingText::DoneSubtitle => "A few things worth knowing before you start.",
            OnboardingText::DoneHomeTitle => "Home Button",
            OnboardingText::DoneHomeDetail => {
                "Press it on your controller to bring the launcher in front while a game runs."
            }
            OnboardingText::DonePadTitle => "Controllers",
            OnboardingText::DonePadDetail => {
                "Plug one in any time. Settings has the button layout and a live test."
            }
            OnboardingText::DoneSettingsTitle => "Settings",
            OnboardingText::DoneSettingsDetail => {
                "The gear in the top bar. Everything you chose here is in it."
            }
            OnboardingText::DoneGamesTitle => "Your Games",
            OnboardingText::DoneGamesDetail => "New games in your folder show up on their own.",
            OnboardingText::DoneNoFolderTitle => "Add Your Games",
            OnboardingText::DoneNoFolderDetail => {
                "Choose a folder in Settings under Library. Home shows how."
            }
            OnboardingText::DoneNextSignIn => "Next: Sign In",
            OnboardingText::DoneNextCreate => "Next: Create Account",
            OnboardingText::DoneSignInDetail => "Your browser opens when you press Start.",
        }
    }

    pub const fn turkish(self) -> &'static str {
        match self {
            OnboardingText::OnlineTitle => "Birlikte Oyna",
            OnboardingText::OnlineSubtitle => {
                "Arkadaşlarını görmek, partilerine ve aramalarına katılmak ve kupalarını profilinde tutmak için giriş yap. Giriş için tarayıcın kısa bir süre açılır."
            }
            OnboardingText::NotNow => "Şimdi Değil",
            OnboardingText::OnlineLater => "İstediğin zaman üst çubuktan giriş yapabilirsin.",
            OnboardingText::StepOf => "Adım %@ / %@",
            OnboardingText::LookTitle => "Görünümünü Seç",
            OnboardingText::LookSubtitle => {
                "Arka plan seçimini gösterir. Buradaki her şeyi sonra Ayarlar'dan değiştirebilirsin."
            }
            OnboardingText::Accessibility => "Erişilebilirlik",
            OnboardingText::ReduceMotion => "Hareketi Azalt",
            OnboardingText::PadTitle => "Kolun",
            OnboardingText::PadConnected => {
                "%@ bağlı. Sağdaki dört tuştan birine bas, aşağıda yanar."
            }
            OnboardingText::PadLost => {
                "Kol şu anda bağlı değil. Düğmeleri denemek için bağla. Aşağıdaki seçimler yine de geçerli."
            }
            OnboardingText::PadGoesBack => "%@ geri gider",
            OnboardingText::NoFolderTitle => "Henüz klasör seçilmedi",
            OnboardingText::NoFolderDetail => "Bir klasörü sonra Ayarlar'dan seçebilirsin.",
            OnboardingText::FactPrivateTitle => "Varsayılan Olarak Gizli",
            OnboardingText::FactPrivateDetail => {
                "Profilin, kupaların ve oynama sürelerin yalnızca arkadaşlarına görünür."
            }
            OnboardingText::FactOfflineDetail => {
                "Arkadaşların çevrimiçi olduğunu görür. Çevrimdışı Görün bunu herkesten gizler, istediğin zaman."
            }
            OnboardingText::FactLocalTitle => "Oyun Dosyaların Bu PC'de Kalır",
            OnboardingText::FactLocalDetail => "Oyun klasöründen hiçbir şey yüklenmez.",
            OnboardingText::DoneTitle => "Hazırsın",
            OnboardingText::DoneSubtitle => "Başlamadan önce bilmeye değer birkaç şey.",
            OnboardingText::DoneHomeTitle => "Ana Düğme",
            OnboardingText::DoneHomeDetail => {
                "Oyun açıkken başlatıcıyı öne getirmek için koldan ona bas."
            }
            OnboardingText::DonePadTitle => "Kollar",
            OnboardingText::DonePadDetail => {
                "İstediğin zaman bir tane tak. Ayarlar'da düğme düzeni ve canlı bir test var."
            }
            OnboardingText::DoneSettingsTitle => "Ayarlar",
            OnboardingText::DoneSettingsDetail => {
                "Üst çubuktaki dişli. Burada seçtiğin her şey orada."
            }
            OnboardingText::DoneGamesTitle => "Oyunların",
            OnboardingText::DoneGamesDetail => "Klasöründeki yeni oyunlar kendiliğinden görünür.",
            OnboardingText::DoneNoFolderTitle => "Oyunlarını Ekle",
            OnboardingText::DoneNoFolderDetail => {
                "Ayarlar'da Kitaplık altından bir klasör seç. Ana ekran nasıl yapılacağını gösterir."
            }
            OnboardingText::DoneNextSignIn => "Sonra: Giriş Yap",
            OnboardingText::DoneNextCreate => "Sonra: Hesap Oluştur",
            OnboardingText::DoneSignInDetail => "Başla'ya bastığında tarayıcın açılır.",
        }
    }

    /// The German text; `None` falls back to English.
    #[allow(unreachable_patterns)]
    pub const fn german(self) -> Option<&'static str> {
        Some(match self {
            OnboardingText::OnlineTitle => "Gemeinsam spielen",
            OnboardingText::OnlineSubtitle => {
                "Melde dich an, um deine Freunde zu sehen, ihren Partys und Anrufen beizutreten und deine Trophäen im Profil zu behalten. Die Anmeldung öffnet kurz deinen Browser."
            }
            OnboardingText::NotNow => "Nicht jetzt",
            OnboardingText::OnlineLater => {
                "Du kannst dich jederzeit in der oberen Leiste anmelden."
            }
            OnboardingText::StepOf => "Schritt %@ von %@",
            OnboardingText::LookTitle => "Wähle dein Aussehen",
            OnboardingText::LookSubtitle => {
                "Der Hintergrund zeigt deine Wahl. Alles hier kannst du später in den Einstellungen ändern."
            }
            OnboardingText::Accessibility => "Barrierefreiheit",
            OnboardingText::ReduceMotion => "Bewegung reduzieren",
            OnboardingText::PadTitle => "Dein Controller",
            OnboardingText::PadConnected => {
                "%@ ist verbunden. Drück eine der vier Tasten rechts, dann leuchtet sie unten auf."
            }
            OnboardingText::PadLost => {
                "Der Controller ist gerade nicht verbunden. Verbinde ihn, um die Tasten zu testen. Die Auswahl unten gilt trotzdem."
            }
            OnboardingText::PadGoesBack => "%@ geht zurück",
            OnboardingText::NoFolderTitle => "Noch kein Ordner gewählt",
            OnboardingText::NoFolderDetail => {
                "Einen Ordner kannst du später in den Einstellungen wählen."
            }
            OnboardingText::FactPrivateTitle => "Standardmäßig privat",
            OnboardingText::FactPrivateDetail => {
                "Dein Profil, deine Trophäen und deine Spielzeit sehen nur Freunde."
            }
            OnboardingText::FactOfflineDetail => {
                "Freunde sehen, wann du online bist. Mit Offline erscheinen verbirgst du das vor allen, jederzeit."
            }
            OnboardingText::FactLocalTitle => "Deine Spieldateien bleiben auf diesem PC",
            OnboardingText::FactLocalDetail => "Aus deinem Spieleordner wird nichts hochgeladen.",
            OnboardingText::DoneTitle => "Alles bereit",
            OnboardingText::DoneSubtitle => "Ein paar Dinge, die du vor dem Start wissen solltest.",
            OnboardingText::DoneHomeTitle => "Home-Taste",
            OnboardingText::DoneHomeDetail => {
                "Drück sie auf dem Controller, um den Launcher nach vorn zu holen, während ein Spiel läuft."
            }
            OnboardingText::DonePadTitle => "Controller",
            OnboardingText::DonePadDetail => {
                "Schließ jederzeit einen an. Die Einstellungen haben das Tastenlayout und einen Live-Test."
            }
            OnboardingText::DoneSettingsTitle => "Einstellungen",
            OnboardingText::DoneSettingsDetail => {
                "Das Zahnrad in der oberen Leiste. Alles, was du hier gewählt hast, steht dort."
            }
            OnboardingText::DoneGamesTitle => "Deine Spiele",
            OnboardingText::DoneGamesDetail => {
                "Neue Spiele in deinem Ordner erscheinen von selbst."
            }
            OnboardingText::DoneNoFolderTitle => "Spiele hinzufügen",
            OnboardingText::DoneNoFolderDetail => {
                "Wähle in den Einstellungen unter Bibliothek einen Ordner. Der Startbildschirm zeigt, wie."
            }
            OnboardingText::DoneNextSignIn => "Als Nächstes: Anmelden",
            OnboardingText::DoneNextCreate => "Als Nächstes: Konto erstellen",
            OnboardingText::DoneSignInDetail => {
                "Dein Browser öffnet sich, wenn du auf Starten drückst."
            }
            _ => return None,
        })
    }

    pub const fn text(self, language: Language) -> &'static str {
        match language {
            Language::Turkish => self.turkish(),
            Language::English => self.english(),
            Language::German => match self.german() {
                Some(text) => text,
                None => self.english(),
            },
        }
    }

    pub fn format(self, language: Language, args: &[Arg<'_>]) -> String {
        substitute(self.text(language), args)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_text_is_there_in_every_language() {
        for text in OnboardingText::ALL {
            for language in Language::ALL {
                assert!(
                    !text.text(language).trim().is_empty(),
                    "{text:?} {language:?}"
                );
            }
            assert!(text.german().is_some(), "{text:?} has no German");
        }
    }

    #[test]
    fn no_text_has_an_exclamation_mark_or_a_please() {
        for text in OnboardingText::ALL {
            for language in Language::ALL {
                let s = text.text(language);
                assert!(!s.contains('!'), "{text:?} {language:?}");
                assert!(!s.to_lowercase().contains("please"), "{text:?}");
            }
        }
    }
}
