use plotters::prelude::*;
fn vecteur_vers_graphique(vecteur: Vec!((f32, f32))) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("plotters-doc-data/0.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE)?;
    let mut chart = ChartBuilder::on(&root)
        .caption("Taux de défonçage non-consentant du robot", ("comic-sans-MS", 25).into_font())    //titre
        .margin(5)
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0f32..totalparties as f32, 0f32..1f32)?;                                       //nombre max x et y

    chart.configure_mesh().draw()?;
    chart
        .draw_series(LineSeries::new(
            vecteur,                                                                                  //formule???  x , nbvictoire_ia/x
            &RED,
        ))?
        .label("y = x^2")                                                                           //formule affiché à droite
        .legend(|(x , y)| PathElement::new(vec![(x, y), (x+20 , y)], &RED));                        // formule pour ligne dans la légende


    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.8))
        .border_style(&BLACK)
        .draw()?;

    Ok(())
}