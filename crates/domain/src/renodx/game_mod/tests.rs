// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;
use crate::component::ComponentNeed;

mod game_mod {
    use super::*;

    #[test]
    fn nexus_only_is_valid() {
        let links = ModLinks {
            nexus_mods: Some(
                NexusModsUrl::parse("https://www.nexusmods.com/avowed/mods/101").unwrap(),
            ),
            ..links_without_any()
        };
        let result = GameMod::new(parts_with(links));
        assert!(result.is_ok(), "{result:?}");
    }

    #[test]
    fn discussion_only_is_invalid() {
        let links = ModLinks {
            discussion: Some(
                DiscussionUrl::parse("https://github.com/clshortfuse/renodx/discussions/490")
                    .unwrap(),
            ),
            ..links_without_any()
        };

        assert_eq!(
            GameMod::new(parts_with(links)),
            Err(GameModProblem::NoSourceLink)
        );
    }

    #[test]
    fn no_links_is_invalid() {
        assert_eq!(
            GameMod::new(parts_with(links_without_any())),
            Err(GameModProblem::NoSourceLink)
        );
    }

    #[test]
    fn component_in_depends_and_recommends_is_invalid() {
        let parts = GameModParts {
            depends: needs(&[Component::ReFramework, Component::DxvkSource1ReShadeProxy]),
            recommends: needs(&[Component::DxvkSource1ReShadeProxy]),
            ..parts_with(nexus_only_links())
        };

        assert_eq!(
            GameMod::new(parts),
            Err(GameModProblem::ComponentInBothLists(
                Component::DxvkSource1ReShadeProxy
            ))
        );
    }

    #[test]
    fn into_parts_returns_the_parts_it_was_built_from() {
        let links = ModLinks {
            discord: Some(DiscordUrl::parse("https://discord.com/invite/renodx").unwrap()),
            ..links_without_any()
        };
        let parts = parts_with(links);
        let game_mod = GameMod::new(parts.clone()).unwrap();

        assert_eq!(game_mod.into_parts(), parts);
    }

    fn links_without_any() -> ModLinks {
        ModLinks {
            snapshot_64: None,
            snapshot_32: None,
            nexus_mods: None,
            game_banana: None,
            discord: None,
            discussion: None,
        }
    }

    fn parts_with(links: ModLinks) -> GameModParts {
        GameModParts {
            id: ModId::parse("control").unwrap(),
            name: GameName::parse("Control").unwrap(),
            maintainer: Maintainer::parse("John (Rust)").unwrap(),
            status: ModStatus::Working,
            notes: None,
            links,
            depends: ComponentNeeds::default(),
            recommends: ComponentNeeds::default(),
        }
    }

    fn nexus_only_links() -> ModLinks {
        ModLinks {
            nexus_mods: Some(
                NexusModsUrl::parse("https://www.nexusmods.com/avowed/mods/101").unwrap(),
            ),
            ..links_without_any()
        }
    }

    fn needs(components: &[Component]) -> ComponentNeeds {
        let mut needs = Vec::new();

        for component in components {
            needs.push(ComponentNeed {
                component: *component,
                notes: None,
            });
        }
        ComponentNeeds::new(needs).unwrap()
    }
}
