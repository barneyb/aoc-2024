use aoc::aocd::get_input;
use aoc::timing::Timing;
use aoc::y2024::*;
use std::io::{stdout, Write};
use std::sync::mpsc::channel;
use std::time::Duration;

fn main() {
    let raw_inputs: Vec<_> = {
        let mut lock = stdout().lock();
        write!(lock, "Reading inputs").unwrap();
        (1..=25)
            .map(|d| {
                write!(lock, "{d:.>4}").unwrap();
                lock.flush().unwrap();
                get_input(2024, d).unwrap()
            })
            .collect()
    };
    let inputs: Vec<_> = raw_inputs
        .iter()
        .map(|i| i.trim_end_matches('\n'))
        .collect();
    println!("...done!");
    let (tx, _rx) = channel(); // just a dumb sink, but need to bind the receiver
    let results = vec![
        (
            "historian_hysteria",
            Timing::duration(|| historian_hysteria_01::do_solve(inputs[1 - 1], tx.clone())),
        ),
        (
            "red_nosed_reports",
            Timing::duration(|| red_nosed_reports_02::do_solve(inputs[2 - 1], tx.clone())),
        ),
        (
            "mull_it_over",
            Timing::duration(|| mull_it_over_03::do_solve(inputs[3 - 1], tx.clone())),
        ),
        (
            "ceres_search",
            Timing::duration(|| ceres_search_04::do_solve(inputs[4 - 1], tx.clone())),
        ),
        (
            "print_queue",
            Timing::duration(|| print_queue_05::do_solve(inputs[5 - 1], tx.clone())),
        ),
        (
            "guard_gallivant",
            Timing::duration(|| guard_gallivant_06::do_solve(inputs[6 - 1], tx.clone())),
        ),
        (
            "bridge_repair",
            Timing::duration(|| bridge_repair_07::do_solve(inputs[7 - 1], tx.clone())),
        ),
        (
            "resonant_collinearity",
            Timing::duration(|| resonant_collinearity_08::do_solve(inputs[8 - 1], tx.clone())),
        ),
        (
            "disk_fragmenter",
            Timing::duration(|| disk_fragmenter_09::do_solve(inputs[9 - 1], tx.clone())),
        ),
        (
            "hoof_it",
            Timing::duration(|| hoof_it_10::do_solve(inputs[10 - 1], tx.clone())),
        ),
        (
            "plutonian_pebbles",
            Timing::duration(|| plutonian_pebbles_11::do_solve(inputs[11 - 1], tx.clone())),
        ),
        (
            "garden_groups",
            Timing::duration(|| garden_groups_12b::do_solve(inputs[12 - 1], tx.clone())),
        ),
        (
            "claw_contraption",
            Timing::duration(|| claw_contraption_13::do_solve(inputs[13 - 1], tx.clone())),
        ),
        (
            "restroom_redoubt",
            Timing::duration(|| restroom_redoubt_14::do_solve(inputs[14 - 1], tx.clone())),
        ),
        (
            "warehouse_woes",
            Timing::duration(|| warehouse_woes_15::do_solve(inputs[15 - 1], tx.clone())),
        ),
        (
            "reindeer_maze",
            Timing::duration(|| reindeer_maze_16::do_solve(inputs[16 - 1], tx.clone())),
        ),
        (
            "chronospatial_computer",
            Timing::duration(|| chronospatial_computer_17::do_solve(inputs[17 - 1], tx.clone())),
        ),
        (
            "ram_run",
            Timing::duration(|| ram_run_18::do_solve(inputs[18 - 1], tx.clone())),
        ),
        (
            "linen_layout",
            Timing::duration(|| linen_layout_19::do_solve(inputs[19 - 1], tx.clone())),
        ),
        (
            "race_condition",
            Timing::duration(|| race_condition_20::do_solve(inputs[20 - 1], tx.clone())),
        ),
        (
            "keypad_conundrum",
            Timing::duration(|| keypad_conundrum_21::do_solve(inputs[21 - 1], tx.clone())),
        ),
        (
            "monkey_market",
            Timing::duration(|| monkey_market_22::do_solve(inputs[22 - 1], tx.clone())),
        ),
        (
            "lan_party",
            Timing::duration(|| lan_party_23::do_solve(inputs[23 - 1], tx.clone())),
        ),
        (
            "crossed_wires",
            Timing::duration(|| crossed_wires_24::do_solve(inputs[24 - 1], tx.clone())),
        ),
        (
            "code_chronicle",
            Timing::duration(|| code_chronicle_25::do_solve(inputs[25 - 1], tx.clone())),
        ),
    ];
    let total = results.iter().map(|(_, d)| d).sum::<Duration>();
    let max = results.iter().map(|(_, d)| d).max().unwrap().as_nanos();
    let len = results.iter().map(|(l, _)| l.len()).max().unwrap();
    let width = 100 - len;
    for (n, (l, d)) in results.iter().enumerate() {
        println!(
            "{:>3} {l:1$} | {d:>12?} | {2:3$} | {4:>4}%",
            n + 1,
            len,
            "#".repeat((d.as_nanos() * width as u128 / max) as usize),
            width,
            (d.as_nanos() * 1000 / total.as_nanos()) as f32 / 10.
        );
    }
    println!("    {:>1$} | {total:>12?}", "total", len);
}
