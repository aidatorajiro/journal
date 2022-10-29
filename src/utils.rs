pub mod basic {
    use std::{time::{SystemTime, UNIX_EPOCH}, borrow::Cow};

    use egui::{Ui, FontTweak, FontFamily, FontData};

    /// Returns current timestamp in second.
    pub fn create_timestamp() -> u64 {
        return SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Set font.
    pub fn set_default_font(ui: &mut Ui) {
        let mut fonts = egui::text::FontDefinitions::default();

        fonts.font_data.insert("notosans".to_string(), FontData {
            font: Cow::from(include_bytes!("../assets/NotoSansJP-Thin.otf").to_vec()),
            index: 0,
            tweak: FontTweak {
                scale: 2.0,
                y_offset_factor: 0.0,
                y_offset: -6.0,
            },
        });
        
        fonts.font_data.insert("notoemoji".to_string(), FontData {
            font: Cow::from(include_bytes!("../assets/NotoEmoji-VariableFont_wght.ttf").to_vec()),
            index: 0,
            tweak: FontTweak {
                scale: 2.0,
                y_offset_factor: 0.0,
                y_offset: -6.0,
            },
        });

        fonts.families.insert(FontFamily::Proportional, vec!["notosans".to_string(), "notoemoji".to_string()]);
        ui.ctx().set_fonts(fonts.clone());
        
    }
}

pub mod graph {
    use std::{hash::Hash, fmt::Debug};

    use bevy::{utils::HashMap};
    use fdg_sim::{ForceGraph, ForceGraphHelper};
    use petgraph::{Graph, graph::NodeIndex, graph::EdgeIndex};

    /// create force graph nodes from existing graph
    pub fn make_force_graph_nodes<A: Clone + Debug + Eq + Hash, B> (force_graph: &mut ForceGraph<A, f32>, base_graph: &Graph<A, B>, map_ent_idx: &mut HashMap<A, NodeIndex>) {
        for e in base_graph.node_weights() {
            if map_ent_idx.get(e).is_none() {
                let idx = force_graph.add_force_node(format!("{:?}", e.clone()), e.clone());
                map_ent_idx.insert(e.clone(), idx);
            }
        }
    }

    /// create force graph edges from existing graph
    pub fn make_force_graph_edges <A: Eq + Hash, B, T:Into<f32>, F> (force_graph: &mut ForceGraph<A, f32>, base_graph: &Graph<A, B>, map_ent_idx: &HashMap<A, NodeIndex>, func: F)
    where F: Fn(&A, &A, &B, EdgeIndex) -> T {
        for i in base_graph.edge_indices() {
            let (a_idx, b_idx) = base_graph.edge_endpoints(i).unwrap();
            let a_wgt = base_graph.node_weight(a_idx).unwrap();
            let b_wgt = base_graph.node_weight(b_idx).unwrap();
            let self_wgt = base_graph.edge_weight(i).unwrap();
            force_graph.add_edge(map_ent_idx.get(a_wgt).unwrap().clone(), map_ent_idx.get(b_wgt).unwrap().clone(), func(
                a_wgt, b_wgt, self_wgt, i
            ).into());
        }
    }
}