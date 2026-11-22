// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

#[cfg(test)]
mod tests;

use crate::graphics_api::GraphicsApi;
use crate::text::Notes;
use std::slice;

/// Something a mod needs installed next to it to work, named by the recipe that installs it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Component {
    ReFramework,
    /// The DXVK `ReShade` proxy, set up as the `RenoDX` wiki's Source 1 steps describe.
    DxvkSource1ReShadeProxy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentNeed {
    pub component: Component,
    pub notes: Option<Notes>,
}

/// Component needs in the order they were entered, each component at most once.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct ComponentNeeds {
    needs: Vec<ComponentNeed>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComponentNeedsProblem {
    DuplicateComponent(Component),
}

impl Component {
    /// The graphics API the game needs to be set to for this component, if it needs one.
    pub fn graphics_api(self) -> Option<GraphicsApi> {
        match self {
            Component::ReFramework => None,
            Component::DxvkSource1ReShadeProxy => Some(GraphicsApi::D3d9),
        }
    }
}

impl ComponentNeeds {
    pub fn new(needs: Vec<ComponentNeed>) -> Result<ComponentNeeds, ComponentNeedsProblem> {
        let mut seen = Vec::new();

        for need in &needs {
            if seen.contains(&need.component) {
                return Err(ComponentNeedsProblem::DuplicateComponent(need.component));
            }
            seen.push(need.component);
        }
        Ok(ComponentNeeds { needs })
    }

    pub fn iter(&self) -> slice::Iter<'_, ComponentNeed> {
        self.needs.iter()
    }

    /// The first shared component, in entered order, that `other` lists too.
    pub fn first_shared_component(&self, other: &ComponentNeeds) -> Option<Component> {
        for need in &self.needs {
            if other
                .needs
                .iter()
                .any(|other_need| other_need.component == need.component)
            {
                return Some(need.component);
            }
        }
        None
    }
}

impl<'a> IntoIterator for &'a ComponentNeeds {
    type Item = &'a ComponentNeed;
    type IntoIter = slice::Iter<'a, ComponentNeed>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
