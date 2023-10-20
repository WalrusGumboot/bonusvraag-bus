// Hoeveel discrete tijdsstappen er als één seconde gelden.
const RESOLUTIE: u128 = 10_000_000;

fn main() {
    let iteraties_tussen_vliegen = 1u128;

    let mut bus_snelheid: f64 = 138.8888889;  // ms¯¹
    let mut bus_massa: f64 = 1000.0;          // kg
    let mut bus_afgelegde_afstand: f64 = 0.0; // m

    let vlieg_snelheid: f64 = 5.0; // ms¯¹
    let vlieg_massa: f64 = 0.001;  // kg

    let mut kul_kosten: u128 = 0; // eurocent (¢)

    let persoon_effect: f64 = std::f64::consts::PI + 5876.0; // J

    let mut iteraties = 0u128;
    let mut gelanceerde_vliegen = 0u128;

    loop {
        if iteraties % RESOLUTIE * 10 == 0 {
            let precisie = 4usize;
            println!("---");
            println!("Verlopen tijd:       {:.precisie$} s", iteraties / RESOLUTIE);
            println!("Afgelegde afstand:   {:.precisie$} m", bus_afgelegde_afstand);
            println!("Snelheid:            {:.precisie$} m/s", bus_snelheid);
            println!("Massa:               {:.precisie$} kg", bus_massa);
            println!("Kosten KU Leuven:    € {:.2}", kul_kosten / 100);
            println!("Gelanceerde vliegen: {}", gelanceerde_vliegen);
        }

        if iteraties % RESOLUTIE == 0 {
            // een persoon wordt in de bus gesmeten
            bus_snelheid = ((bus_massa * bus_snelheid * bus_snelheid + persoon_effect) / (bus_massa + 100.0)).sqrt();
            bus_massa += 100.0;
        }

        if iteraties % iteraties_tussen_vliegen == 0 {
            // een vlieg raakt de bus
            kul_kosten += 53;

            let oude_massa = bus_massa.clone();

            bus_massa += vlieg_massa;
            bus_snelheid = (vlieg_massa * (vlieg_snelheid - bus_snelheid) + vlieg_massa * vlieg_snelheid + oude_massa * bus_snelheid) / (vlieg_massa + oude_massa);

            gelanceerde_vliegen += 1;
        }

        let delta_afstand = bus_snelheid / RESOLUTIE as f64;
        bus_afgelegde_afstand += delta_afstand;
        bus_massa -= -6.04490500863557858376511e-10 * delta_afstand;

        if bus_afgelegde_afstand >= 10_000_000.0 {
            break;
        }

        iteraties += 1;

        // std::thread::sleep(std::time::Duration::from_millis(5));
        // println!("{iteraties}")
    }

    println!("oeps! Luc Sels is dood.");
}
