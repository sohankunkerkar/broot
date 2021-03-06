use {
    super::*,
    crate::{
        conf::Conf,
    },
    fnv::FnvHashMap,
};


/// all the skin things used by the broot application
/// during runing
pub struct AppSkin {

    /// the skin used in the focused panel
    pub focused: PanelSkin,

    /// the skin used in unfocused panels
    pub unfocused: PanelSkin,
}

impl AppSkin {

    pub fn new(conf: &Conf) -> Self {
        let def_skin;
        let skin = if let Some(skin) = &conf.skin {
            skin
        } else {
            def_skin = FnvHashMap::default();
            &def_skin
        };
        let StyleMaps { focused, unfocused } = StyleMaps::create(skin);
        Self {
            focused: PanelSkin::new(focused),
            unfocused: PanelSkin::new(unfocused),
        }
    }

}
