// SPDX-FileCopyrightText: Johan Lager
// SPDX-License-Identifier: GPL-3.0-only

use super::*;

mod component_needs {
    use super::*;

    #[test]
    fn same_component_twice_is_a_duplicate() {
        let result = ComponentNeeds::new(vec![
            need(Component::DxvkSource1ReShadeProxy),
            need(Component::ReFramework),
            need(Component::DxvkSource1ReShadeProxy),
        ]);

        assert_eq!(
            result,
            Err(ComponentNeedsProblem::DuplicateComponent(
                Component::DxvkSource1ReShadeProxy
            ))
        );
    }

    #[test]
    fn different_components_are_kept_in_entered_order() {
        let needs = ComponentNeeds::new(vec![
            need(Component::DxvkSource1ReShadeProxy),
            need(Component::ReFramework),
        ])
        .unwrap();
        let components: Vec<Component> = needs.iter().map(|need| need.component).collect();

        assert_eq!(
            components,
            vec![Component::DxvkSource1ReShadeProxy, Component::ReFramework]
        );
    }

    fn need(component: Component) -> ComponentNeed {
        ComponentNeed {
            component,
            notes: None,
        }
    }
}
