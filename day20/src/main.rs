use std::collections::{HashMap, VecDeque};

fn main() {
    let data = include_str!("../input.txt");
    println!("{}", part1(data));
    println!("{:?}", part2(data));
}

fn part1(data: &str) -> i64 {
    let mut modules: HashMap<String, Module> = HashMap::default();
    data.lines().map(Module::new).for_each(|module| {
        modules.insert(module.name.clone(), module);
    });

    //add sources to conj
    let mut conj_to_update: Vec<(String, String)> = vec![];
    for module in modules.values() {
        for destination in module.destinations.iter() {
            println!("{:?}", destination);
            if let Some(m) = modules.get(destination) {
                match m.module {
                    ModuleKind::Conj(_) => {
                        conj_to_update.push((m.name.clone(), module.name.clone()))
                    }
                    _ => continue,
                }
            }
        }
    }

    for (m_name, source) in conj_to_update {
        let m = modules.get_mut(&m_name);
        if let Some(m) = m {
            match m.module {
                ModuleKind::Conj(ref mut conj) => {
                    let pulse = Pulse {
                        source,
                        state: PulseState::Low,
                    };
                    conj.update(&pulse)
                }
                _ => continue,
            };
        }
    }

    // for module in modules.values() {
    //     println!("{:?}", module);
    // }

    let mut pulse_queue: VecDeque<(String, Pulse)> = VecDeque::default();
    let mut pulse_count: (i64, i64) = (0, 0);
    let button_presses = 1000;

    for _ in 0..button_presses {
        //add one for the button press
        pulse_count.0 += 1;
        pulse_queue.extend(
            modules
                .get_mut("broadcaster")
                .unwrap()
                .pulse(&Pulse::default()),
        );
        println!("{:?}", pulse_queue);
        while let Some((module_name, pulse)) = pulse_queue.pop_front() {
            match pulse.state {
                PulseState::High => pulse_count.1 += 1,
                PulseState::Low => pulse_count.0 += 1,
            }

            if let Some(module) = modules.get_mut(&module_name){
                pulse_queue.extend(module.pulse(&pulse))
            };
        }
    }
    println!("{:?}", pulse_count);
    pulse_count.0 * pulse_count.1
}

fn part2(_data: &str) {
    todo!()
}

impl Module {
    fn new(line: &str) -> Self {
        let data: Vec<&str> = line.split(',').flat_map(|s| s.split_whitespace()).collect();
        let mut kind: char = '_';
        let mut destinations = vec![];
        let name = if data[0].starts_with('%') || data[0].starts_with('&') {
            kind = data[0].chars().next().unwrap();
            data[0].chars().skip(1).collect()
        } else {
            data[0].to_string()
        };

        if let Some(index) = data.iter().position(|&elem| elem == "->") {
            for elem in data.iter().skip(index + 1) {
                destinations.push(elem.to_string())
            }
        }

        Module {
            name,
            module: match kind {
                '%' => ModuleKind::FlipFlop(FlipFlop::new()),
                '&' => ModuleKind::Conj(Conj::new()),
                _ => ModuleKind::Broadcast,
            },
            destinations,
        }
    }

    fn pulse(&mut self, pulse: &Pulse) -> Vec<(String, Pulse)> {
        let mut pulses: Vec<(String, Pulse)> = vec![];
        let pulse_state = match self.module {
            ModuleKind::FlipFlop(ref mut m) => m.pulse(pulse),
            ModuleKind::Conj(ref mut m) => Some(m.pulse(pulse)),
            ModuleKind::Broadcast => Some(PulseState::Low),
        };

        if let Some(state) = pulse_state {
            let pulse = Pulse {
                state,
                source: self.name.clone(),
            };
            for destination in self.destinations.iter() {
                println!("pulse {:?}, to {:?}", pulse, destination);
                pulses.push((destination.to_string(), pulse.clone()))
            }
        }
        pulses
    }
}

#[derive(Debug, PartialEq, Eq)]
enum ModuleKind {
    FlipFlop(FlipFlop),
    Conj(Conj),
    Broadcast,
}

#[derive(Debug)]
struct Module {
    name: String,
    module: ModuleKind,
    destinations: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
struct FlipFlop {
    state: FlipFlopState,
}

impl FlipFlop {
    fn new() -> Self {
        FlipFlop {
            state: FlipFlopState::Off,
        }
    }

    fn pulse(&mut self, pulse: &Pulse) -> Option<PulseState> {
        match pulse.state {
            PulseState::Low => match self.state {
                FlipFlopState::Off => {
                    self.state = FlipFlopState::On;
                    Some(PulseState::High)
                }
                FlipFlopState::On => {
                    self.state = FlipFlopState::Off;
                    Some(PulseState::Low)
                }
            },
            PulseState::High => None,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Conj {
    memory: HashMap<String, Pulse>,
}

impl Conj {
    fn new() -> Self {
        Conj {
            memory: HashMap::default(),
        }
    }

    fn update(&mut self, pulse: &Pulse) {
        // if let Some(p) = self.memory.get_mut(&pulse.source) {
        //     p.state = pulse.state
        // }
        self.memory
            .entry(pulse.source.clone())
            .and_modify(|p| p.state = pulse.state)
            .or_insert(Pulse {
                source: pulse.source.clone(),
                state: PulseState::Low,
            });
    }

    fn check(&self) -> PulseState {
        if self
            .memory
            .values()
            .all(|value| value.state == PulseState::High)
        {
            PulseState::Low
        } else {
            PulseState::High
        }
    }

    fn pulse(&mut self, pulse: &Pulse) -> PulseState {
        self.update(pulse);
        self.check()
    }
}

#[derive(PartialEq, Eq, Debug)]
enum FlipFlopState {
    On,
    Off,
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Pulse {
    source: String,
    state: PulseState,
}

impl Pulse {
    fn default() -> Self {
        Pulse {
            state: PulseState::Low,
            source: "start".to_string(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum PulseState {
    Low,
    High,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn part1_test() {
        let data = include_str!("./test.txt");
        assert_eq!(part1(data), 32000000)
    }

    #[test]
    fn part1_test2() {
        let data = include_str!("./test2.txt");
        assert_eq!(part1(data), 11687500)
    }

    #[test]
    fn new_module() {
        let data = "%vn -> ts, lq";
        let m = Module::new(data);
        assert_eq!(m.name, "vn".to_string());
        assert_eq!(m.destinations, vec!["ts".to_string(), "lq".to_string()]);

        let data = "broadcaster -> qz, tx, jr, hk";
        let module = Module::new(data);
        assert_eq!(module.name, "broadcaster".to_string());
        assert_eq!(module.module, ModuleKind::Broadcast);
        assert_eq!(
            module.destinations,
            vec![
                "qz".to_string(),
                "tx".to_string(),
                "jr".to_string(),
                "hk".to_string()
            ]
        )
    }
}
