/*
 * Copyright (c) Joseph Prichard 2022.
 */

#[derive(Copy, Clone, Debug)]
pub struct Run {
    max_depth: u32,
    hits: u32,
    misses: u32,
    time_taken: u128,
}

impl Run {
    pub fn new(max_depth: u32, hits: u32, misses: u32, time_taken: u128) -> Self {
        Self { max_depth, hits, misses, time_taken }
    }
}

pub struct Profiler {
    runs: Vec<Run>,
}

impl Profiler {
    pub fn new() -> Profiler {
        Profiler {
            runs: vec![]
        }
    }

    pub fn add_run(&mut self, run: Run) {
        self.runs.push(run);
    }

    pub fn log_runs(&self) {
        let mut total_time = 0;
        let len = self.runs.len();
        for run in self.runs.iter() {
            let debug_output = format!(
                "Finished analysis, max_depth: {}, hits: {}, misses: {}, time_taken: {} ms",
                run.max_depth, run.hits, run.misses, run.time_taken
            );
            eprintln!("{}", debug_output);
            total_time += run.time_taken;
        }
        let avg_time = if len > 0 { total_time / len as u128 } else { 0 };
        eprintln!("Total time: {} ms", total_time);
        eprintln!("Average time {} ms", avg_time)
    }
}