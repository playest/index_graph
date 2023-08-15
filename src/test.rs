#[test]
fn test_graph() {
    use crate::{Graph, GraphView};
    let items = vec!("Ro", "Am", "Li", "Je", "Na", "Ar", "Cl", "La", "Ju", "Ol");
    let i_je = items.iter().position(|&e| e == "Je").unwrap();
    let i_cl = items.iter().position(|&e| e == "Cl").unwrap();
    let i_na = items.iter().position(|&e| e == "Na").unwrap();
    let i_ar = items.iter().position(|&e| e == "Ar").unwrap();
    let i_la = items.iter().position(|&e| e == "La").unwrap();
    let i_ju = items.iter().position(|&e| e == "Ju").unwrap();
    let i_ol = items.iter().position(|&e| e == "Ol").unwrap();
    let i_ro = items.iter().position(|&e| e == "Ro").unwrap();
    let i_am = items.iter().position(|&e| e == "Am").unwrap();
    let i_li = items.iter().position(|&e| e == "Li").unwrap();
    
    let mut graph = Graph::new();
    graph.add(None, i_je).unwrap();
    graph.add(None, i_cl).unwrap();
    graph.add(Some(i_je), i_na).unwrap();
    graph.add(Some(i_je), i_ar).unwrap();
    graph.add(Some(i_je), i_la).unwrap();
    graph.add(Some(i_na), i_ju).unwrap();
    graph.add(Some(i_na), i_ol).unwrap();
    graph.add(Some(i_na), i_ro).unwrap();
    graph.add(Some(i_ar), i_am).unwrap();
    graph.add(Some(i_ar), i_li).unwrap();
    graph.add(Some(i_cl), i_na).unwrap();
    graph.add(Some(i_cl), i_ar).unwrap();
    graph.add(Some(i_cl), i_la).unwrap();

    let view = GraphView::new(graph, &items);
    //view.print();

    let v_i_je = view.get(i_je).unwrap();
    let v_i_je_parents = v_i_je.parents().map(|c| *c.value.translate()).collect::<Vec<_>>();
    assert!(v_i_je_parents.len() == 0);
    let v_i_je_children = v_i_je.children().map(|c| *c.value.translate()).collect::<Vec<_>>();
    assert!(v_i_je_children.len() == 3);
    assert!(vec!("Ar", "Na", "La").iter().all(|e| v_i_je_children.contains(e)));
    
    //dbg!(v_i_je_parents);
    //dbg!(v_i_je.value.translate());
    //dbg!(v_i_je_children);

    let v_i_ar = view.get(i_ar).unwrap();

    let v_i_ar_parents = v_i_ar.parents().map(|c| *c.value.translate()).collect::<Vec<_>>();
    assert!(v_i_ar_parents.len() == 2);
    assert!(vec!("Je", "Cl").iter().all(|e| v_i_ar_parents.contains(e)));

    let v_i_ar_children = v_i_ar.children().map(|c| *c.value.translate()).collect::<Vec<_>>();
    assert!(v_i_ar_children.len() == 2);
    assert!(vec!("Am", "Li").iter().all(|e| v_i_ar_children.contains(e)));

    //dbg!(v_i_ar_parents);
    //dbg!(v_i_ar.value.translate());
    //dbg!(v_i_ar_children);
}